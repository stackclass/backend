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

/// Schema for the codecraft.yml file.
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
