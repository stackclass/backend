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
use std::collections::HashMap;

// Hook event types and definitions based on Gitea's webhook implementation.
// See official source for complete reference:
// https://github.com/go-gitea/gitea/blob/main/modules/webhook/type.go

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hook {
    /// Indicates whether the hook is active.
    pub active: bool,

    /// Authorization header for the hook.
    pub authorization_header: Option<String>,

    /// Branch filter for the hook.
    pub branch_filter: Option<String>,

    /// Configuration for the hook.
    pub config: HashMap<String, String>,

    /// Timestamp when the hook was created.
    pub created_at: DateTime<Utc>,

    /// Events that trigger the hook.
    pub events: Vec<String>,

    /// Unique identifier for the hook.
    pub id: u64,

    /// Type of the hook.
    #[serde(rename = "type")]
    pub kind: String,

    /// Timestamp when the hook was last updated.
    pub updated_at: DateTime<Utc>,
}

/// Checks if a hook matches the configuration of a create request.
/// This is used to avoid duplicate hooks with the same settings.
pub fn matching(hook: &Hook, req: &CreateHookRequest) -> bool {
    hook.kind == req.kind &&
        hook.config == req.config &&
        hook.branch_filter == req.branch_filter &&
        hook.events == req.events
}

/// Request body for creating a hook.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateHookRequest {
    /// Indicates whether the hook is active.
    pub active: bool,

    /// Authorization header for the hook.
    pub authorization_header: Option<String>,

    /// Branch filter for the hook.
    pub branch_filter: Option<String>,

    /// Configuration for the hook.
    pub config: HashMap<String, String>,

    /// Events that trigger the hook.
    pub events: Vec<String>,

    /// Type of the hook.
    #[serde(rename = "type")]
    pub kind: String,
}
