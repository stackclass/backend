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

use std::sync::Arc;
use tokio::process::Command;
use tracing::debug;

use crate::{
    config::Config,
    context::Context,
    errors::ApiError,
    service::{StorageError, StorageService},
};

#[allow(dead_code)]
pub struct RepoService {
    ctx: Arc<Context>,
}

impl RepoService {
    pub fn new(ctx: Arc<Context>) -> Self {
        RepoService { ctx }
    }

    pub async fn create(&self, template_url: &str, repo_name: &str) -> Result<(), StorageError> {
        debug!("Creating repo {} from template {}", repo_name, template_url);

        let Config { cache_dir, github_token, repo_dir, .. } = &self.ctx.config;

        // Fetch and validate the template directory
        let storage = StorageService::new(cache_dir, github_token)?;
        let dir = storage.fetch(template_url).await?;
        let template_dir = cache_dir.join(dir).join("template");
        if !template_dir.exists() {
            return Err(StorageError::MissingTemplate);
        }

        // Create a temporary directory for staging
        let temp_dir = tempfile::tempdir().map_err(StorageError::CreateDir)?;

        // Copy template contents to the temporary directory
        let copy_options = fs_extra::dir::CopyOptions::new().content_only(true).copy_inside(true);
        fs_extra::dir::copy(&template_dir, temp_dir.path(), &copy_options)
            .map_err(|e| StorageError::CopyFiles(e.to_string()))?;

        // Initialize Git repository
        let init_output = Command::new("git")
            .arg("init")
            .current_dir(temp_dir.path())
            .output()
            .await
            .map_err(|e| StorageError::GitCommand(e.to_string()))?;

        if !init_output.status.success() {
            return Err(StorageError::GitCommand(format!(
                "git init failed: {}",
                String::from_utf8_lossy(&init_output.stderr)
            )));
        }

        // Stage all files
        let add_output = Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir(temp_dir.path())
            .output()
            .await
            .map_err(|e| StorageError::GitCommand(e.to_string()))?;

        if !add_output.status.success() {
            return Err(StorageError::GitCommand(format!(
                "git add failed: {}",
                String::from_utf8_lossy(&add_output.stderr)
            )));
        }

        // Commit the staged files
        let commit_output = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg("Initial commit from template")
            .current_dir(temp_dir.path())
            .output()
            .await
            .map_err(|e| StorageError::GitCommand(e.to_string()))?;

        if !commit_output.status.success() {
            return Err(StorageError::GitCommand(format!(
                "git commit failed: {}",
                String::from_utf8_lossy(&commit_output.stderr)
            )));
        }

        // Clone as a bare repository to the target path
        let bare_repo_path = repo_dir.join(repo_name);
        let clone_output = Command::new("git")
            .arg("clone")
            .arg("--bare")
            .arg(temp_dir.path().to_str().unwrap())
            .arg(&bare_repo_path)
            .output()
            .await
            .map_err(|e| StorageError::GitCommand(e.to_string()))?;

        if !clone_output.status.success() {
            return Err(StorageError::GitCommand(format!(
                "git clone --bare failed: {}",
                String::from_utf8_lossy(&clone_output.stderr)
            )));
        }

        debug!("Repo {} created as bare repository from template", repo_name);
        Ok(())
    }

    pub async fn handle_push_event(&self, repo: &str) -> Result<(), ApiError> {
        debug!("Handling push event for repository: {}", repo);

        Ok(())
    }
}
