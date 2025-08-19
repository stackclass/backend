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

use crate::types::{Repository, User};

use super::PartialCommit;

/// GitHub webhook event payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    /// Secret token for verifying the webhook origin.
    pub secret: Option<String>,

    /// The full Git ref that was pushed.
    #[serde(rename = "ref")]
    pub reference: String,

    /// The SHA of the most recent commit on the branch before the push.
    pub before: String,

    /// The SHA of the most recent commit on the branch after the push.
    pub after: String,

    /// URL showing the changes between before and after commits.
    pub compare_url: String,

    /// List of commits included in the push.
    pub commits: Vec<PartialCommit>,

    /// Total number of commits in the push event.
    pub total_commits: u64,

    /// The most recent commit in the push event.
    pub head_commit: PartialCommit,

    /// Repository where the event occurred.
    pub repository: Repository,

    /// User who performed the push.
    pub pusher: User,

    /// User who triggered the event.
    pub sender: User,
}
