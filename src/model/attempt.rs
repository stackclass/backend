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

use sqlx::FromRow;

/// Database model representing a user's attempt in a course
#[derive(Debug, FromRow)]
pub struct AttemptModel {
    /// Unique identifier of the user
    pub user_id: String,

    /// URL of the user's avatar image
    pub avatar: String,

    /// The display name of the user
    pub username: String,

    /// Number of tasks completed by the user
    pub completed: i32,

    /// Total number of tasks available
    pub total: i32,
}
