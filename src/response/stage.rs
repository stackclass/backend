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
use serde_json::Value;
use utoipa::ToSchema;

use crate::model::{SolutionModel, StageModel, UserStageModel};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StageResponse {
    /// Unique human-readable identifier within parent context
    pub slug: String,

    /// Optional slug of the parent extension (null if part of main course)
    pub extension_slug: Option<String>,

    /// Display name of the stage
    pub name: String,

    /// Difficulty level (very_easy, easy, medium, hard)
    pub difficulty: String,

    /// A short markdown description of the stage,
    /// used in the course overview page.
    pub description: String,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl From<StageModel> for StageResponse {
    fn from(model: StageModel) -> Self {
        Self {
            slug: model.slug,
            extension_slug: model.extension_slug,
            name: model.name,
            difficulty: model.difficulty,
            description: model.description,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StageDetailResponse {
    /// Unique human-readable identifier within parent context
    pub slug: String,

    /// Optional slug of the parent extension (null if part of main course)
    pub extension_slug: Option<String>,

    /// Display name of the stage
    pub name: String,

    /// Difficulty level (very_easy, easy, medium, hard)
    pub difficulty: String,

    /// A short markdown description of the stage,
    /// used in the course overview page.
    pub description: String,

    /// A markdown description for this stage.
    pub instruction: String,

    /// The solution to this stage, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution: Option<SolutionResponse>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl From<StageModel> for StageDetailResponse {
    fn from(model: StageModel) -> Self {
        Self {
            slug: model.slug,
            extension_slug: model.extension_slug,
            name: model.name,
            difficulty: model.difficulty,
            description: model.description,
            instruction: model.instruction,
            solution: None,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SolutionResponse {
    /// Detailed description of the solution approach and logic
    pub explanation: String,

    /// Collection of file changes needed to implement this solution
    /// Stored as tuples of (file_path, patch_content)
    pub patches: Vec<(String, String)>,
}

impl From<SolutionModel> for SolutionResponse {
    fn from(model: SolutionModel) -> Self {
        fn parse_patch_entry(value: &Value) -> Option<(String, String)> {
            value.as_array().and_then(|pair| {
                if pair.len() == 2 {
                    Some((pair[0].as_str()?.to_owned(), pair[1].as_str()?.to_owned()))
                } else {
                    None
                }
            })
        }

        let SolutionModel { explanation, patches, .. } = model;

        let patches = patches
            .as_ref()
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(parse_patch_entry).collect())
            .unwrap_or_default();

        Self { explanation, patches }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserStageResponse {
    /// Slug of the enrolled course
    pub course_slug: String,

    /// Slug of the stage
    pub stage_slug: String,

    /// Current progress status (PENDING, IN_PROGRESS, COMPLETED)
    pub status: String,

    /// Timestamp when the stage was started
    pub started_at: DateTime<Utc>,

    /// Timestamp when the stage was completed
    pub completed_at: Option<DateTime<Utc>>,
}

impl From<UserStageModel> for UserStageResponse {
    fn from(model: UserStageModel) -> Self {
        Self {
            course_slug: model.course_slug,
            stage_slug: model.stage_slug,
            status: model.status,
            started_at: model.started_at,
            completed_at: model.completed_at,
        }
    }
}
