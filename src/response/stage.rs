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

use crate::model::StageModel;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StageResponse {
    /// Unique human-readable identifier within parent context
    pub slug: String,

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
            name: model.name,
            difficulty: model.difficulty,
            description: model.description,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
