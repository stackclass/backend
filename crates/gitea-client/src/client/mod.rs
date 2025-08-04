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

pub mod hook;
pub mod repository;
pub mod user;

use base64::Engine;
use reqwest::{
    Client, Error, Response,
    header::{AUTHORIZATION, HeaderMap, HeaderValue},
};
use serde::Serialize;

/// A client for interacting with the Gitea API.
pub struct GiteaClient {
    pub(crate) client: Client,
    pub(crate) base_url: String,
    pub(crate) username: String,
    pub(crate) password: String,
}

impl GiteaClient {
    /// Creates a new `GiteaClient` instance.
    pub fn new(base_url: String, username: String, password: String) -> Self {
        GiteaClient { client: Client::new(), base_url, username, password }
    }

    /// Builds the headers with Basic Auth.
    pub(crate) fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let engine = base64::engine::general_purpose::STANDARD;
        let encoded = engine.encode(format!("{}:{}", self.username, self.password));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Basic {encoded}")).unwrap());
        headers
    }

    /// Sends a GET request.
    pub(crate) async fn get(&self, endpoint: &str) -> Result<Response, Error> {
        let url = format!("{}/{}", self.base_url, endpoint);
        self.client.get(&url).headers(self.build_headers()).send().await
    }

    /// Sends a POST request with a JSON body.
    pub(crate) async fn post<T: Serialize>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<Response, Error> {
        let url = format!("{}/{}", self.base_url, endpoint);
        self.client.post(&url).headers(self.build_headers()).json(body).send().await
    }

    /// Sends a DELETE request.
    pub(crate) async fn delete(&self, endpoint: &str) -> Result<Response, Error> {
        let url = format!("{}/{}", self.base_url, endpoint);
        self.client.delete(&url).headers(self.build_headers()).send().await
    }
}
