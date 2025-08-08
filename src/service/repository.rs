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

use std::{collections::HashMap, sync::Arc};

use gitea_client::{ClientError, types::*};
use tracing::{debug, info};

use crate::{
    config::Config,
    constant::TEMPLATE_OWNER,
    context::Context,
    errors::Result,
    model::UserModel,
    repository::{CourseRepository, UserRepository},
    service::{CourseService, PipelineService, StageService, StorageError, StorageService},
    utils::git,
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

        // Commits the template source code to the template repository
        self.commit(repository, TEMPLATE_OWNER, template).await?;

        Ok(())
    }

    /// Commits the template source code to a specified repository.
    async fn commit(&self, template_url: &str, owner: &str, repo: &str) -> Result<()> {
        let Config {
            cache_dir,
            github_token,
            git_server_endpoint,
            git_committer_name,
            git_committer_email,
            ..
        } = &self.ctx.config;

        // Fetch and validate the template directory
        let storage = StorageService::new(cache_dir, github_token)?;
        let dir = storage.fetch(template_url).await?;
        let template_dir = cache_dir.join(dir).join("template");
        if !template_dir.exists() {
            return Err(StorageError::MissingTemplate.into());
        }

        // Create a temporary directory for staging
        let temp_dir = tempfile::tempdir().map_err(StorageError::CreateDir)?;
        let workspace = temp_dir.path();

        // Copy template contents to the workspace directory
        let copy_options = fs_extra::dir::CopyOptions::new().content_only(true).copy_inside(true);
        fs_extra::dir::copy(&template_dir, workspace, &copy_options)
            .map_err(|e| StorageError::CopyFiles(e.to_string()))?;

        // Configure Git user information
        git::config(workspace, "user.name", git_committer_name).await?;
        git::config(workspace, "user.email", git_committer_email).await?;

        // Perform Git operations to commit the source code
        git::init(workspace).await?;
        git::stage(workspace).await?;
        git::commit(workspace, "Initial commit from template").await?;

        // ... and push to the remote repository
        let remote_url = format!("{git_server_endpoint}/{owner}/{repo}");
        git::add_remote(workspace, "origin", &remote_url).await?;
        git::push(workspace, "origin", "main").await?;

        debug!("Successfully pushed template contents to repository: {}", remote_url);
        Ok(())
    }

    /// Processes a repository push event by managing the associated course workflow.
    ///
    /// This function handles the following logic:
    /// - If the course has no active stage, it activates the course.
    /// - Otherwise, it triggers the pipeline for the current stage and monitors completion.
    /// - On success, marks the stage as complete.
    pub async fn process(&self, event: &Event) -> Result<()> {
        let PartialRepository { owner, name, .. } = &event.repository;
        debug!("Handling push event for repository: {}", name);

        let ctx = self.ctx.clone();
        let user = UserRepository::get_by_email(&ctx.database, &owner.email).await?;
        let mut course = CourseRepository::get_user_course(&ctx.database, &user.id, name).await?;

        // If there's no current stage, this is the first setup of the course,
        // so we just need to activate it without running any pipeline stages
        let Some(current_stage_slug) = course.current_stage_slug else {
            CourseService::activate(ctx, &mut course).await?;
            return Ok(());
        };

        let pipeline = PipelineService::new(ctx.clone());

        // Trigger the pipeline run
        pipeline.trigger(&owner.username, name).await?;

        // Define a callback function that will be executed when the pipeline
        // succeeds. This callback marks the current stage as complete in DB.
        #[rustfmt::skip]
        let callback = || async move {
            StageService::complete(
                ctx,
                &course.user_id,
                &course.course_slug,
                &current_stage_slug,
            )
            .await
        };

        // Watch the pipeline and handle only success
        pipeline.watch(&owner.username, name, callback).await?;

        Ok(())
    }

    /// Gets a template repository by name,
    /// or creates the repository if it doesn't exist.
    async fn fetch_template(&self, repo: &str) -> Result<Repository> {
        let Config { git_committer_email, .. } = &self.ctx.config;

        self.fetch_user(
            TEMPLATE_OWNER,
            CreateUserRequest {
                email: git_committer_email.clone(),
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
        let username = user.username();
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

    /// Creates a new admin hook if it doesn't already exist.
    pub async fn setup_webhook(&self) -> Result<()> {
        let url = format!("{}/v1/webhooks/gitea", self.ctx.config.webhook_endpoint);
        let req = CreateHookRequest {
            active: true,
            branch_filter: Some("main".to_string()),
            config: HashMap::from([
                ("content_type".to_string(), "application/json".to_string()),
                ("url".to_string(), url.clone()),
            ]),
            events: vec!["push".to_string()],
            kind: "gitea".to_string(),
            ..Default::default()
        };

        // List all existing hooks
        let hooks = self.ctx.git.list_system_hooks().await?;

        // Check if a hook with the same configuration already exists
        if !hooks.iter().any(|hook| matching(hook, &req)) {
            info!("Setting up the global webhook in SCM...");
            self.ctx.git.create_admin_hook(req).await?;
        }

        Ok(())
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
