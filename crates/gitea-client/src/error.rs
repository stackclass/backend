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

use reqwest::{Response, StatusCode};
use serde_json::Value;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ClientError>;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not found")]
    NotFound,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Unexpected status code: {0}")]
    UnexpectedStatusCode(StatusCode),
}

impl ClientError {
    /// Constructs a `ClientError` from a `reqwest::Response`.
    pub async fn from_response(response: Response) -> Self {
        let status = response.status();

        // Define a closure to extract the error message from the response
        let message = || async {
            response
                .json::<Value>()
                .await
                .unwrap_or_default()
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string()
        };

        match status {
            StatusCode::BAD_REQUEST => ClientError::BadRequest(message().await),
            StatusCode::FORBIDDEN => ClientError::Forbidden(message().await),
            StatusCode::NOT_FOUND => ClientError::NotFound,
            StatusCode::CONFLICT => ClientError::Conflict(message().await),
            StatusCode::UNPROCESSABLE_ENTITY => ClientError::ValidationError(message().await),

            _ => ClientError::UnexpectedStatusCode(status),
        }
    }
}
