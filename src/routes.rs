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
    Router,
    routing::{delete, get, patch, post},
};

use crate::{
    context::Context,
    handler::{course, extension, git, stage, webhook},
};

pub fn build() -> Router<Arc<Context>> {
    Router::new()
        .route("/v1/courses", get(course::find))
        .route("/v1/courses", post(course::create))
        .route("/v1/courses/{slug}", get(course::get))
        .route("/v1/courses/{slug}", delete(course::delete))
        .route("/v1/courses/{slug}", patch(course::update))
        //
        .route("/v1/courses/{slug}/attempts", get(course::find_attempts))
        .route("/v1/courses/{slug}/extensions", get(extension::find))
        // git-http-backend
        .route("/git/{repo}/HEAD", get(git::head))
        .route("/git/{repo}/info/refs", get(git::info_refs))
        .route("/git/{repo}/git-upload-pack", post(git::upload_pack))
        .route("/git/{repo}/git-receive-pack", post(git::receive_pack))
        .route("/git/{repo}/objects/info/packs", get(git::info_packs))
        .route("/git/{repo}/objects/pack/{file}", get(git::pack_file))
        .route("/git/{repo}/objects/info/{file}", get(git::text_file))
        .route("/git/{repo}/objects/{two}/{thirtyeight}", get(git::loose_object))
        // Stage
        .route("/v1/courses/{slug}/stages", get(stage::find_all_stages))
        .route("/v1/courses/{slug}/stages/base", get(stage::find_base_stages))
        .route("/v1/courses/{slug}/stages/extended", get(stage::find_extended_stages))
        .route("/v1/courses/{slug}/stages/{stage_slug}", get(stage::get))
        // User course
        .route("/v1/user/courses", get(course::find_user_courses))
        .route("/v1/user/courses", post(course::create_user_course))
        .route("/v1/user/courses/{slug}", get(course::get_user_course))
        .route("/v1/user/courses/{slug}", patch(course::update_user_course))
        .route("/v1/user/courses/{slug}/status", get(course::stream_user_course_status))
        // User stage
        .route("/v1/user/courses/{slug}/stages", get(stage::find_user_stages))
        .route("/v1/user/courses/{slug}/stages", post(stage::complete_stage))
        .route("/v1/user/courses/{slug}/stages/{stage_slug}", get(stage::get_user_stage))
        .route(
            "/v1/user/courses/{slug}/stages/{stage_slug}/status",
            get(stage::stream_user_stage_status),
        )
        // Webhooks
        .route("/v1/webhooks/gitea", post(webhook::handle_gitea_webhook))
}
