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

use crate::schema::Course;

/// Database model representing a course entity
#[derive(Debug, FromRow)]
pub struct CourseModel {
    /// Unique internal identifier
    pub id: Uuid,

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

    /// The git repository URL of the course
    pub repository: String,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl CourseModel {
    /// Sets the repository field
    pub fn with_repository(mut self, repository: &str) -> CourseModel {
        self.repository = repository.to_string();
        self
    }
}

impl From<&Course> for CourseModel {
    fn from(course: &Course) -> Self {
        Self {
            id: Uuid::new_v4(),
            slug: course.slug.clone(),
            name: course.name.clone(),
            short_name: course.short_name.clone(),
            release_status: course.release_status.to_string(),
            description: course.description.clone(),
            summary: course.summary.clone(),
            repository: String::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
