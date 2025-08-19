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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Indicates whether the user is active.
    pub active: bool,

    /// URL to the user's avatar.
    pub avatar_url: String,

    /// Timestamp when the user was created.
    pub created: DateTime<Utc>,

    /// Description of the user.
    pub description: String,

    /// Email address of the user.
    pub email: String,

    /// Number of followers the user has.
    pub followers_count: u64,

    /// Number of users this user is following.
    pub following_count: u64,

    /// Full name of the user.
    pub full_name: String,

    /// URL to the user's HTML page.
    pub html_url: String,

    /// Unique identifier for the user.
    pub id: u64,

    /// Indicates whether the user is an administrator.
    pub is_admin: bool,

    /// Preferred language of the user.
    pub language: String,

    /// Timestamp of the user's last login.
    pub last_login: DateTime<Utc>,

    /// Physical location of the user.
    pub location: String,

    /// Username of the user.
    pub login: String,

    /// Login name of the user (default: "empty").
    pub login_name: String,

    /// Indicates whether the user is prohibited from logging in.
    pub prohibit_login: bool,

    /// Indicates whether the user is restricted.
    pub restricted: bool,

    /// Source ID of the user.
    pub source_id: u64,

    /// Number of repositories starred by the user.
    pub starred_repos_count: u64,

    /// Username of the user.
    pub username: String,

    /// Visibility setting of the user.
    pub visibility: String,

    /// Website URL of the user.
    pub website: String,
}

/// Request body for creating a user.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateUserRequest {
    /// For explicitly setting the user creation timestamp. Useful when users
    /// are migrated from other systems. When omitted, the user's creation
    /// timestamp will be set to "now".
    pub created_at: Option<DateTime<Utc>>,

    /// Email address of the user.
    pub email: String,

    /// Full name of the user.
    pub full_name: Option<String>,

    /// Login name of the user.
    pub login_name: Option<String>,

    /// Whether the user must change their password on first login.
    pub must_change_password: Option<bool>,

    /// Password for the user account.
    pub password: Option<String>,

    /// Whether the user is restricted.
    pub restricted: Option<bool>,

    /// Whether to send notification email to the user.
    pub send_notify: Option<bool>,

    /// Source ID of the user.
    pub source_id: Option<i64>,

    /// Username of the user.
    pub username: String,

    /// Visibility setting of the user.
    pub visibility: Option<String>,
}

/// A partial representation of a user,
/// containing only the most essential fields.
#[derive(Debug, Serialize, Deserialize)]
pub struct PartialUser {
    /// Full name of the user.
    pub name: String,

    /// Email address of the user.
    pub email: String,

    /// Username of the user.
    pub username: String,
}
