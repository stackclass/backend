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

/// Represents a solution for a specific stage
#[derive(Debug, FromRow)]
pub struct SolutionModel {
    /// Unique UUID identifier (primary key)
    pub id: Uuid,

    /// Reference to parent stage (foreign key)
    pub stage_id: Uuid,

    /// Detailed solution explanation in Markdown format
    pub explanation: String,

    /// JSON array of file patches (path -> content)
    pub patches: Option<Value>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}
