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

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{handler, request, response};

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::course::find,
        handler::course::create,
        handler::course::get,
        handler::course::delete,
        handler::course::update,
    ),
    components(
        schemas(
            request::CreateCourseRequest,
            response::CourseResponse,
        )
    ),
    tags(
        (name = "Course", description = "The Course Service Handlers"),
    ),
)]
pub struct ApiDoc;

pub fn build() -> SwaggerUi {
    SwaggerUi::new("/swagger").url("/openapi.json", ApiDoc::openapi())
}
