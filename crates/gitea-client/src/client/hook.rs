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
    types::{CreateHookRequest, Hook},
};

impl GiteaClient {
    /// Creates a new hook (admin endpoint).
    ///
    /// # Possible Responses
    /// - 201: Hook created successfully (returns `Hook`).
    ///
    /// https://docs.gitea.com/api/1.24/#tag/admin/operation/adminCreateHook
    #[inline]
    pub async fn create_admin_hook(&self, request: CreateHookRequest) -> Result<Hook, ClientError> {
        self.create_hook("admin/hooks", request).await
    }

    /// Lists all system hooks (admin endpoint).
    ///
    /// # Possible Responses
    /// - 200: List of hooks returned successfully (returns `Vec<Hook>`).
    ///
    /// https://docs.gitea.com/api/1.24/#tag/admin/operation/adminListHooks
    #[inline]
    pub async fn list_system_hooks(&self) -> Result<Vec<Hook>, ClientError> {
        self.list_hooks("admin/hooks").await
    }

    /// Creates a new hook at the specified path.
    ///
    /// This is an internal helper function used by both admin and repository hook creation.
    ///
    /// # Arguments
    /// * `path` - The API endpoint path (e.g. "admin/hooks" or "repos/{owner}/{repo}/hooks")
    /// * `req` - The hook creation request payload
    ///
    /// # Possible Responses
    /// - 201: Hook created successfully (returns `Hook`)
    /// - Other: Returns appropriate `ClientError`
    ///
    /// # Notes
    /// This is not meant to be called directly - use the appropriate public method instead.
    async fn create_hook(&self, path: &str, req: CreateHookRequest) -> Result<Hook, ClientError> {
        let response = self.post(path, &req).await?;

        match response.status() {
            StatusCode::CREATED => Ok(response.json::<Hook>().await?),
            _ => Err(ClientError::from_response(response).await),
        }
    }

    /// Lists hooks at the specified path.
    ///
    /// This is an internal helper function used by both admin and repository hook listing.
    ///
    /// # Arguments
    /// * `path` - The API endpoint path (e.g. "admin/hooks" or "repos/{owner}/{repo}/hooks")
    ///
    /// # Possible Responses
    /// - 200: List of hooks returned successfully (returns `Vec<Hook>`)
    /// - Other: Returns appropriate `ClientError`
    ///
    /// # Notes
    /// This is not meant to be called directly - use the appropriate public method instead.
    async fn list_hooks(&self, path: &str) -> Result<Vec<Hook>, ClientError> {
        let response = self.get(path).await?;

        match response.status() {
            StatusCode::OK => Ok(response.json::<Vec<Hook>>().await?),
            _ => Err(ClientError::from_response(response).await),
        }
    }
}
