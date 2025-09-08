// Copyright (c) The StackClass Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{collections::HashMap, sync::Arc};

use jsonwebtoken::{
    DecodingKey,
    jwk::{AlgorithmParameters, Jwk},
};
use thiserror::Error;
use tokio::sync::{OnceCell, RwLock};
use tracing::{error, info};

use crate::{context::Context, repository::UserRepository};

/// Global cached JWK decoding keys (async initialization via OnceCell)
static KEYS: OnceCell<Arc<RwLock<HashMap<String, DecodingKey>>>> = OnceCell::const_new();

/// Represents errors that can occur during key operations.
#[derive(Debug, Error)]
pub enum KeysError {
    #[error("Invalid key format")]
    InvalidKeyFormat,

    #[error("Failed to load keys")]
    KeyLoadFailure,

    #[error("Key not found for ID: {0}")]
    KeyNotFound(String),

    #[error("Failed to refresh keys")]
    KeyRefreshFailed,
}

/// Loads JSON Web Keys (JWKs) from the database and converts them into `DecodingKey` instances.
/// Returns a `HashMap` mapping key IDs to their corresponding `DecodingKey`.
pub async fn load_keys(ctx: Arc<Context>) -> Result<HashMap<String, DecodingKey>, KeysError> {
    info!("Fetching all JSON Web Keys (JWKS) from the database");
    let keys = UserRepository::find_all_json_web_keys(&ctx.database).await.map_err(|e| {
        error!("Failed to load JSON web keys: {}", e);
        KeysError::KeyLoadFailure
    })?;

    let mut map = HashMap::new();
    for key in keys {
        let jwk: Jwk = serde_json::from_str(&key.public_key) //
            .map_err(|_| KeysError::InvalidKeyFormat)?;

        if let AlgorithmParameters::RSA(rsa) = jwk.algorithm {
            let decoded = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                .map_err(|_| KeysError::InvalidKeyFormat)?;
            map.insert(key.id, decoded);
        }
    }

    Ok(map)
}

/// Get global keys cache (initialize if empty)
pub async fn get_keys() -> &'static Arc<RwLock<HashMap<String, DecodingKey>>> {
    KEYS.get_or_init(|| async {
        Arc::new(RwLock::new(HashMap::new())) // Initial empty cache
    })
    .await
}

/// Refresh keys from database and update cache
pub async fn refresh_keys(ctx: Arc<Context>) -> Result<(), KeysError> {
    let keys = load_keys(ctx).await?;
    *get_keys().await.write().await = keys;

    Ok(())
}
