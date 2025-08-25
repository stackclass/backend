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

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use gitea_client::types::Event;
use std::sync::Arc;
use uuid::Uuid;

use tracing::{error, info};

use crate::{
    context::Context,
    errors::{ApiError, Result},
    repository::CourseRepository,
    request::event::PipelineEvent,
    service::{RepoService, StageService},
    utils::crypto,
};

/// Handle Gitea Webhook Event.
pub async fn handle_gitea_webhook(
    State(ctx): State<Arc<Context>>,
    Json(event): Json<Event>,
) -> Result<impl IntoResponse> {
    let Event { reference, repository, .. } = &event;
    info!("Received push event for repository: {}, ref: {}", repository.full_name, reference);

    // Skip if the event is from the template repository or a non-main branch.
    if repository.template || reference.ne("refs/heads/main") {
        return Ok(StatusCode::OK);
    }

    // Process the push event for the repository.
    RepoService::new(ctx).process(&event).await?;

    Ok(StatusCode::OK)
}

/// Handle Tekton pipeline notification webhook events.
pub async fn handle_tekton_webhook(
    State(ctx): State<Arc<Context>>,
    Json(event): Json<PipelineEvent>,
) -> Result<impl IntoResponse> {
    info!("Received pipeline event for: {} - {}", event.course, event.repo);

    // Verify HMAC signature to prevent request forgery
    let auth_secret = &ctx.config.auth_secret;
    let payload = format!("{}{}{}", event.repo, event.course, event.stage);
    let is_valid = crypto::hmac_sha256_verify(&payload, auth_secret, &event.secret)
        .map_err(|e| ApiError::InternalError(format!("Signature verification failed: {}", e)))?;

    if !is_valid {
        return Err(ApiError::Unauthorized("Invalid signature".into()));
    }

    // Check overall pipeline status first
    if event.status != "Succeeded" {
        error!("Pipeline run failed, please check it. Aggregate status: {}", event.status);
        return Ok(StatusCode::OK);
    }

    // Process the pipeline event based on test task status:
    // - If succeeded: mark the stage as complete for the user
    // - If failed: log error (TODO: collect error details from Tekton and save to database)
    // - Other states: log and ignore non-terminal states
    match event.tasks.test.status.as_str() {
        "Succeeded" => {
            // Look up the course to get the user_id
            let id = Uuid::parse_str(&event.repo)?;
            let course = CourseRepository::get_user_course_by_id(&ctx.database, &id).await?;

            // Mark the stage as complete
            StageService::complete(ctx, &course.user_id, &event.course, &event.stage).await?;

            info!("Stage {} completed successfully for course {}", event.stage, event.course);
        }
        "Failed" => {
            info!("Test task failed: reason={}, stage={}", event.tasks.test.reason, event.stage);
            return Ok(StatusCode::OK);
        }
        _ => {
            error!(
                "Test task in non-terminal state: status={}, reason={}",
                event.tasks.test.status, event.tasks.test.reason
            );
            return Ok(StatusCode::OK);
        }
    }

    Ok(StatusCode::OK)
}
