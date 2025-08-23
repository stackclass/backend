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

pub mod project;

use reqwest::{Client, Error, Response};
use serde::Serialize;

/// A client for interacting with the Harbor API.
pub struct HarborClient {
    pub(crate) client: Client,
    pub(crate) base_url: String,
    pub(crate) username: String,
    pub(crate) password: String,
}

impl HarborClient {
    /// Creates a new `HarborClient` instance.
    pub fn new(endpoint: String, username: String, password: String) -> Self {
        HarborClient {
            client: Client::new(),
            base_url: format!("{endpoint}/api/v2.0"),
            username,
            password,
        }
    }

    /// Sends a GET request.
    #[allow(dead_code)]
    pub(crate) async fn get(&self, path: &str) -> Result<Response, Error> {
        let url = format!("{}/{}", self.base_url, path);
        self.client.get(&url).basic_auth(&self.username, Some(&self.password)).send().await
    }

    /// Sends a HEAD request.
    #[allow(dead_code)]
    pub(crate) async fn head(&self, path: &str) -> Result<Response, Error> {
        let url = format!("{}/{}", self.base_url, path);
        self.client.head(&url).basic_auth(&self.username, Some(&self.password)).send().await
    }

    /// Sends a POST request with a JSON body.
    pub(crate) async fn post<T: Serialize>(&self, path: &str, body: &T) -> Result<Response, Error> {
        let url = format!("{}/{}", self.base_url, path);
        self.client
            .post(&url)
            .basic_auth(&self.username, Some(&self.password))
            .json(body)
            .send()
            .await
    }

    /// Sends a DELETE request.
    #[allow(dead_code)]
    pub(crate) async fn delete(&self, path: &str) -> Result<Response, Error> {
        let url = format!("{}/{}", self.base_url, path);
        self.client.delete(&url).basic_auth(&self.username, Some(&self.password)).send().await
    }
}
