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

    /// URL or path to the course logo
    pub logo: String,

    /// Number of stages in the course
    pub stage_count: i32,

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

    /// Sets the stage_count field
    pub fn with_stage_count(mut self, stage_count: i32) -> CourseModel {
        self.stage_count = stage_count;
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
            logo: String::new(),
            stage_count: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

/// Database model representing a user's enrollment in a course
#[derive(Debug, FromRow)]
pub struct UserCourseModel {
    /// Unique internal identifier
    pub id: Uuid,

    /// ID of the enrolled user
    pub user_id: String,

    /// ID of the enrolled course
    pub course_id: Uuid,

    /// Slug of the enrolled course
    pub course_slug: String,

    /// Timestamp when the enrollment started
    pub started_at: DateTime<Utc>,

    /// ID of the current stage the user is on
    pub current_stage_id: Option<Uuid>,

    /// Slug of the current stage the user is on
    pub current_stage_slug: Option<String>,

    /// Number of stages completed by the user
    pub completed_stage_count: i32,

    /// Language proficiency level of the user
    pub proficiency: String,

    /// Practice cadence of the user
    pub cadence: String,

    /// Whether the user wants accountability emails
    pub accountability: bool,

    /// Whether the first Git push was received
    pub activated: bool,
}

impl Default for UserCourseModel {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: String::new(),
            course_id: Uuid::new_v4(),
            course_slug: String::new(),
            started_at: Utc::now(),
            current_stage_id: None,
            current_stage_slug: None,
            completed_stage_count: 0,
            proficiency: "beginner".to_string(),
            cadence: "weekly".to_string(),
            accountability: false,
            activated: false,
        }
    }
}

impl UserCourseModel {
    /// Creates a new instance with default values
    pub fn new(user_id: &str, course_id: &Uuid) -> Self {
        Self { user_id: user_id.to_string(), course_id: *course_id, ..Default::default() }
    }

    /// Sets the proficiency field
    pub fn with_proficiency(mut self, proficiency: &str) -> Self {
        self.proficiency = proficiency.to_string();
        self
    }

    /// Sets the cadence field
    pub fn with_cadence(mut self, cadence: &str) -> Self {
        self.cadence = cadence.to_string();
        self
    }

    /// Sets the accountability field
    pub fn with_accountability(mut self, accountability: bool) -> Self {
        self.accountability = accountability;
        self
    }
}
