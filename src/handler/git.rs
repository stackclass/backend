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

use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Path, State},
    http::{Request, StatusCode, Uri, header},
    response::IntoResponse,
};
use base64::{Engine, engine::general_purpose::STANDARD as Base64};
use tracing::{debug, error, warn};
use uuid::Uuid;

use crate::{
    config::Config,
    context::Context,
    database::Database,
    repository::{CourseRepository, UserRepository},
    utils::crypto,
};

/// Proxies a Git request to the appropriate repository in the Git server.
/// This function handles authentication and routing for Git operations.
pub async fn proxy(
    State(ctx): State<Arc<Context>>,
    Path((uuid, path)): Path<(Uuid, String)>,
    mut req: Request<Body>,
) -> impl IntoResponse {
    debug!(%uuid, %path, "Proxying Git request");

    // Look up repository information by UUID (user course ID)
    // Return NOT_FOUND if repository is invalid or doesn't exist
    let (owner, repo, email) = lookup(&ctx.database, &uuid).await.ok_or_else(|| {
        warn!(%uuid, "Git repository not found for UUID");
        StatusCode::NOT_FOUND
    })?;

    let Config { git_server_endpoint, auth_secret, .. } = &ctx.config;

    // Construct the URI for the Git server request to Gitea backend.
    let sanitized_path = path.trim_start_matches('/');
    let uri_str = format!("{git_server_endpoint}/{owner}/{repo}.git/{}", sanitized_path);
    *req.uri_mut() = Uri::try_from(uri_str).map_err(|e| {
        error!(error = %e, "Failed to construct URI for proxy destination");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Generate a password for the user's email using the auth secret and then
    // construct the Basic Auth header value for the Git server request.
    let password = crypto::password(&email, auth_secret);
    let credentials = Base64.encode(format!("{owner}:{password}"));
    let auth_header_value = format!("Basic {credentials}").parse().unwrap();
    req.headers_mut().insert(header::AUTHORIZATION, auth_header_value);

    debug!(uri = %req.uri(), "Forwarding to Git server");
    ctx.http.request(req).await.map_err(|e| {
        error!(error = %e, "Git server request failed");
        StatusCode::BAD_GATEWAY
    })
}

/// Look up the repository information by UUID
async fn lookup(db: &Database, uuid: &Uuid) -> Option<(String, String, String)> {
    let course = CourseRepository::get_user_course_by_id(db, uuid).await.ok()?;
    let user = UserRepository::get_by_id(db, &course.user_id).await.ok()?;

    Some((user.username(), course.course_slug, user.email))
}
