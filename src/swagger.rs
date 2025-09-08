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

use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};
use utoipa_swagger_ui::SwaggerUi;

use crate::{handler, request, response};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "StackClass API Reference"
    ),
    paths(
        handler::course::find,
        handler::course::create,
        handler::course::get,
        handler::course::delete,
        handler::course::update,

        handler::course::find_attempts,
        handler::extension::find,

        handler::stage::find_all_stages,
        handler::stage::find_base_stages,
        handler::stage::find_extended_stages,
        handler::stage::get,

        handler::course::find_user_courses,
        handler::course::create_user_course,
        handler::course::get_user_course,
        handler::course::update_user_course,
        handler::course::stream_user_course_status,

        handler::stage::find_user_stages,
        handler::stage::complete_stage,
        handler::stage::get_user_stage,
        handler::stage::stream_user_stage_status
    ),
    components(
        schemas(
            request::CreateCourseRequest,
            response::CourseResponse,
            response::CourseDetailResponse,

            response::AttemptResponse,
            response::ExtensionResponse,

            response::StageResponse,
            response::StageDetailResponse,

            request::CreateUserCourseRequest,
            request::UpdateUserCourseRequest,
            response::UserCourseResponse,
            response::UserStageResponse,
            response::UserStageStatusResponse,
        )
    ),
    tags(
        (name = "Course", description = "The Course Service Handlers"),
        (name = "Extension", description = "The Extension Service Handlers"),
        (name = "Stage", description = "The Stage Service Handlers"),
        (name = "User", description = "The User Service Handlers"),
    ),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "JWTBearerAuth",
                SecurityScheme::Http(
                    HttpBuilder::new().scheme(HttpAuthScheme::Bearer).bearer_format("JWT").build(),
                ),
            );

            components.add_security_scheme(
                "AdminBasicAuth",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Basic).build()),
            )
        }
    }
}

pub fn build() -> SwaggerUi {
    SwaggerUi::new("/swagger").url("/openapi.json", ApiDoc::openapi())
}
