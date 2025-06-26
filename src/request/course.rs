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

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateCourseRequest {
    /// The git repository URL of the course
    pub repository: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserCourseRequest {
    /// The slug of the course to enroll in
    pub course_slug: String,

    /// Language proficiency level of the user
    pub proficiency: String,

    /// Practice cadence of the user
    pub cadence: String,

    /// Whether the user wants accountability emails
    pub accountability: bool,
}
