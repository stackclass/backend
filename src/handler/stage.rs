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
    extract::{Path, State},
    http::StatusCode,
    response::{
        IntoResponse, Sse,
        sse::{Event, KeepAlive},
    },
};
use futures::{Stream, StreamExt};
use std::{convert::Infallible, sync::Arc};
use tokio_stream::wrappers::ReceiverStream;
use tracing::{error, info};

use crate::{
    context::Context,
    errors::Result,
    extractor::Claims,
    request::CompleteStageRequest,
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
    security(("JWTBearerAuth" = [])),
    tags = ["User", "Stage"]
)]
pub async fn find_user_stages(
    claims: Claims,
    State(ctx): State<Arc<Context>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse> {
    Ok((StatusCode::OK, Json(StageService::find_user_stages(ctx, &claims.id, &slug).await?)))
}

/// Get the details of the stage for the current user.
#[utoipa::path(
    operation_id = "get-user-stage-detail",
    get, path = "/v1/user/courses/{slug}/stages/{stage_slug}",
    params(
        ("slug" = String, description = "The slug of course"),
        ("stage_slug" = String, description = "The slug of stage"),
    ),
    responses(
        (status = 200, description = "Stage retrieved successfully", body = UserStageResponse),
        (status = 404, description = "Course or stage not found"),
        (status = 500, description = "Failed to get course or stage")
    ),
    security(("JWTBearerAuth" = [])),
    tags = ["User", "Stage"]
)]
pub async fn get_user_stage(
    claims: Claims,
    State(ctx): State<Arc<Context>>,
    Path((slug, stage_slug)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    let res = StageService::get_user_stage(ctx, &claims.id, &slug, &stage_slug).await?;
    Ok((StatusCode::OK, Json(res)))
}

/// Mark a stage as completed for the current user.
#[utoipa::path(
    operation_id = "complete-stage",
    post, path = "/v1/user/courses/{slug}/stages",
    params(
        ("slug" = String, description = "The slug of course"),
    ),
    request_body(
        content = inline(CompleteStageRequest),
        description = "Complete stage request",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Stage completed successfully", body = UserStageResponse),
        (status = 404, description = "Course or stage not found"),
        (status = 500, description = "Failed to complete stage")
    ),
    security(("JWTBearerAuth" = [])),
    tags = ["User", "Stage"]
)]
pub async fn complete_stage(
    claims: Claims,
    State(ctx): State<Arc<Context>>,
    Path(slug): Path<String>,
    Json(req): Json<CompleteStageRequest>,
) -> Result<impl IntoResponse> {
    let res = StageService::complete(ctx, &claims.id, &slug, &req.slug).await?;
    Ok((StatusCode::OK, Json(res)))
}

/// Stream the status of a specific stage for the current user.
#[utoipa::path(
    operation_id = "stream_user_stage_status",
    get, path = "/v1/user/courses/{slug}/stages/{stage_slug}/status",
    params(
        ("slug" = String, description = "The slug of course"),
        ("stage_slug" = String, description = "The slug of stage"),
    ),
    responses(
        (status = 200, description = "Successfully started streaming stage status updates"),
        (status = 404, description = "Course or stage not found"),
        (status = 500, description = "Failed to stream stage status")
    ),
    security(("JWTBearerAuth" = [])),
    tags = ["User", "Stage"]
)]
pub async fn stream_user_stage_status(
    claims: Claims,
    State(ctx): State<Arc<Context>>,
    Path((slug, stage_slug)): Path<(String, String)>,
) -> Sse<impl Stream<Item = axum::response::Result<Event, Infallible>>> {
    info!(
        "Starting to stream status updates for stage {} in course {} for user {}...",
        stage_slug, slug, claims.id
    );

    // Create a channel for sending status updates.
    let (sender, receiver) = tokio::sync::mpsc::channel(100);

    // Spawn a background task to fetch and send status updates.
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            let status =
                StageService::get_user_stage_status(&ctx, &claims.id, &slug, &stage_slug).await;
            if let Ok(status) = status {
                let event = Event::default().json_data(status).unwrap_or_else(|e| {
                    error!("Failed to serialize status update: {}", e);
                    Event::default().data("status update error")
                });
                if sender.send(event).await.is_err() {
                    break;
                }
            }
        }
    });

    // Convert the receiver into a stream.
    let stream = ReceiverStream::new(receiver);
    let stream = stream.map(Ok);

    // Return the SSE stream with keep-alive.
    Sse::new(stream).keep_alive(KeepAlive::default())
}
