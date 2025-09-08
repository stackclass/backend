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

use axum::{
    Json,
    http::{self, StatusCode},
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;
use tracing::{debug, error};

use crate::{
    schema,
    service::StorageError,
    utils::{crypto::CryptoError, git::GitError},
};

pub type Result<T, E = ApiError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Bad Request")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Not Found")]
    NotFound,

    #[error("Record already exists")]
    Conflict,

    #[error("Internal Error: {0}")]
    InternalError(String),

    #[error("HTTP error: {0}")]
    HTTPError(#[from] http::Error),

    #[error("Storage service error: {0}")]
    StorageError(#[from] StorageError),

    #[error("Schema parse error: {0}")]
    SchemaParserError(#[from] schema::ParseError),

    #[error("Database error: {0}")]
    DatabaseError(sqlx::Error),

    #[error("Migrate error: {0}")]
    MigrateError(#[from] sqlx::migrate::MigrateError),

    #[error("Stage is already completed")]
    StageAlreadyCompleted,

    #[error("Stage must be in progress to complete")]
    StageNotInProgress,

    #[error("Cannot complete a stage out of order")]
    StageOutOfOrder,

    #[error("Gitea client error: {0}")]
    GiteaClientError(#[from] gitea_client::ClientError),

    #[error("Git error: {0}")]
    GitError(#[from] GitError),

    #[error("Kubernetes Error: {0}")]
    KubernetesError(#[from] kube::Error),

    #[error("Serialization Error: {0}")]
    SerializationError(#[source] serde_json::Error),

    #[error("Url Parser Error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Invalid UUID: {0}")]
    InvalidUuid(#[from] uuid::Error),

    #[error("Harbor client error: {0}")]
    HarborClientError(#[from] harbor_client::ClientError),

    #[error("Crypto operation failed")]
    CryptoError(#[from] CryptoError),
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::Database(err) if err.is_unique_violation() => ApiError::Conflict,
            sqlx::Error::RowNotFound => ApiError::NotFound,
            _ => ApiError::DatabaseError(e),
        }
    }
}

impl From<&ApiError> for StatusCode {
    fn from(val: &ApiError) -> Self {
        match val {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Conflict => StatusCode::CONFLICT,
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::HTTPError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::StorageError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::SchemaParserError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::MigrateError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::StageAlreadyCompleted => StatusCode::BAD_REQUEST,
            ApiError::StageNotInProgress => StatusCode::BAD_REQUEST,
            ApiError::StageOutOfOrder => StatusCode::BAD_REQUEST,
            ApiError::GiteaClientError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::GitError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::KubernetesError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::SerializationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UrlParseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InvalidUuid(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::HarborClientError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::CryptoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        AutoIntoResponse::into(&self)
    }
}

/// Automatically implement `IntoResponse` for types that satisfy:
/// - `impl thiserror::Error`
/// - `impl Into<StatusCode> for &T`
pub trait AutoIntoResponse: std::error::Error + Sized
where
    for<'a> StatusCode: From<&'a Self>,
{
    fn into(&self) -> Response;
}

impl<T> AutoIntoResponse for T
where
    T: std::error::Error + Sized,
    for<'a> StatusCode: From<&'a T>,
{
    fn into(&self) -> Response {
        let status = StatusCode::from(self);
        let message = self.to_string();

        if status.is_server_error() {
            error!("{} - {} - {:?}", status, message, self);
        } else {
            debug!("{} - {}", status, message);
        }

        (status, Json(json!({ "message": message }))).into_response()
    }
}
