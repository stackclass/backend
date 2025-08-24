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

use fs_extra::dir::CopyOptions;
use gitea_client::{ClientError, types::*};
use tracing::{debug, info};
use uuid::Uuid;

use crate::{
    config::Config,
    context::Context,
    errors::Result,
    repository::CourseRepository,
    service::{CourseService, PipelineService, StageService, StorageError, StorageService},
    utils::{git, url},
};

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
    pub async fn init(&self, template: &str, template_url: &str) -> Result<()> {
        let org = &self.ctx.config.namespace;

        // Fetch or create the template repository in SCM
        self.fetch_template(org, template).await?;

        // Commits the template source code to the template repository
        self.commit(template_url, org, template).await?;

        Ok(())
    }

    /// Commits the template source code to a specified repository.
    async fn commit(&self, template_url: &str, owner: &str, repo: &str) -> Result<()> {
        // Fetch and validate the template directory
        let Config { cache_dir, github_token, .. } = &self.ctx.config;
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
        let copy_options = CopyOptions::new().content_only(true).copy_inside(true);
        fs_extra::dir::copy(&template_dir, workspace, &copy_options)
            .map_err(|e| StorageError::CopyFiles(e.to_string()))?;

        // Initialize Git repository with the 'main' branch
        git::init(workspace, "main").await?;

        // Configure Git user information
        git::config(workspace, "user.name", &self.ctx.config.git_committer_name).await?;
        git::config(workspace, "user.email", &self.ctx.config.git_committer_email).await?;

        // Perform Git operations to commit the source code
        git::stage(workspace).await?;
        git::commit(workspace, "Initial commit from template").await?;

        // ... and push to the remote repository
        let endpoint = &self.ctx.config.git_server_endpoint;
        let base_url = format!("{endpoint}/{owner}/{repo}.git");
        let remote_url = url::authenticate(
            &base_url,
            &self.ctx.config.git_server_username,
            &self.ctx.config.git_server_password,
        )?;
        git::add_remote(workspace, "origin", &remote_url).await?;
        git::push(workspace, "origin", "main").await?;

        info!("Successfully pushed template contents to repository: {}", base_url);
        Ok(())
    }

    /// Processes a repository push event by managing the associated course workflow.
    ///
    /// This function handles the following logic:
    /// - If the course has no active stage, it activates the course.
    /// - Otherwise, it triggers the pipeline for the current stage and monitors completion.
    /// - On success, marks the stage as complete.
    pub async fn process(&self, event: &Event) -> Result<()> {
        let repo = &event.repository.name;
        debug!("Handling push event for repository: {}", repo);

        let id = Uuid::parse_str(repo)?;
        let mut course = CourseRepository::get_user_course_by_id(&self.ctx.database, &id).await?;

        // If there's no current stage, this is the first setup of the course,
        // so we just need to activate it without running any pipeline stages
        let Some(current_stage_slug) = course.current_stage_slug else {
            CourseService::activate(self.ctx.clone(), &mut course).await?;
            return Ok(());
        };

        // Trigger the pipeline run
        let pipeline = PipelineService::new(self.ctx.clone());
        let name = pipeline.trigger(repo, &course.course_slug, &current_stage_slug).await?;

        // Define a callback function that will be executed when the pipeline
        // succeeds. This callback marks the current stage as complete in DB.
        let ctx = self.ctx.clone();
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
        pipeline.watch(&name, callback).await?;

        Ok(())
    }

    /// Gets a organization by name,
    /// or creates the organization if it doesn't exist.
    pub async fn fetch_organization(&self, name: &str) -> Result<Organization> {
        let organization = match self.ctx.git.get_organization(name).await {
            Ok(organization) => organization,
            Err(ClientError::NotFound) => {
                let req = CreateOrganizationRequest::new(name);
                let organization = self.ctx.git.create_organization(req).await?;

                info!("Successfully created organization: {name}");
                organization
            }
            Err(e) => return Err(e.into()),
        };

        Ok(organization)
    }

    /// Gets a template repository by name,
    /// or creates the repository if it doesn't exist.
    async fn fetch_template(&self, org: &str, repo: &str) -> Result<Repository> {
        let repository = match self.ctx.git.get_repository(org, repo).await {
            Ok(repository) => repository,
            Err(ClientError::NotFound) => {
                let req = CreateRepositoryRequest {
                    name: repo.to_string(),
                    template: Some(true),
                    ..Default::default()
                };
                self.ctx.git.create_org_repository(org, req).await?
            }
            Err(e) => return Err(e.into()),
        };

        info!("Successfully created template repository: {org}/{repo}");
        Ok(repository)
    }

    /// Generates a new repository from a template if it doesn't exist.
    pub async fn generate(&self, template: &str, repo: &str) -> Result<Repository> {
        let org = &self.ctx.config.namespace;

        let repository = match self.ctx.git.get_repository(org, repo).await {
            Ok(repository) => repository,
            Err(ClientError::NotFound) => {
                let req = GenerateRepositoryRequest {
                    git_content: Some(true),
                    git_hooks: Some(true),
                    name: repo.to_string(),
                    owner: org.to_string(),
                    webhooks: Some(true),
                    ..Default::default()
                };
                self.ctx.git.generate_repository(org, template, req).await?
            }
            Err(e) => return Err(e.into()),
        };

        info!("Successfully generated new repository: {org}/{repo}");
        Ok(repository)
    }

    /// Setup the webhook for the organization
    pub async fn setup_webhook(&self, org: &str) -> Result<()> {
        let webhook_endpoint = &self.ctx.config.webhook_endpoint;
        let url = format!("{webhook_endpoint}/v1/webhooks/gitea");

        // Define the webhook request body to listen for push events on the main branch
        // and send them to the specified webhook endpoint in JSON format.
        let req = CreateHookRequest {
            active: true,
            branch_filter: Some("main".to_string()),
            config: HashMap::from([
                ("content_type".to_string(), "json".to_string()),
                ("url".to_string(), url.clone()),
            ]),
            events: vec!["push".to_string()],
            kind: "gitea".to_string(),
            ..Default::default()
        };

        // List all existing hooks
        let hooks = self.ctx.git.list_org_hooks(org).await?;

        // Check if a hook with the same configuration already exists
        if !hooks.iter().any(|hook| matching(hook, &req)) {
            info!("Setting up the webhook for the organization {org}.");
            self.ctx.git.create_org_hook(org, req).await?;
        }

        Ok(())
    }
}
