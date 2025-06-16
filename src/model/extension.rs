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

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
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
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
