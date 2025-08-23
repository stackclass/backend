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
    types::{CreateOrganizationRequest, CreateRepositoryRequest, Organization, Repository},
};

impl GiteaClient {
    /// Creates a new organization.
    ///
    /// # Possible Responses
    /// - 201: Organization created successfully (returns `Organization`).
    /// - 403: Forbidden (insufficient permissions).
    /// - 422: Validation error.
    ///
    /// https://docs.gitea.com/api/1.24/#tag/organization/operation/orgCreate
    pub async fn create_organization(
        &self,
        request: CreateOrganizationRequest,
    ) -> Result<Organization, ClientError> {
        let response = self.post("orgs", &request).await?;

        match response.status() {
            StatusCode::CREATED => Ok(response.json::<Organization>().await?),
            _ => Err(ClientError::from_response(response).await),
        }
    }

    /// Creates a new repository in an organization.
    ///
    /// # Possible Responses
    /// - 201: Repository created successfully (returns `Repository`).
    /// - 400: Bad request (invalid repository name or other validation error).
    /// - 403: Forbidden (not an organization member or insufficient permissions).
    /// - 404: Organization not found.
    ///
    /// https://docs.gitea.com/api/1.24/#tag/organization/operation/createOrgRepo
    pub async fn create_org_repository(
        &self,
        org: &str,
        request: CreateRepositoryRequest,
    ) -> Result<Repository, ClientError> {
        let endpoint = format!("orgs/{org}/repos");
        let response = self.post(&endpoint, &request).await?;

        match response.status() {
            StatusCode::CREATED => Ok(response.json::<Repository>().await?),
            _ => Err(ClientError::from_response(response).await),
        }
    }
}
