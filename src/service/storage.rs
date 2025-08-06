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

use axum::body::{Body, to_bytes};
use flate2::read::GzDecoder;
use ghrepo::GHRepo;
use http_body_util::BodyExt;
use octocrab::{
    Octocrab,
    models::repos::Object,
    params::repos::{Commitish, Reference},
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tar::Archive;
use thiserror::Error;
use tokio::fs;
use tracing::{debug, info};

type Result<T, E = StorageError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Failed to create GitHub client")]
    GitHubClientCreation(#[source] octocrab::Error),

    #[error("Invalid repository URL")]
    InvalidRepoUrl(#[source] ghrepo::ParseError),

    #[error("Failed to fetch repository info")]
    FetchRepoInfo(#[source] octocrab::Error),

    #[error("Invalid reference type")]
    InvalidReferenceType,

    #[error("No default branch found")]
    NoDefaultBranch,

    #[error("Failed to download tarball")]
    DownloadTarball(#[source] octocrab::Error),

    #[error("Failed to read tarball")]
    ReadTarball(#[source] axum::Error),

    #[error("Failed to create directory")]
    CreateDir(#[source] std::io::Error),

    #[error("Failed to unpack tarball")]
    UnpackTarball(#[source] std::io::Error),

    #[error("Failed to copy files: {0}")]
    CopyFiles(String),

    #[error("Template directory is missing")]
    MissingTemplate,
}

// Service for downloading and caching GitHub repositories
pub struct StorageService {
    cache_dir: PathBuf,      // Base directory for storing cached repositories
    octocrab: Arc<Octocrab>, // GitHub API client
}

impl StorageService {
    // Creates new StorageService with optional GitHub token
    pub fn new(cache_dir: &Path, github_token: &Option<String>) -> Result<Self> {
        let octocrab = match github_token {
            Some(token) => Arc::new(
                Octocrab::builder()
                    .personal_token(token.as_str())
                    .build()
                    .map_err(StorageError::GitHubClientCreation)?,
            ),
            None => octocrab::instance(),
        };

        Ok(Self { cache_dir: cache_dir.to_path_buf(), octocrab })
    }

    /// Download and store GitHub repository,
    /// and return the path of the cached directory.
    pub async fn fetch(&self, url: &str) -> Result<PathBuf> {
        let repo = GHRepo::from_url(url).map_err(StorageError::InvalidRepoUrl)?;
        let api = self.octocrab.repos(repo.owner(), repo.name());

        info!("Fetching the repository info {}", repo);
        let repository = api.get().await.map_err(StorageError::FetchRepoInfo)?;

        let reference = match repository.default_branch {
            Some(branch) => {
                let reference = api
                    .get_ref(&Reference::Branch(branch.to_string()))
                    .await
                    .map_err(StorageError::FetchRepoInfo)?;
                match reference.object {
                    Object::Commit { sha, .. } => sha,
                    _ => return Err(StorageError::InvalidReferenceType),
                }
            }
            None => return Err(StorageError::NoDefaultBranch),
        };

        info!("Downloading repository {}", repo);
        let dir = self.download(repo.owner(), repo.name(), &reference).await?;

        Ok(dir)
    }

    // Downloads and extracts GitHub repository tarball to cache directory
    async fn download(&self, owner: &str, repo: &str, reference: &str) -> Result<PathBuf> {
        let dir = PathBuf::from(format!("{}-{}-{}", owner, repo, &reference[..7]));

        if self.cache_dir.join(&dir).exists() {
            info!("Repository {} (commit {}) already cached", repo, reference);
            return Ok(dir);
        }

        debug!("Downloading tarball for {}/{} (commit {})", owner, repo, reference);
        let tarball = self
            .octocrab
            .repos(owner, repo)
            .download_tarball(Commitish::from(reference.to_string()))
            .await
            .map_err(StorageError::DownloadTarball)?;

        debug!("Collecting tarball data...");
        let collected = tarball.collect().await.map_err(StorageError::DownloadTarball)?;
        let body = Body::from(collected.to_bytes());

        let bytes = to_bytes(body, usize::MAX).await.map_err(StorageError::ReadTarball)?;
        debug!("Tarball size: {} bytes", bytes.len());
        debug!("Tarball header (hex): {:02x?}", &bytes[..32.min(bytes.len())]);

        // Create caches directory if it doesn't exist
        fs::create_dir_all(&self.cache_dir).await.map_err(StorageError::CreateDir)?;

        // Unpack the tarball
        self.unarchive(&bytes)?;

        debug!("Successfully unpacked tarball to {:?}", dir);
        Ok(dir)
    }

    /// Unarchive the tarball data to the caches directory
    fn unarchive(&self, bytes: &[u8]) -> Result<()> {
        debug!("Unpacking tarball...");

        let tar = GzDecoder::new(bytes);
        let mut archive = Archive::new(tar);
        archive.unpack(&self.cache_dir).map_err(StorageError::UnpackTarball)?;

        Ok(())
    }
}
