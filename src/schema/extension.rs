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

use std::{hash::Hash, str::FromStr};

use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

use crate::schema::Stage;

/// Schema for the extensions.yml file.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Extensions(IndexSet<Extension>);

impl Extensions {
    pub fn iter(&self) -> indexmap::set::Iter<Extension> {
        self.0.iter()
    }
}

impl FromStr for Extensions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yml::from_str(s).map_err(|e| e.to_string())
    }
}

/// Schema for the extension.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Extension {
    /// A unique identifier for this extension.
    pub slug: String,

    /// The name of the extension.
    pub name: String,

    /// A markdown description for this extension.
    pub description: String,

    /// Sequential stages of the extension.
    #[serde(skip)]
    pub stages: IndexSet<Stage>,
}

impl Hash for Extension {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.slug.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extensions_from_str() {
        let yaml = r#"
            - slug: test1
              name: Test Extension 1
              description: Test description 1
            - slug: test2
              name: Test Extension 2
              description: Test description 2
        "#;

        let extensions = Extensions::from_str(yaml).unwrap();

        let mut iter = extensions.iter();

        let first = iter.next().unwrap();
        assert_eq!(first.slug, "test1");
        assert_eq!(first.name, "Test Extension 1");

        let second = iter.next().unwrap();
        assert_eq!(second.slug, "test2");
        assert_eq!(second.name, "Test Extension 2");
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_extensions_from_str_invalid() {
        let invalid_yaml = "invalid: yaml";
        let result = Extensions::from_str(invalid_yaml);
        assert!(result.is_err());
    }
}
