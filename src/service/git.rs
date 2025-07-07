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

use crate::errors::{ApiError, Result};

use std::path::{Path, PathBuf};
use tokio::{fs, io::AsyncWriteExt, process::Command};
use tracing::debug;

pub struct GitService {
    root: PathBuf,
}

impl GitService {
    pub fn new(root: &Path) -> Self {
        GitService { root: root.to_path_buf() }
    }

    pub async fn exists(&self, repo: &str) -> bool {
        fs::metadata(&self.root.join(repo)).await.is_ok()
    }

    pub async fn get_info_refs(&self, repo: &str, service: &str) -> Result<Vec<u8>> {
        debug!("GitService::get_info_refs({}, {})", repo, service);

        let output = Command::new("git")
            .arg(service)
            .arg("--stateless-rpc")
            .arg("--advertise-refs")
            .arg(self.root.join(repo))
            .output()
            .await
            .map_err(|e| ApiError::GitCommandFailed(e.to_string()))?;

        if !output.status.success() {
            return Err(ApiError::GitCommandFailed(format!(
                "git {} failed with status: {}",
                service, output.status
            )));
        }

        Ok(output.stdout)
    }

    pub async fn upload_pack(&self, repo: &str, body: Vec<u8>) -> Result<Vec<u8>> {
        let mut child = Command::new("git")
            .arg("upload-pack")
            .arg("--stateless-rpc")
            .arg(self.root.join(repo))
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| ApiError::GitCommandFailed(e.to_string()))?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(&body).await.map_err(|e| ApiError::GitCommandFailed(e.to_string()))?;
        }

        let output = child
            .wait_with_output()
            .await
            .map_err(|e| ApiError::GitCommandFailed(e.to_string()))?;

        if !output.status.success() {
            return Err(ApiError::GitCommandFailed(format!(
                "git upload-pack failed with status: {}",
                output.status
            )));
        }

        Ok(output.stdout)
    }

    pub async fn receive_pack(&self, repo: &str, body: Vec<u8>) -> Result<Vec<u8>> {
        let mut child = Command::new("git")
            .arg("receive-pack")
            .arg("--stateless-rpc")
            .arg(self.root.join(repo))
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| ApiError::GitCommandFailed(e.to_string()))?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(&body).await.map_err(|e| ApiError::GitCommandFailed(e.to_string()))?;
        }

        let output = child
            .wait_with_output()
            .await
            .map_err(|e| ApiError::GitCommandFailed(e.to_string()))?;

        if !output.status.success() {
            return Err(ApiError::GitCommandFailed(format!(
                "git receive-pack failed with status: {}",
                output.status
            )));
        }

        Ok(output.stdout)
    }

    pub async fn get_text_file(&self, repo: &str, path: &str) -> Result<Vec<u8>> {
        let full_path = self.root.join(repo).join(path);
        fs::read(&full_path).await.map_err(|_| ApiError::NotFound)
    }

    pub async fn get_info_packs(&self, repo: &str) -> Result<Vec<u8>> {
        self.get_text_file(repo, "objects/info/packs").await
    }

    pub async fn get_loose_object(&self, repo: &str, path: &str) -> Result<Vec<u8>> {
        self.get_text_file(repo, path).await
    }

    pub async fn get_pack_file(&self, repo: &str, path: &str) -> Result<Vec<u8>> {
        self.get_text_file(repo, path).await
    }
}
