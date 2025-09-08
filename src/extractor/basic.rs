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

use std::sync::Arc;

use axum::{
    RequestPartsExt,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Basic},
};
use thiserror::Error;
use tracing::debug;

use crate::{
    context::Context,
    errors::AutoIntoResponse,
    utils::crypto::{self, CryptoError},
};

/// Represents admin authentication via Basic Auth
#[derive(Debug)]
pub struct AdminBasic;

/// Errors that can occur during Basic Auth validation
#[derive(Debug, Error)]
pub enum BasicAuthError {
    #[error("Authorization header missing")]
    MissingAuthorizationHeader,

    #[error("Invalid username or password")]
    InvalidCredentials,

    #[error("Access forbidden")]
    Forbidden,

    #[error("Crypto operation failed")]
    CryptoError(#[from] CryptoError),
}

impl From<&BasicAuthError> for StatusCode {
    fn from(val: &BasicAuthError) -> Self {
        match val {
            BasicAuthError::MissingAuthorizationHeader => StatusCode::UNAUTHORIZED,
            BasicAuthError::InvalidCredentials => StatusCode::FORBIDDEN,
            BasicAuthError::Forbidden => StatusCode::FORBIDDEN,
            BasicAuthError::CryptoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for BasicAuthError {
    fn into_response(self) -> Response {
        AutoIntoResponse::into(&self)
    }
}

/// Extracts and validates admin Basic Auth credentials
impl FromRequestParts<Arc<Context>> for AdminBasic {
    type Rejection = BasicAuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        ctx: &Arc<Context>,
    ) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let TypedHeader(Authorization(basic)) =
            parts.extract::<TypedHeader<Authorization<Basic>>>().await.map_err(|_| {
                debug!("Authorization header missing or invalid");
                BasicAuthError::MissingAuthorizationHeader
            })?;

        let username = basic.username();
        let password = basic.password();

        // Validate username
        if username != "admin" {
            debug!("Invalid username: {}", username);
            return Err(BasicAuthError::Forbidden);
        }

        // Validate password using HMAC-SHA256 with the existing auth_secret
        if !crypto::hmac_sha256_verify("admin", &ctx.config.auth_secret, password)
            .map_err(BasicAuthError::CryptoError)?
        {
            debug!("Password validation failed");
            return Err(BasicAuthError::InvalidCredentials);
        }

        Ok(AdminBasic)
    }
}
