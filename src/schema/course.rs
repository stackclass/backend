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

use std::{fmt, str::FromStr};

use indexmap::IndexMap;

use serde::{Deserialize, Serialize};

use crate::schema::{ExtensionMap, Stage};

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
    #[serde(skip)]
    pub stages: IndexMap<String, Stage>,

    /// Sets of additional stages.
    #[serde(skip)]
    pub extensions: Option<ExtensionMap>,
}

impl FromStr for Course {
    type Err = serde_yml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yml::from_str(s)
    }
}

/// The release status of the course.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Alpha,
    Beta,
    Live,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Alpha => write!(f, "alpha"),
            Status::Beta => write!(f, "beta"),
            Status::Live => write!(f, "live"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_course_from_str() {
        let yaml = r#"
            slug: rust-course
            name: Rust Programming
            short_name: Rust
            release_status: beta
            description: A comprehensive course on Rust programming language.
            summary: Learn Rust programming
        "#;

        let course = Course::from_str(yaml).unwrap();

        assert_eq!(course.slug, "rust-course");
        assert_eq!(course.name, "Rust Programming");
        assert_eq!(course.short_name, "Rust");
        assert!(matches!(course.release_status, Status::Beta));
        assert_eq!(course.description, "A comprehensive course on Rust programming language.");
        assert_eq!(course.summary, "Learn Rust programming");
    }

    #[test]
    fn test_course_from_str_error() {
        let invalid_yaml = "invalid: yaml: content";
        let result = Course::from_str(invalid_yaml);
        assert!(result.is_err());
    }
}
