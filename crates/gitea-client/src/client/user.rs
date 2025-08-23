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
    error::{ClientError, Result},
    types::{CreateUserRequest, User},
};

impl GiteaClient {
    /// Get a user by username.
    ///
    /// # Possible Responses
    /// - 201: User created successfully (returns `User`).
    /// - 404: User not found.
    ///
    /// https://docs.gitea.com/api/1.24/#tag/user/operation/userGet
    pub async fn get_user(&self, username: &str) -> Result<User> {
        let response = self.get(&format!("users/{username}")).await?;

        match response.status() {
            StatusCode::OK => Ok(response.json::<User>().await?),
            _ => Err(ClientError::from_response(response).await),
        }
    }

    /// Creates a new user.
    ///
    /// # Possible Responses
    /// - 201: User created successfully (returns `User`).
    /// - 400: Bad request (invalid input format).
    /// - 403: Forbidden (not an admin).
    /// - 422: Input validation failed.
    ///
    /// https://docs.gitea.com/api/1.24/#tag/admin/operation/adminCreateUser
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User> {
        let response = self.post("admin/users", &request).await?;

        match response.status() {
            StatusCode::CREATED => Ok(response.json::<User>().await?),
            _ => Err(ClientError::from_response(response).await),
        }
    }
}
