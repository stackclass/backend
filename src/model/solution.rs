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
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

use crate::schema::Solution;

/// Represents a solution for a specific stage
#[derive(Debug, FromRow)]
pub struct SolutionModel {
    /// Unique UUID identifier (primary key)
    pub id: Uuid,

    /// Reference to parent stage (foreign key)
    pub stage_id: Uuid,

    /// Detailed solution explanation in Markdown format
    pub explanation: String,

    ///
    /// JSON array of file patches.
    /// Patch format: [ ["file/path", "content"], ... ]
    pub patches: Option<Value>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl SolutionModel {
    /// Sets the stage_id field
    pub fn with_stage(mut self, id: Uuid) -> SolutionModel {
        self.stage_id = id;
        self
    }
}

impl From<Solution> for SolutionModel {
    fn from(sol: Solution) -> Self {
        Self {
            id: Uuid::new_v4(),
            // Will be replaced by actual stage_id
            stage_id: Uuid::new_v4(),
            explanation: sol.explanation,
            patches: Some(serde_json::to_value(sol.patches).unwrap()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
