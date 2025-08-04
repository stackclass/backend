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

use super::{Team, User};

/// Request body for creating a repository.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRepositoryRequest {
    /// Whether the repository should be auto-initialized?
    pub auto_init: Option<bool>,

    /// The default branch name for the repository
    /// (used when initializes and in template)
    pub default_branch: Option<String>,

    /// A description of the repository
    pub description: Option<String>,

    /// The template gitignore file to use
    pub gitignores: Option<String>,

    /// The template issue labels to use
    pub issue_labels: Option<String>,

    /// The template license to use
    pub license: Option<String>,

    /// The name of the repository
    pub name: String,

    /// The object format name (e.g. "sha1" or "sha256")
    pub object_format_name: Option<String>,

    /// Whether the repository should be private
    pub private: Option<bool>,

    /// The template README file to use
    pub readme: Option<String>,

    /// Whether the repository is template
    pub template: Option<bool>,

    /// The trust model for the repository
    /// options: "default", "collaborator", "committer", "collaboratorcommitter"
    pub trust_model: Option<String>,
}

/// Repository data type representing a Gitea repository.
#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    /// Whether fast-forward-only merges are allowed.
    pub allow_fast_forward_only_merge: bool,

    /// Whether merge commits are allowed.
    pub allow_merge_commits: bool,

    /// Whether rebase merges are allowed.
    pub allow_rebase: bool,

    /// Whether explicit rebase merges are allowed.
    pub allow_rebase_explicit: bool,

    /// Whether rebase updates are allowed.
    pub allow_rebase_update: bool,

    /// Whether squash merges are allowed.
    pub allow_squash_merge: bool,

    /// Whether the repository is archived.
    pub archived: bool,

    /// Timestamp when the repository was archived.
    pub archived_at: Option<String>,

    /// URL to the repository's avatar.
    pub avatar_url: String,

    /// URL to clone the repository.
    pub clone_url: String,

    /// Timestamp when the repository was created.
    pub created_at: String,

    /// Whether maintainers are allowed to edit the repository by default.
    pub default_allow_maintainer_edit: bool,

    /// Default branch of the repository.
    pub default_branch: String,

    /// Whether branches are deleted after merging by default.
    pub default_delete_branch_after_merge: bool,

    /// Default merge style for the repository.
    pub default_merge_style: String,

    /// Description of the repository.
    pub description: String,

    /// Whether the repository is empty.
    pub empty: bool,

    /// External tracker configuration.
    pub external_tracker: ExternalTracker,

    /// External wiki configuration.
    pub external_wiki: ExternalWiki,

    /// Whether the repository is a fork.
    pub fork: bool,

    /// Number of forks of the repository.
    pub forks_count: u64,

    /// Full name of the repository (e.g., "owner/repo").
    pub full_name: String,

    /// Whether the repository has GitHub Actions enabled.
    pub has_actions: bool,

    /// Whether the repository has issues enabled.
    pub has_issues: bool,

    /// Whether the repository has packages enabled.
    pub has_packages: bool,

    /// Whether the repository has projects enabled.
    pub has_projects: bool,

    /// Whether the repository has pull requests enabled.
    pub has_pull_requests: bool,

    /// Whether the repository has releases enabled.
    pub has_releases: bool,

    /// Whether the repository has a wiki enabled.
    pub has_wiki: bool,

    /// URL to the repository's HTML page.
    pub html_url: String,

    /// Unique identifier for the repository.
    pub id: u64,

    /// Whether whitespace conflicts are ignored.
    pub ignore_whitespace_conflicts: bool,

    /// Whether the repository is internal.
    pub internal: bool,

    /// Internal tracker configuration.
    pub internal_tracker: InternalTracker,

    /// Primary language of the repository.
    pub language: String,

    /// URL to the repository's languages data.
    pub languages_url: String,

    /// List of licenses associated with the repository.
    pub licenses: Vec<String>,

    /// Link to the repository.
    pub link: String,

    /// Whether the repository is a mirror.
    pub mirror: bool,

    /// Interval for mirror updates.
    pub mirror_interval: String,

    /// Timestamp when the mirror was last updated.
    pub mirror_updated: String,

    /// Name of the repository.
    pub name: String,

    /// Object format name (e.g., "sha1" or "sha256").
    pub object_format_name: String,

    /// Number of open issues in the repository.
    pub open_issues_count: u64,

    /// Number of open pull requests in the repository.
    pub open_pr_counter: u64,

    /// Original URL of the repository (for mirrors).
    pub original_url: String,

    /// Owner of the repository.
    pub owner: User,

    /// Parent repository (if this is a fork).
    pub parent: Option<Box<Repository>>,

    /// Permissions for the repository.
    pub permissions: Permissions,

    /// Whether the repository is private.
    pub private: bool,

    /// Projects mode for the repository.
    pub projects_mode: String,

    /// Number of releases in the repository.
    pub release_counter: u64,

    /// Repository transfer details.
    pub repo_transfer: Option<RepoTransfer>,

    /// Size of the repository in bytes.
    pub size: u64,

    /// SSH URL to clone the repository.
    pub ssh_url: String,

    /// Number of stars on the repository.
    pub stars_count: u64,

    /// Whether the repository is a template.
    pub template: bool,

    /// List of topics associated with the repository.
    pub topics: Vec<String>,

    /// Timestamp when the repository was last updated.
    pub updated_at: String,

    /// URL of the repository.
    pub url: String,

    /// Number of watchers of the repository.
    pub watchers_count: u64,

    /// Website URL of the repository.
    pub website: String,
}

/// External tracker configuration for a repository.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalTracker {
    /// Format for external tracker references.
    pub external_tracker_format: String,

    /// Regex pattern for external tracker references.
    pub external_tracker_regexp_pattern: String,

    /// Style for external tracker references.
    pub external_tracker_style: String,

    /// URL for the external tracker.
    pub external_tracker_url: String,
}

/// External wiki configuration for a repository.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalWiki {
    /// URL for the external wiki.
    pub external_wiki_url: String,
}

/// Internal tracker configuration for a repository.
#[derive(Debug, Serialize, Deserialize)]
pub struct InternalTracker {
    /// Whether only contributors can track time.
    pub allow_only_contributors_to_track_time: bool,

    /// Whether issue dependencies are enabled.
    pub enable_issue_dependencies: bool,

    /// Whether time tracking is enabled.
    pub enable_time_tracker: bool,
}

/// Permissions for a repository.
#[derive(Debug, Serialize, Deserialize)]
pub struct Permissions {
    /// Whether the user has admin permissions.
    pub admin: bool,

    /// Whether the user has pull permissions.
    pub pull: bool,

    /// Whether the user has push permissions.
    pub push: bool,
}

/// Repository transfer details.
#[derive(Debug, Serialize, Deserialize)]
pub struct RepoTransfer {
    /// User who initiated the transfer.
    pub doer: User,

    /// Recipient of the transfer.
    pub recipient: User,

    /// Teams involved in the transfer.
    pub teams: Vec<Team>,
}
