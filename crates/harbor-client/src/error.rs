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

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not found")]
    NotFound,

    #[error("Method not allowed: {0}")]
    MethodNotAllowed(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Precondition failed: {0}")]
    PreconditionFailed(String),

    #[error("Unsupported MediaType: {0}")]
    UnsupportedMediaType(String),

    #[error("Unsupported Type: {0}")]
    UnsupportedType(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Unexpected status code: {0}")]
    UnexpectedStatusCode(StatusCode),
}

use ClientError::*;
impl ClientError {
    /// Constructs a `ClientError` from a `reqwest::Response`.
    pub async fn from_response(response: Response) -> Self {
        let status = response.status();
        let value = response.json::<Value>().await.ok();

        match status {
            StatusCode::BAD_REQUEST => BadRequest(message(value, status)),
            StatusCode::UNAUTHORIZED => Unauthorized(message(value, status)),
            StatusCode::FORBIDDEN => Forbidden(message(value, status)),
            StatusCode::NOT_FOUND => NotFound,
            StatusCode::METHOD_NOT_ALLOWED => MethodNotAllowed(message(value, status)),
            StatusCode::CONFLICT => Conflict(message(value, status)),
            StatusCode::PRECONDITION_FAILED => PreconditionFailed(message(value, status)),
            StatusCode::UNSUPPORTED_MEDIA_TYPE => UnsupportedMediaType(message(value, status)),
            StatusCode::UNPROCESSABLE_ENTITY => UnsupportedType(message(value, status)),
            StatusCode::INTERNAL_SERVER_ERROR => InternalServerError(message(value, status)),

            _ => UnexpectedStatusCode(status),
        }
    }
}

/// Extracts an error message from a JSON value.
fn message(value: Option<Value>, status: StatusCode) -> String {
    value
        .and_then(|json| {
            // Extract the first error message from the errors array
            json.get("errors")
                .and_then(|errors| errors.as_array())
                .and_then(|errors| errors.first())
                .and_then(|error| error.get("message"))
                .and_then(|msg| msg.as_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| {
            // Fallback to status text if no message found
            status.canonical_reason().unwrap_or("Unknown error").to_string()
        })
}
