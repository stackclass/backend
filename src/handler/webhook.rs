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
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::{
    context::Context,
    errors::{ApiError, Result},
    extractor::AdminBasic,
    repository::CourseRepository,
    request::event::PipelineEvent,
    service::{PipelineCleanupGuard, RepoService, StageService},
    utils::crypto,
};

/// Handle Gitea Webhook Event.
pub async fn handle_gitea_webhook(
    _: AdminBasic,
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
    debug!("Received pipeline event: {:?}", event);
    let PipelineEvent { name, status, repo, course, stage, secret, tasks } = &event;

    // Create cleanup guard - will delete pipeline when this function exits
    let _cleanup_guard = PipelineCleanupGuard::new(ctx.clone(), name);

    // Verify HMAC signature to prevent request forgery
    let auth_secret = &ctx.config.auth_secret;
    let payload = format!("{}{}{}", repo, course, stage);

    if !crypto::hmac_sha256_verify(&payload, auth_secret, secret)? {
        error!("Received pipeline event with invalid signature");
        return Err(ApiError::Unauthorized("Invalid signature".into()));
    }

    // Check overall pipeline status first
    if status != "Succeeded" {
        error!("Pipeline run failed, please check it");
        return Ok(StatusCode::OK);
    }

    // Process the pipeline event based on test task status
    match tasks.test.status.as_str() {
        "Succeeded" => {
            // Look up the course to get the user_id
            let id = Uuid::parse_str(repo)?;
            let user_course = CourseRepository::get_user_course_by_id(&ctx.database, &id).await?;

            // Mark the stage as complete
            StageService::complete(ctx.clone(), &user_course.user_id, course, stage).await?;
            info!("Stage {} completed successfully for course {}", stage, course);
        }
        "Failed" => {
            info!("Test task failed: reason={}, stage={}", tasks.test.reason, stage);
        }
        _ => {
            error!(
                "Test task in non-terminal state: status={}, reason={}",
                tasks.test.status, tasks.test.reason
            );
        }
    }

    Ok(StatusCode::OK)
}
