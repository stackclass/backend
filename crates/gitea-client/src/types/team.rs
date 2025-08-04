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

use super::Organization;

/// Team details.
#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    /// Whether the team can create organization repositories.
    pub can_create_org_repo: bool,

    /// Description of the team.
    pub description: String,

    /// Unique identifier for the team.
    pub id: u64,

    /// Whether the team includes all repositories.
    pub includes_all_repositories: bool,

    /// Name of the team.
    pub name: String,

    /// Organization the team belongs to.
    pub organization: Organization,

    /// Permission level of the team.
    pub permission: String,

    /// Units the team has access to.
    pub units: Vec<String>,

    /// Mapping of units to permission levels.
    pub units_map: std::collections::HashMap<String, String>,
}
