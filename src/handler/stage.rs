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
    context::Context,
    errors::Result,
    response::{StageDetailResponse, StageResponse, UserStageResponse},
    service::StageService,
};

// The Stage Service Handlers.

/// Find all stages for a course (including extensions)
#[utoipa::path(
    operation_id = "find-all-stages",
    get, path = "/v1/courses/{slug}/stages",
    params(
        ("slug" = String, description = "The slug of course"),
    ),
    responses(
        (status = 200, description = "Stages retrieved successfully", body = Vec<StageResponse>),
        (status = 404, description = "Course not found"),
        (status = 500, description = "Failed to get course")
    ),
    tag = "Stage"
)]
pub async fn find_all_stages(
    State(ctx): State<Arc<Context>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse> {
    Ok((StatusCode::OK, Json(StageService::find_all_stages(ctx, &slug).await?)))
}

/// Find only base stages for a course (excluding extensions).
#[utoipa::path(
    operation_id = "find-base-stages",
    get, path = "/v1/courses/{slug}/stages/base",
    params(
        ("slug" = String, description = "The slug of course"),
    ),
    responses(
        (status = 200, description = "Stages retrieved successfully", body = Vec<StageResponse>),
        (status = 404, description = "Course not found"),
        (status = 500, description = "Failed to get course")
    ),
    tag = "Stage"
)]
pub async fn find_base_stages(
    State(ctx): State<Arc<Context>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse> {
    Ok((StatusCode::OK, Json(StageService::find_base_stages(ctx, &slug).await?)))
}

/// Find only extended stages for a course.
#[utoipa::path(
    operation_id = "find-extended-stages",
    get, path = "/v1/courses/{slug}/stages/extended",
    params(
        ("slug" = String, description = "The slug of course"),
    ),
    responses(
        (status = 200, description = "Stages retrieved successfully", body = Vec<StageResponse>),
        (status = 404, description = "Course not found"),
        (status = 500, description = "Failed to get course")
    ),
    tag = "Stage"
)]
pub async fn find_extended_stages(
    State(ctx): State<Arc<Context>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse> {
    Ok((StatusCode::OK, Json(StageService::find_extended_stages(ctx, &slug).await?)))
}

/// Get the details of the stage.
#[utoipa::path(
    operation_id = "get-stage-detail",
    get, path = "/v1/courses/{slug}/stages/{stage_slug}",
    params(
        ("slug" = String, description = "The slug of course"),
        ("stage_slug" = String, description = "The slug of stage"),
    ),
    responses(
        (status = 200, description = "Stage retrieved successfully", body = StageDetailResponse),
        (status = 404, description = "Course or stage not found"),
        (status = 500, description = "Failed to get course or stage")
    ),
    tag = "Stage"
)]
pub async fn get(
    State(ctx): State<Arc<Context>>,
    Path((slug, stage_slug)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    Ok((StatusCode::OK, Json(StageService::get(ctx, &slug, &stage_slug).await?)))
}

/// Find all stages for the current user.
#[utoipa::path(
    operation_id = "find-user-stages",
    get, path = "/v1/user/courses/{slug}/stages",
    params(
        ("slug" = String, description = "The slug of course"),
    ),
    responses(
        (status = 200, description = "Stages retrieved successfully", body = Vec<UserStageResponse>),
        (status = 404, description = "Course not found"),
        (status = 500, description = "Failed to get course")
    ),
    tags = ["User", "Stage"]
)]
pub async fn find_user_stages(
    State(ctx): State<Arc<Context>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse> {
    Ok((StatusCode::OK, Json(StageService::find_user_stages(ctx, "<user_id>", &slug).await?)))
}
