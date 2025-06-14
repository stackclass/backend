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

use crate::{
    context::Context,
    errors::{ApiError, Result},
    services::git::GitService,
};
use axum::{
    body::Bytes,
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use std::{collections::HashMap, sync::Arc};
use tracing::debug;

// Type aliases for clearer path parameters
type RepoPath = Path<String>;
type RepoAndFilePath = Path<(String, String)>;
type LooseObjectPath = Path<(String, String, String)>;

pub async fn info_refs(
    State(ctx): State<Arc<Context>>,
    Path(repo): RepoPath,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse> {
    debug!("git::info_refs({}) #{:?}", repo, params);

    let service = GitService::new(&ctx.config.repo_dir);

    if !service.exists(&repo).await {
        return Err(ApiError::NotFound);
    }

    let (protocol, srv) = match params.get("service") {
        Some(s) if s == "git-upload-pack" => (&format!("001e# service={s}\n0000"), "upload-pack"),
        Some(s) if s == "git-receive-pack" => (&format!("001f# service={s}\n0000"), "receive-pack"),
        _ => return Err(ApiError::BadRequest("Invalid service".into())),
    };

    let refs = service.get_info_refs(&repo, srv).await?;

    let mut body = String::new();
    body.push_str(protocol);
    body.push_str(&String::from_utf8(refs).unwrap());

    let content_type = format!("application/x-git-{}-advertisement", srv);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .body(body.into_response())?)
}

pub async fn upload_pack(
    State(ctx): State<Arc<Context>>,
    Path(repo): RepoPath,
    body: Bytes,
) -> Result<impl IntoResponse> {
    debug!("git::upload_pack({}) #{:?}", repo, body);

    let service = GitService::new(&ctx.config.repo_dir);

    if !service.exists(&repo).await {
        return Err(ApiError::NotFound);
    }

    let data = service.upload_pack(&repo, body.to_vec()).await?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/x-git-upload-pack-result")
        .body(data.into_response())?)
}

pub async fn receive_pack(
    State(ctx): State<Arc<Context>>,
    Path(repo): RepoPath,
    body: Bytes,
) -> Result<impl IntoResponse> {
    debug!("git::receive_pack({}) #{:?}", repo, body);

    let service = GitService::new(&ctx.config.repo_dir);

    if !service.exists(&repo).await {
        return Err(ApiError::NotFound);
    }

    let data = service.receive_pack(&repo, body.to_vec()).await?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/x-git-receive-pack-result")
        .body(data.into_response())?)
}

#[inline]
pub async fn head(
    State(ctx): State<Arc<Context>>,
    Path(repo): RepoPath,
) -> Result<impl IntoResponse> {
    text_file(State(ctx), Path((repo, "HEAD".to_string()))).await
}

pub async fn text_file(
    State(ctx): State<Arc<Context>>,
    Path((repo, file)): RepoAndFilePath,
) -> Result<impl IntoResponse> {
    debug!("git::text_file({}, {})", repo, file);

    let service = GitService::new(&ctx.config.repo_dir);
    let content = service.get_text_file(&repo, &file).await?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain")
        .body(content.into_response())?)
}

pub async fn info_packs(
    State(ctx): State<Arc<Context>>,
    Path(repo): RepoPath,
) -> Result<impl IntoResponse> {
    debug!("git::info_packs({})", repo);

    let service = GitService::new(&ctx.config.repo_dir);
    let content = service.get_info_packs(&repo).await?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain")
        .body(content.into_response())?)
}

pub async fn loose_object(
    State(ctx): State<Arc<Context>>,
    Path((repo, two, thirtyeight)): LooseObjectPath,
) -> Result<impl IntoResponse> {
    debug!("git::loose_object({}, {}, {})", repo, two, thirtyeight);

    let service = GitService::new(&ctx.config.repo_dir);
    let path = format!("objects/{}/{}", two, thirtyeight);

    let content = service.get_loose_object(&repo, &path).await?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/x-git-loose-object")
        .body(content.into_response())?)
}

pub async fn pack_file(
    State(ctx): State<Arc<Context>>,
    Path((repo, file)): RepoAndFilePath,
) -> Result<impl IntoResponse> {
    debug!("git::pack_file({}, {})", repo, file);

    let (_, _, ext) = parse_pack_filename(&file)?;

    let service = GitService::new(&ctx.config.repo_dir);
    let path = format!("objects/pack/{}", file);
    let content = service.get_pack_file(&repo, &path).await?;

    let content_type = match ext {
        "pack" => "application/x-git-packed-objects",
        "idx" => "application/x-git-packed-objects-toc",
        _ => return Err(ApiError::BadRequest("Invalid file type".into())),
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .body(content.into_response())?)
}

// pack-<hash>.<ext>
fn parse_pack_filename(filename: &str) -> Result<(&str, &str, &str)> {
    let mut parts = filename.splitn(3, ['-', '.']);

    let file_type =
        parts.next().ok_or_else(|| ApiError::BadRequest("Invalid filename format".into()))?;

    let hash =
        parts.next().ok_or_else(|| ApiError::BadRequest("Missing hash in filename".into()))?;

    let ext =
        parts.next().ok_or_else(|| ApiError::BadRequest("Missing extension in filename".into()))?;

    if hash.len() != 40 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ApiError::BadRequest("Invalid hash format".into()));
    }

    Ok((file_type, hash, ext))
}
