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

use reqwest::StatusCode;

use crate::{
    client::GiteaClient,
    error::ClientError,
    types::{CreateRepositoryRequest, GenerateRepositoryRequest, Repository},
};

impl GiteaClient {
    /// Gets a repository by owner and repository name.
    ///
    /// # Possible Responses
    /// - 200: Repository found (returns `Repository`).
    /// - 404: Repository not found.
    ///
    /// https://docs.gitea.com/api/1.24/#tag/repository/operation/repoGet
    pub async fn get_repository(&self, owner: &str, repo: &str) -> Result<Repository, ClientError> {
        let endpoint = format!("repos/{owner}/{repo}");
        let response = self.get(&endpoint).await?;

        match response.status() {
            StatusCode::OK => Ok(response.json::<Repository>().await?),
            _ => Err(ClientError::from_response(response).await),
        }
    }

    /// Creates a new repository on behalf of a user (admin endpoint).
    ///
    /// # Possible Responses
    /// - 201: Repository created successfully (returns `Repository`).
    /// - 400: Bad request (invalid input format).
    /// - 403: Forbidden (not an admin).
    /// - 404: User not found.
    /// - 409: Repository with the same name already exists.
    /// - 422: Input validation failed.
    ///
    /// https://docs.gitea.com/api/1.24/#tag/repository/operation/adminCreateRepo
    pub async fn create_repository_for_user(
        &self,
        username: &str,
        request: CreateRepositoryRequest,
    ) -> Result<Repository, ClientError> {
        let endpoint = format!("admin/users/{username}/repos");
        let response = self.post(&endpoint, &request).await?;

        match response.status() {
            StatusCode::CREATED => Ok(response.json::<Repository>().await?),
            _ => Err(ClientError::from_response(response).await),
        }
    }

    /// Creates a new repository using a template.
    ///
    /// # Possible Responses
    /// - 201: Repository created successfully (returns `Repository`).
    /// - 403: Forbidden (not an admin or insufficient permissions).
    /// - 404: Template repository not found.
    /// - 409: Repository with the same name already exists.
    /// - 422: Input validation failed.
    ///
    /// https://docs.gitea.com/api/1.24/#tag/repository/operation/generateRepo
    pub async fn generate_repository(
        &self,
        template_owner: &str,
        template_repo: &str,
        request: GenerateRepositoryRequest,
    ) -> Result<Repository, ClientError> {
        let endpoint = format!("repos/{template_owner}/{template_repo}/generate");
        let response = self.post(&endpoint, &request).await?;

        match response.status() {
            StatusCode::CREATED => Ok(response.json::<Repository>().await?),
            _ => Err(ClientError::from_response(response).await),
        }
    }
}
