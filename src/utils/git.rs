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

use std::path::Path;
use thiserror::Error;
use tokio::process::Command;

#[derive(Debug, Error)]
pub enum GitError {
    #[error("Failed to initialize Git repository: {0}")]
    InitRepo(String),

    #[error("Failed to stage files: {0}")]
    StageFiles(String),

    #[error("Failed to commit changes: {0}")]
    CommitChanges(String),

    #[error("Failed to add remote: {0}")]
    AddRemote(String),

    #[error("Failed to push changes: {0}")]
    PushChanges(String),
}

/// Initializes a new Git repository in the specified directory.
#[inline]
pub async fn init(dir: &Path) -> Result<(), GitError> {
    git(dir, &["init"]).await.map_err(GitError::InitRepo)
}

/// Stages all files in the working directory.
#[inline]
pub async fn stage(dir: &Path) -> Result<(), GitError> {
    git(dir, &["add", "."]).await.map_err(GitError::StageFiles)
}

/// Commits staged files with the given message.
#[inline]
pub async fn commit(dir: &Path, message: &str) -> Result<(), GitError> {
    git(dir, &["commit", "-m", message]).await.map_err(GitError::CommitChanges)
}

/// Adds a remote repository.
#[inline]
pub async fn add_remote(dir: &Path, remote_name: &str, remote_url: &str) -> Result<(), GitError> {
    git(dir, &["remote", "add", remote_name, remote_url]).await.map_err(GitError::AddRemote)
}

/// Pushes changes to a remote repository.
#[inline]
pub async fn push(dir: &Path, remote_name: &str, branch: &str) -> Result<(), GitError> {
    git(dir, &["push", "--force", remote_name, branch]).await.map_err(GitError::PushChanges)
}

/// Executes a Git command and returns a raw error message if failed.
async fn git(dir: &Path, args: &[&str]) -> Result<(), String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(())
}
