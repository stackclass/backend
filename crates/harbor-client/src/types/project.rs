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

/// Request body for creating a project.
/// https://github.com/goharbor/harbor/blob/v2.13.1/api/v2.0/swagger.yaml#L7182
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateProjectRequest {
    /// The name of the project.
    pub project_name: String,

    /// deprecated, reserved for project creation in replication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,

    /// The metadata of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ProjectMetadata>,

    /// The CVE allowlist of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cve_allowlist: Option<CVEAllowlist>,

    /// The storage quota of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_limit: Option<i64>,

    /// The ID of referenced registry when creating the proxy cache project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<i64>,
}

impl CreateProjectRequest {
    pub fn new(name: impl ToString) -> Self {
        Self { project_name: name.to_string(), ..Default::default() }
    }
}

/// Project metadata configuration
/// https://github.com/goharbor/harbor/blob/v2.13.1/api/v2.0/swagger.yaml#L7272
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectMetadata {
    /// The public status of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<String>,

    /// Whether content trust is enabled or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_content_trust: Option<String>,

    /// Whether cosign content trust is enabled or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_content_trust_cosign: Option<String>,

    /// Whether to prevent vulnerable images from running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevent_vul: Option<String>,

    /// Severity threshold for blocking image pulls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,

    /// Whether to scan images automatically when pushing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scan: Option<String>,

    /// Whether to generate SBOM automatically when pushing artifacts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sbom_generation: Option<String>,

    /// Whether to reuse the system level CVE allowlist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_sys_cve_allowlist: Option<String>,

    /// The ID of the tag retention policy for the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_id: Option<String>,

    /// The bandwidth limit of proxy cache, in Kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_speed_kb: Option<String>,
}

/// The CVE Allowlist for system or project
/// https://github.com/goharbor/harbor/blob/v2.13.1/api/v2.0/swagger.yaml#L7358
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CVEAllowlist {
    /// ID of the allowlist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    /// ID of the project which the allowlist belongs to.
    /// For system level allowlist this attribute is zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i32>,

    /// The time for expiration of the allowlist, in the form of seconds since epoch.
    /// This is an optional attribute, if it's not set the CVE allowlist does not expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,

    /// Items in the CVE allowlist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CVEAllowlistItem>>,

    /// The creation time of the allowlist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<DateTime<Utc>>,

    /// The update time of the allowlist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<DateTime<Utc>>,
}

/// The item in CVE allowlist
/// https://github.com/goharbor/harbor/blob/v2.13.1/api/v2.0/swagger.yaml#L7384
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CVEAllowlistItem {
    /// The ID of the CVE, such as "CVE-2019-10164"
    pub cve_id: String,
}
