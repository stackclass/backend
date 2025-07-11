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

use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Schema for the stackclass.yml file.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manifest {
    /// Set this to true if you want debug logs.
    pub debug: bool,

    /// Use this to change the compiler version used to run your code.
    pub language_pack: String,

    /// The executable required to build and run the this project.
    pub required_executable: String,

    /// The main source file that users can edit for this project.
    pub user_editable_file: String,
}

impl FromStr for Manifest {
    type Err = serde_yml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yml::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_from_str() {
        let yaml = r#"
            debug: true
            language_pack: "rust-1.70.0"
            required_executable: "cargo"
            user_editable_file: "src/main.rs"
        "#;

        let manifest = Manifest::from_str(yaml).unwrap();
        assert!(manifest.debug);
        assert_eq!(manifest.language_pack, "rust-1.70.0");
        assert_eq!(manifest.required_executable, "cargo");
        assert_eq!(manifest.user_editable_file, "src/main.rs");
    }

    #[test]
    fn test_manifest_from_str_error() {
        let invalid_yaml = "invalid: yaml: content";
        let result = Manifest::from_str(invalid_yaml);
        assert!(result.is_err());
    }
}
