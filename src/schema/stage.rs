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

use serde::{Deserialize, Serialize};
use std::{fmt, hash::Hash, str::FromStr};

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
    #[serde(skip)]
    pub instruction: String,

    /// Detailed description of the solution approach and logic, if available.
    #[serde(skip)]
    pub solution: Option<String>,
}

impl Hash for Stage {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.slug.hash(state);
    }
}

impl FromStr for Stage {
    type Err = serde_yml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yml::from_str(s)
    }
}

/// A difficulty rating,
/// from the perspective of a proficient programmer.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Difficulty {
    VeryEasy, // <5m
    Easy,     // 5-10m
    Medium,   // 30m-1h
    Hard,     // >1h
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Difficulty::VeryEasy => write!(f, "very_easy"),
            Difficulty::Easy => write!(f, "easy"),
            Difficulty::Medium => write!(f, "medium"),
            Difficulty::Hard => write!(f, "hard"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_from_str() {
        let yaml = r#"
            slug: test-stage
            name: Test Stage
            difficulty: easy
            description: A test stage
        "#;

        let stage = Stage::from_str(yaml).unwrap();
        assert_eq!(stage.slug, "test-stage");
        assert_eq!(stage.name, "Test Stage");
        assert_eq!(stage.difficulty, Difficulty::Easy);
        assert_eq!(stage.description, "A test stage");
    }

    #[test]
    fn test_stage_from_str_invalid() {
        let invalid_yaml = "invalid: yaml: content";
        let result = Stage::from_str(invalid_yaml);
        assert!(result.is_err());
    }
}
