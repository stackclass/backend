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

use indexmap::{IndexMap, IndexSet, set::Iter};
use serde::{Deserialize, Serialize};
use std::{hash::Hash, str::FromStr};

use crate::schema::Stage;

pub type ExtensionMap = IndexMap<String, Extension>;

impl From<ExtensionSet> for ExtensionMap {
    fn from(val: ExtensionSet) -> Self {
        val.0.into_iter().map(|ext| (ext.slug.clone(), ext)).collect()
    }
}

/// Schema for the extensions.yml file.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ExtensionSet(pub(crate) IndexSet<Extension>);

impl ExtensionSet {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> Iter<Extension> {
        self.0.iter()
    }
}

impl Default for ExtensionSet {
    fn default() -> Self {
        Self(IndexSet::new())
    }
}

impl FromStr for ExtensionSet {
    type Err = serde_yml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yml::from_str(s)
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
    pub stages: IndexMap<String, Stage>,
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

        let extensions = ExtensionSet::from_str(yaml).unwrap();
        let extensions: ExtensionMap = extensions.into();

        let test1 = extensions.get("test1").unwrap();
        assert_eq!(test1.slug, "test1");
        assert_eq!(test1.name, "Test Extension 1");

        let test2 = extensions.get("test2").unwrap();
        assert_eq!(test2.slug, "test2");
        assert_eq!(test2.name, "Test Extension 2");
    }

    #[test]
    fn test_extensions_from_str_invalid() {
        let invalid_yaml = "invalid: yaml";
        let result = ExtensionSet::from_str(invalid_yaml);
        assert!(result.is_err());
    }
}
