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

use tracing::info;

use crate::{constant::TEMPLATE_OWNER, context::Context, errors::Result, service::RepoService};

/// Handle Gitea Webhook Event.
pub async fn handle_gitea_webhook(
    State(ctx): State<Arc<Context>>,
    Json(event): Json<Event>,
) -> Result<impl IntoResponse> {
    let Event { reference, repository, .. } = &event;
    info!("Received push event for repository: {}, ref: {}", repository.full_name, reference);

    // Skip if the event is from the template repository or a non-main branch.
    let owner = &repository.owner.username;
    if owner.eq(TEMPLATE_OWNER) || reference.ne("refs/heads/main") {
        return Ok(StatusCode::OK);
    }

    // Process the push event for the repository.
    RepoService::new(ctx).process(&event).await?;

    Ok(StatusCode::OK)
}
