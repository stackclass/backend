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
    client::HarborClient,
    error::{ClientError, Result},
    types::CreateProjectRequest,
};

impl HarborClient {
    /// Check if a project exists by name.
    ///
    /// # Possible Responses
    /// - 200: Project exists.
    /// - 404: Project not found.
    /// - 500: Internal server error.
    ///
    /// https://github.com/goharbor/harbor/blob/v2.13.1/api/v2.0/swagger.yaml#L323
    pub async fn head_project(&self, name: &str) -> Result<()> {
        let response = self.head(&format!("projects?project_name={}", name)).await?;

        match response.status() {
            StatusCode::OK => Ok(()),
            _ => Err(ClientError::from_response(response).await),
        }
    }

    /// Create a new project.
    ///
    /// # Possible Responses
    /// - 201: Project created successfully.
    /// - 400: Bad request (invalid input format).
    /// - 401: Unauthorized.
    /// - 409: Conflict (project already exists).
    /// - 500: Internal server error.
    ///
    /// https://github.com/goharbor/harbor/blob/v2.13.1/api/v2.0/swagger.yaml#L343
    pub async fn create_project(&self, request: CreateProjectRequest) -> Result<()> {
        let response = self.post("projects", &request).await?;

        match response.status() {
            StatusCode::CREATED => Ok(()),
            _ => Err(ClientError::from_response(response).await),
        }
    }
}
