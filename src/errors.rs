// Copyright (c) wangeguo. All rights reserved.
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

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;
use tracing::error;

pub type Result<T, E = ApiError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Not Found")]
    NotFound,

    #[error("Bad Request")]
    BadRequest(String),

    #[error("Not Found Workflow: {0}")]
    NotFoundWorkflow(String),

    #[error("Failed to create workflow: {0}")]
    FailedToCreateWorkflow(String),

    #[error("Failed to delete workflow: {0}")]
    FailedToDeleteWorkflow(String),

    #[error("Bad Workflow Request: {0}")]
    BadWorkflowRequest(String),

    #[error("Invalid path")]
    InvalidPath,

    #[error("Axum error: {0}")]
    AxumError(String),

    #[error("Git command failed: {0}")]
    GitCommandFailed(String),
}

impl From<axum::http::Error> for ApiError {
    fn from(err: axum::http::Error) -> Self {
        ApiError::AxumError(err.to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFoundWorkflow(_) => StatusCode::NOT_FOUND,
            Self::FailedToCreateWorkflow(_) => StatusCode::BAD_REQUEST,
            Self::FailedToDeleteWorkflow(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadWorkflowRequest(_) => StatusCode::BAD_REQUEST,
            Self::InvalidPath => StatusCode::BAD_REQUEST,
            Self::AxumError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::GitCommandFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let message = self.to_string();

        error!("{} - {}", status, message);
        (status, Json(json!({ "message": message }))).into_response()
    }
}
