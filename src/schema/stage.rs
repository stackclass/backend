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

use serde::{Deserialize, Serialize};
use std::hash::Hash;

/// A self-contained coding task with specific objectives and validation.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Stage {
    /// A unique identifier for this stage.
    /// Your tester will reference this value.
    pub slug: String,

    /// The name of the stage.
    pub name: String,

    /// A difficulty rating for this stage,
    /// from the perspective of a proficient programmer.
    pub difficulty: Difficulty,

    /// A short markdown description of the stage,
    /// used in the course overview page.
    pub description: String,

    /// A markdown description for this stage.
    pub instruction: String,

    /// The solution to this stage, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution: Option<Solution>,
}

impl Hash for Stage {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.slug.hash(state);
    }
}

/// A difficulty rating,
/// from the perspective of a proficient programmer.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Difficulty {
    VeryEasy, // <5m
    Easy,     // 5-10m
    Medium,   // 30m-1h
    Hard,     // >1h
}

/// Represents a solution to a coding problem or task.
/// Contains an explanation of the solution and a collection of code patches.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Solution {
    /// Detailed description of the solution approach and logic
    pub explanation: String,

    /// Collection of file changes needed to implement this solution
    /// Stored as tuples of (file_path, patch_content)
    pub patches: Vec<(String, String)>,
}

impl Solution {
    /// Creates a new Solution with the given explanation and empty patches
    pub fn new(explanation: String) -> Self {
        Solution { explanation, patches: Vec::new() }
    }

    /// Adds a code patch for a specific file to the solution
    pub fn add_patch(&mut self, path: String, content: String) {
        self.patches.push((path, content));
    }
}
