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

use std::{collections::HashMap, fmt::Display, sync::Arc};

use axum::{
    RequestPartsExt,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{
    Algorithm, DecodingKey, Validation, decode,
    jwk::{AlgorithmParameters, Jwk},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{OnceCell, RwLock};
use tracing::{debug, error};

use crate::{context::Context, errors::AutoIntoResponse, repository::UserRepository};

/// Global cached JWK decoding keys (async initialization via OnceCell)
static KEYS: OnceCell<Arc<RwLock<HashMap<String, DecodingKey>>>> = OnceCell::const_new();

/// Loads JSON Web Keys (JWKs) from the database and converts them into `DecodingKey` instances.
/// Returns a `HashMap` mapping key IDs to their corresponding `DecodingKey`.
async fn load_keys(ctx: Arc<Context>) -> Result<HashMap<String, DecodingKey>, ClaimsError> {
    let keys = UserRepository::find_all_json_web_keys(&ctx.database).await.map_err(|e| {
        error!("Failed to load JSON web keys: {}", e);
        ClaimsError::KeyLoadFailure
    })?;

    let mut map = HashMap::new();
    for key in keys {
        let jwk: Jwk = serde_json::from_str(&key.public_key) //
            .map_err(|_| ClaimsError::InvalidKeyFormat)?;

        if let AlgorithmParameters::RSA(rsa) = jwk.algorithm {
            let decoded = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                .map_err(|_| ClaimsError::InvalidKeyFormat)?;
            map.insert(key.id, decoded);
        }
    }

    Ok(map)
}

/// Get global keys cache (initialize if empty)
async fn get_keys() -> &'static Arc<RwLock<HashMap<String, DecodingKey>>> {
    KEYS.get_or_init(|| async {
        Arc::new(RwLock::new(HashMap::new())) // Initial empty cache
    })
    .await
}

/// Refresh keys from database and update cache
pub async fn refresh_keys(ctx: Arc<Context>) -> Result<(), ClaimsError> {
    let keys = load_keys(ctx).await?;
    *get_keys().await.write().await = keys;

    Ok(())
}

/// Represents the claims extracted from a JWT token.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Unique identifier of the user.
    pub id: String,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User Id: {}", self.id)
    }
}

/// Extracts `Claims` from an HTTP request by validating the JWT token.
impl FromRequestParts<Arc<Context>> for Claims {
    type Rejection = ClaimsError;

    async fn from_request_parts(
        parts: &mut Parts,
        ctx: &Arc<Context>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ClaimsError::TokenNotFound)?;

        let header = jsonwebtoken::decode_header(bearer.token())
            .map_err(|_| ClaimsError::TokenParseError)?;
        let kid = header.kid.ok_or(ClaimsError::MissingKeyId)?;

        // First attempt with cached keys
        let keys = get_keys().await;
        if let Some(decoding_key) = keys.read().await.get(&kid) {
            return validate_token(bearer.token(), decoding_key);
        }

        // If kid not found, refresh keys and try again
        refresh_keys(ctx.clone()).await.map_err(|_| ClaimsError::KeyRefreshFailed)?;

        if let Some(decoding_key) = keys.read().await.get(&kid) {
            return validate_token(bearer.token(), decoding_key);
        }

        Err(ClaimsError::KeyNotFound(kid))
    }
}

/// Validates a JWT token using the provided `DecodingKey`.
fn validate_token(token: &str, decoding_key: &DecodingKey) -> Result<Claims, ClaimsError> {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&["StackClass"]);
    validation.set_audience(&["StackClass"]);

    let token_data = decode::<Claims>(token, decoding_key, &validation).map_err(|e| {
        debug!("Failed to decode token: {}", e);
        ClaimsError::InvalidToken
    })?;

    Ok(token_data.claims)
}

/// Represents errors that can occur during JWT token validation.
#[derive(Debug, Error)]
pub enum ClaimsError {
    #[error("Token not found")]
    TokenNotFound,

    #[error("Failed to parse token")]
    TokenParseError,

    #[error("Token missing key ID")]
    MissingKeyId,

    #[error("Key not found for ID: {0}")]
    KeyNotFound(String),

    #[error("Invalid token")]
    InvalidToken,

    #[error("Invalid key format")]
    InvalidKeyFormat,

    #[error("Failed to load keys")]
    KeyLoadFailure,

    #[error("Failed to refresh keys")]
    KeyRefreshFailed,
}

impl From<&ClaimsError> for StatusCode {
    fn from(val: &ClaimsError) -> Self {
        match val {
            ClaimsError::TokenNotFound => StatusCode::UNAUTHORIZED,
            ClaimsError::TokenParseError => StatusCode::UNAUTHORIZED,
            ClaimsError::MissingKeyId => StatusCode::UNAUTHORIZED,
            ClaimsError::KeyNotFound(_) => StatusCode::UNAUTHORIZED,
            ClaimsError::InvalidToken => StatusCode::UNAUTHORIZED,
            ClaimsError::InvalidKeyFormat => StatusCode::UNAUTHORIZED,
            ClaimsError::KeyLoadFailure => StatusCode::INTERNAL_SERVER_ERROR,
            ClaimsError::KeyRefreshFailed => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ClaimsError {
    fn into_response(self) -> Response {
        AutoIntoResponse::into(&self)
    }
}
