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
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    context::Context, errors::Result, response::ExtensionResponse, service::ExtensionService,
};

// The Extension Service Handlers.

/// Find all extensions for a course.
#[utoipa::path(
    operation_id = "find-all-extensions",
    get, path = "/v1/courses/{slug}/extensions",
    params(
        ("slug" = String, description = "The slug of course"),
    ),
    responses(
        (status = 200, description = "Extensions retrieved successfully", body = Vec<ExtensionResponse>),
        (status = 404, description = "Course not found"),
        (status = 500, description = "Failed to get course")
    ),
    tag = "Extension"
)]
pub async fn find(
    State(ctx): State<Arc<Context>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse> {
    Ok((StatusCode::OK, Json(ExtensionService::find(ctx, &slug).await?)))
}
