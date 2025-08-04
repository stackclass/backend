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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::PartialUser;

/// A partial representation of a commit,
/// containing only the most essential fields.
#[derive(Debug, Serialize, Deserialize)]
pub struct PartialCommit {
    /// The unique identifier of the commit
    pub id: String,

    /// The commit message describing the changes
    pub message: String,

    /// URL to view the commit in the source control system
    pub url: String,

    /// The author who originally created the changes
    pub author: PartialUser,

    /// The committer who actually committed the changes
    pub committer: PartialUser,

    /// The timestamp when the commit was created
    pub timestamp: DateTime<Utc>,
}
