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

use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

use crate::schema::{Extension, Stage};

/// Schema for the course.yml file.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Course {
    /// A unique identifier for this course.
    pub slug: String,

    /// The name of the course.
    pub name: String,

    /// A short name for this course.
    pub short_name: String,

    /// The release status of the course.
    pub release_status: Status,

    /// A markdown description for this course. 25-50 words.
    pub description: String,

    /// A short description of course, < 15 words.
    pub summary: String,

    /// Sequential stages of the course.
    pub stages: IndexSet<Stage>,

    /// Sets of additional stages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<IndexSet<Extension>>,
}

/// The release status of the course.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Status {
    Alpha,
    Beta,
    Live,
}
