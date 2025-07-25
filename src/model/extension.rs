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

use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use crate::schema::Extension;

/// Database model representing a course extension
#[derive(Debug, FromRow)]
pub struct ExtensionModel {
    /// Unique internal identifier
    pub id: Uuid,

    /// Reference to parent course
    pub course_id: Uuid,

    /// Unique identifier within course
    pub slug: String,

    /// Extension name
    pub name: String,

    /// Extension description
    pub description: String,

    /// Number of stages in the extension
    pub stage_count: i32,

    /// Sorting weight (default: 0)
    pub weight: i32,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl ExtensionModel {
    /// Sets the course_id field
    pub fn with_course(mut self, id: Uuid) -> ExtensionModel {
        self.course_id = id;
        self
    }

    /// Sets the stage_count field
    pub fn with_stage_count(mut self, stage_count: i32) -> ExtensionModel {
        self.stage_count = stage_count;
        self
    }

    /// Sets the weight field
    pub fn with_weight(mut self, weight: i32) -> ExtensionModel {
        self.weight = weight;
        self
    }
}

impl From<Extension> for ExtensionModel {
    fn from(ext: Extension) -> Self {
        Self {
            id: Uuid::new_v4(),
            // Will be replaced by actual course_id
            course_id: Uuid::new_v4(),
            slug: ext.slug,
            name: ext.name,
            description: ext.description,
            stage_count: 0,
            weight: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
