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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::model::CourseModel;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CourseResponse {
    /// Unique human-readable identifier
    pub slug: String,

    // Full course name
    pub name: String,

    // Short display name
    pub short_name: String,

    /// Release status (alpha/beta/live)
    pub release_status: String,

    /// Brief summary
    pub summary: String,

    /// URL or path to the course logo
    pub logo: String,

    /// Number of stages in the course
    pub stage_count: i32,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl From<CourseModel> for CourseResponse {
    fn from(model: CourseModel) -> Self {
        Self {
            slug: model.slug,
            name: model.name,
            short_name: model.short_name,
            release_status: model.release_status,
            summary: model.summary,
            logo: model.logo,
            stage_count: model.stage_count,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CourseDetailResponse {
    /// Unique human-readable identifier
    pub slug: String,

    // Full course name
    pub name: String,

    // Short display name
    pub short_name: String,

    /// Release status (alpha/beta/live)
    pub release_status: String,

    /// Detailed description
    pub description: String,

    /// Brief summary
    pub summary: String,

    /// URL or path to the course logo
    pub logo: String,

    /// Number of stages in the course
    pub stage_count: i32,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl From<CourseModel> for CourseDetailResponse {
    fn from(model: CourseModel) -> Self {
        Self {
            slug: model.slug,
            name: model.name,
            short_name: model.short_name,
            release_status: model.release_status,
            description: model.description,
            summary: model.summary,
            logo: model.logo,
            stage_count: model.stage_count,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
