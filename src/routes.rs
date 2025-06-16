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

use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::{
    context::Context,
    handler::{course, git, workflow},
};

pub fn build() -> Router<Arc<Context>> {
    Router::new()
        .route("/v1/courses", get(course::list))
        .route("/v1/courses", post(course::create))
        .route("/v1/courses/{slug}", get(course::get))
        .route("/v1/courses/{slug}", delete(course::delete))
        // git-http-backend
        .route("/git/{repo}/HEAD", get(git::head))
        .route("/git/{repo}/info/refs", get(git::info_refs))
        .route("/git/{repo}/git-upload-pack", post(git::upload_pack))
        .route("/git/{repo}/git-receive-pack", post(git::receive_pack))
        .route("/git/{repo}/objects/info/packs", get(git::info_packs))
        .route("/git/{repo}/objects/pack/{file}", get(git::pack_file))
        .route("/git/{repo}/objects/info/{file}", get(git::text_file))
        .route("/git/{repo}/objects/{two}/{thirtyeight}", get(git::loose_object))
        // workflows
        .route("/v1/workflows", post(workflow::create))
        .route("/v1/workflows/{id}", delete(workflow::delete))
        .route("/v1/workflows/{id}", get(workflow::get))
}
