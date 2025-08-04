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

use chrono::{DateTime, offset::FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub secret: String,

    #[serde(rename = "ref")]
    pub reference: String,

    pub before: String,
    pub after: String,
    pub compare_url: String,
    pub commits: Vec<Commit>,
    pub repository: Repository,
    pub pusher: User,
    pub sender: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub id: String,
    pub message: String,
    pub url: String,
    pub author: User,
    pub committer: User,
    pub timestamp: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub id: u64,
    pub owner: User,
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub private: bool,
    pub fork: bool,
    pub html_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub website: String,
    pub stars_count: u64,
    pub forks_count: u64,
    pub watchers_count: u64,
    pub open_issues_count: u64,
    pub default_branch: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub login: String,
    pub full_name: String,
    pub email: String,
    pub avatar_url: String,
    pub username: String,
}
