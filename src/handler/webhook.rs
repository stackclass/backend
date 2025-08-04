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
use std::sync::Arc;

use tracing::info;

use crate::{context::Context, errors::Result, request::gitea, service::RepoService};

/// Handle Gitea Webhook Event.
pub async fn handle_gitea_webhook(
    State(ctx): State<Arc<Context>>,
    Json(event): Json<gitea::Event>,
) -> Result<impl IntoResponse> {
    info!(
        "Received push event for repository: {}, ref: {}",
        event.repository.full_name, event.reference
    );

    let repo = &event.repository.name;
    let repo_service = RepoService::new(ctx);
    repo_service.handle_push_event(repo).await?;

    Ok(StatusCode::OK)
}
