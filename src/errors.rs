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

use axum::{
    http::{self, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use thiserror::Error;
use tracing::error;

use crate::{schema, service::StorageError};

pub type Result<T, E = ApiError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Not Found")]
    NotFound,

    #[error("Bad Request")]
    BadRequest(String),

    #[error("HTTP error: {0}")]
    HTTPError(#[from] http::Error),

    #[error("Git command failed: {0}")]
    GitCommandFailed(String),

    #[error("Storage service error: {0}")]
    StorageError(#[from] StorageError),

    #[error("Schema parse error: {0}")]
    SchemaParserError(#[from] schema::ParseError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Migrate error: {0}")]
    MigrateError(#[from] sqlx::migrate::MigrateError),

    #[error("Stage is already completed")]
    StageAlreadyCompleted,

    #[error("Stage must be in progress to complete")]
    StageNotInProgress,

    #[error("Cannot complete a stage out of order")]
    StageOutOfOrder,

    #[error("User is not enrolled in the course")]
    UserNotEnrolled,

    #[error("Course not found")]
    CourseNotFound,

    #[error("Stage not found")]
    StageNotFound,

    #[error("Conflict: {0}")]
    Conflict(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::HTTPError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::GitCommandFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::StorageError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::SchemaParserError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::MigrateError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::StageAlreadyCompleted => StatusCode::BAD_REQUEST,
            ApiError::StageNotInProgress => StatusCode::BAD_REQUEST,
            ApiError::StageOutOfOrder => StatusCode::BAD_REQUEST,
            ApiError::UserNotEnrolled => StatusCode::BAD_REQUEST,
            ApiError::CourseNotFound => StatusCode::NOT_FOUND,
            ApiError::StageNotFound => StatusCode::NOT_FOUND,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
        };
        let message = self.to_string();

        error!("{} - {}", status, message);
        (status, Json(json!({ "message": message }))).into_response()
    }
}
