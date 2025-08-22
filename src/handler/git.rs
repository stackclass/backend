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

use std::{io::Error, sync::Arc};

use axum::{
    body::Body,
    extract::{Path, State},
    http::{Request, StatusCode, Uri, header},
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use futures_util::stream::TryStreamExt;
use tracing::{error, info, trace, warn};
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
    Path((uuid, _)): Path<(Uuid, String)>,
    req: Request<Body>,
) -> impl IntoResponse {
    // Look up repository information by UUID (user course ID)
    // Return NOT_FOUND if repository is invalid or doesn't exist
    let (owner, repo, email) = lookup(&ctx.database, &uuid).await.ok_or_else(|| {
        warn!(%uuid, "Git repository not found for UUID");
        StatusCode::NOT_FOUND
    })?;

    let Config { git_server_endpoint, auth_secret, .. } = &ctx.config;
    let password = crypto::password(&email, auth_secret);

    // Construct the URI for the Git server request to Gitea backend.
    let trimmed = strip_uuid_prefix(req.uri(), &uuid);
    let url = format!("{git_server_endpoint}/{owner}/{repo}.git{trimmed}");
    let url = reqwest::Url::parse(&url).map_err(|e| {
        error!(error = %e, "Failed to parse URI for proxy destination");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    info!(url = %url, "Forwarding to Git server");

    // Convert axum Request to reqwest Request with streaming body
    let (mut parts, body) = req.into_parts();

    // Remove the original host header
    parts.headers.remove(header::HOST);

    // Convert axum Body to a stream of bytes for reqwest
    let stream = body.into_data_stream().map_ok(Bytes::from).map_err(|e| {
        error!(error = %e, "Failed to convert body to stream");
        Error::other("Body conversion error")
    });
    let body = reqwest::Body::wrap_stream(stream);

    let request = ctx
        .http
        .request(parts.method, url)
        .headers(parts.headers)
        .basic_auth(owner, Some(password))
        .body(body)
        .build()
        .map_err(|e| {
            error!(error = %e, "Failed to build reqwest request");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    trace!(?request, "Built client request for Git server");

    // Execute the request and get streaming response
    let response = ctx.http.execute(request).await.map_err(|e| {
        error!(error = %e, "Git server request failed");
        StatusCode::BAD_GATEWAY
    })?;
    trace!(?response, "Received response from Git server");

    // Convert reqwest Response to axum Response with streaming body
    let status = response.status();
    let headers = response.headers().clone();
    let body = Body::from_stream(response.bytes_stream());

    let mut response_builder = Response::builder().status(status);
    for (key, value) in headers.iter() {
        response_builder = response_builder.header(key, value);
    }

    response_builder.body(body).map_err(|e| {
        error!(error = %e, "Failed to build response");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

/// Look up the repository information by UUID
async fn lookup(db: &Database, uuid: &Uuid) -> Option<(String, String, String)> {
    let course = CourseRepository::get_user_course_by_id(db, uuid).await.ok()?;
    let user = UserRepository::get_by_id(db, &course.user_id).await.ok()?;

    Some((user.username(), course.course_slug, user.email))
}

/// Strip the leading "/{uuid}" from a request URI and return the remaining path+query.
/// For example:
///   input:  "/5a0e.../info/refs?service=git-receive-pack"
///   output: "/info/refs?service=git-receive-pack"
fn strip_uuid_prefix(uri: &Uri, uuid: &Uuid) -> String {
    let path_and_query = uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("");
    let prefix = format!("/{}", uuid);
    path_and_query.strip_prefix(&prefix).unwrap_or(path_and_query).to_string()
}
