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
    types::{CreateRepositoryRequest, Repository},
};

impl GiteaClient {
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
}
