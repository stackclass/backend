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

use std::{collections::HashMap, str::FromStr, sync::Arc};

use gitea_client::{ClientError, types::*};
use tokio::process::Command;
use tracing::debug;
use uuid::Uuid;

use crate::{
    config::Config,
    constant::TEMPLATE_OWNER,
    context::Context,
    errors::{ApiError, Result},
    model::UserModel,
    repository::CourseRepository,
    service::{CourseService, StageService, StorageError, StorageService},
};

#[allow(dead_code)]
pub struct RepoService {
    ctx: Arc<Context>,
}

impl RepoService {
    pub fn new(ctx: Arc<Context>) -> Self {
        RepoService { ctx }
    }

    /// Initializes a template repository in the Source Code Management system
    /// for this course. The repository will contain the course's template
    /// source code.
    pub async fn init(&self, template: &str, repository: &str) -> Result<()> {
        // Fetch or create the template repository in SCM
        self.fetch_template(template).await?;
        debug!("Successfully created template repository in SCM: {:?}", template);

        // Commit the course's template source code to the template repository
        self.commit(repository, &format!("{TEMPLATE_OWNER}/{template}")).await?;

        Ok(())
    }

    async fn commit(&self, template_url: &str, repo_name: &str) -> Result<(), StorageError> {
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

        let mut user_course = CourseRepository::get_user_course_by_id(
            &self.ctx.database,
            Uuid::from_str(repo).unwrap(),
        )
        .await?;

        if let Some(stage_slug) = &user_course.current_stage_slug {
            StageService::complete(
                self.ctx.clone(),
                &user_course.user_id,
                &user_course.course_slug,
                stage_slug,
            )
            .await?;
        } else {
            CourseService::activate(self.ctx.clone(), &mut user_course).await?
        };

        Ok(())
    }

    /// Gets a template repository by name,
    /// or creates the repository if it doesn't exist.
    async fn fetch_template(&self, repo: &str) -> Result<Repository> {
        self.fetch_user(
            TEMPLATE_OWNER,
            CreateUserRequest {
                email: self.ctx.config.email.clone(),
                username: TEMPLATE_OWNER.to_string(),
                ..Default::default()
            },
        )
        .await?;

        match self.ctx.git.get_repository(TEMPLATE_OWNER, repo).await {
            Ok(repo) => Ok(repo),
            Err(ClientError::NotFound) => {
                let req = CreateRepositoryRequest {
                    name: repo.to_string(),
                    template: Some(true),
                    ..Default::default()
                };
                Ok(self.ctx.git.create_repository_for_user(TEMPLATE_OWNER, req).await?)
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Gets a repository by username and template name,
    /// or generates a new repository from a template if it doesn't exist.
    pub async fn generate(&self, user: &UserModel, template: &str) -> Result<Repository> {
        // @TODO: Currently using display name temporarily,
        // will use GitHub username in the future.
        let username = user.name.to_ascii_lowercase().replace(" ", "");

        self.fetch_user(
            &username,
            CreateUserRequest {
                email: user.email.clone(),
                username: username.clone(),
                ..Default::default()
            },
        )
        .await?;

        match self.ctx.git.get_repository(&username, template).await {
            Ok(repo) => Ok(repo),
            Err(ClientError::NotFound) => {
                let req = GenerateRepositoryRequest {
                    git_content: Some(true),
                    git_hooks: Some(true),
                    name: template.to_string(),
                    owner: username.to_string(),
                    webhooks: Some(true),
                    ..Default::default()
                };
                Ok(self.ctx.git.generate_repository(TEMPLATE_OWNER, template, req).await?)
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Creates a new admin hook.
    pub async fn setup_hook(&self) -> Result<Hook> {
        let url = format!("{}/v1/webhooks/gitea", self.ctx.config.webhook_endpoint);
        let req = CreateHookRequest {
            active: true,
            branch_filter: Some("main".to_string()),
            config: HashMap::from([
                ("content_type".to_string(), "application/json".to_string()),
                ("url".to_string(), url),
            ]),
            events: vec!["push".to_string()],
            kind: "gitea".to_string(),
            ..Default::default()
        };

        Ok(self.ctx.git.create_admin_hook(req).await?)
    }

    /// Gets a user by username, or creates the user if they don't exist.
    async fn fetch_user(&self, username: &str, req: CreateUserRequest) -> Result<User> {
        match self.ctx.git.get_user(username).await {
            Ok(user) => Ok(user),
            Err(ClientError::NotFound) => Ok(self.ctx.git.create_user(req).await?),
            Err(e) => Err(e.into()),
        }
    }
}
