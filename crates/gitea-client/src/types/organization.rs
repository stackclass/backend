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

/// Organization details.
#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    /// URL to the organization's avatar.
    pub avatar_url: String,

    /// Description of the organization.
    pub description: String,

    /// Email address of the organization.
    pub email: String,

    /// Full name of the organization.
    pub full_name: String,

    /// Unique identifier for the organization.
    pub id: u64,

    /// Physical location of the organization.
    pub location: String,

    /// Name of the organization.
    pub name: String,

    /// Whether team access can be changed by repository admins.
    pub repo_admin_change_team_access: bool,

    /// Username of the organization.
    #[deprecated(note = "Use `name` instead.")]
    pub username: String,

    /// Visibility setting of the organization.
    pub visibility: String,

    /// Website URL of the organization.
    pub website: String,
}

/// Request body for creating an organization.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateOrganizationRequest {
    /// Description of the organization.
    pub description: Option<String>,

    /// Email address of the organization.
    pub email: Option<String>,

    /// Full name of the organization.
    pub full_name: Option<String>,

    /// Physical location of the organization.
    pub location: Option<String>,

    /// Whether team access can be changed by repository admins.
    pub repo_admin_change_team_access: Option<bool>,

    /// Name of the organization.
    #[serde(rename = "username")]
    pub name: String,

    /// Visibility setting of the organization.
    pub visibility: Option<String>,

    /// Website URL of the organization.
    pub website: Option<String>,
}
