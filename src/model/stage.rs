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

use crate::schema::Stage;

/// Represents a learning stage within a course or extension
#[derive(Debug, FromRow)]
pub struct StageModel {
    /// Unique UUID identifier (primary key)
    pub id: Uuid,

    /// Reference to parent course (foreign key)
    pub course_id: Uuid,

    /// Optional reference to parent extension (null if part of main course)
    pub extension_id: Option<Uuid>,

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

    /// Detailed description of the solution approach and logic, if available.
    pub solution: Option<String>,

    /// Sorting weight (default: 0)
    pub weight: i32,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl StageModel {
    /// Sets the course_id field
    pub fn with_course(mut self, id: Uuid) -> StageModel {
        self.course_id = id;
        self
    }

    /// Sets the extension_id field
    pub fn with_extension(mut self, id: Uuid) -> StageModel {
        self.extension_id = Some(id);
        self
    }

    /// Sets the weight field
    pub fn with_weight(mut self, weight: i32) -> StageModel {
        self.weight = weight;
        self
    }
}

impl From<Stage> for StageModel {
    fn from(stage: Stage) -> Self {
        Self {
            id: Uuid::new_v4(),
            // Will be replaced by actual course_id
            course_id: Uuid::new_v4(),
            // Will be replaced by actual extension_id
            extension_id: None,
            extension_slug: None,
            slug: stage.slug,
            name: stage.name,
            difficulty: stage.difficulty.to_string(),
            description: stage.description,
            instruction: stage.instruction,
            solution: stage.solution,
            weight: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

/// Database model representing a user's progress in a course stage
#[derive(Debug, FromRow)]
pub struct UserStageModel {
    /// Unique internal identifier
    pub id: Uuid,

    /// ID of the user's course enrollment
    pub user_course_id: Uuid,

    /// Slug of the enrolled course
    pub course_slug: String,

    /// ID of the stage
    pub stage_id: Uuid,

    /// Slug of the stage
    pub stage_slug: String,

    /// Current progress status (in_progress, completed)
    pub status: String,

    /// Test result status (passed, failed)
    pub test: String,

    /// Timestamp when the stage was started
    pub started_at: DateTime<Utc>,

    /// Timestamp when the stage was completed
    pub completed_at: Option<DateTime<Utc>>,
}

impl UserStageModel {
    /// Creates a new instance with default values
    pub fn new(user_course_id: Uuid, stage_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_course_id,
            course_slug: String::new(),
            stage_id,
            stage_slug: String::new(),
            status: "in_progress".to_string(),
            test: "failed".to_string(),
            started_at: Utc::now(),
            completed_at: None,
        }
    }

    /// Marks the stage as test passed
    pub fn passed(mut self) -> Self {
        self.test = "passed".to_string();
        self
    }

    /// Marks the stage as completed with current timestamp
    pub fn complete(mut self) -> Self {
        self.status = "completed".to_string();
        self.completed_at = Some(Utc::now());
        self
    }
}
