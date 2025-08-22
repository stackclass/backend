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

use axum::body::Body;
use gitea_client::GiteaClient;
use hyper_util::{
    client::legacy::{Client, connect::HttpConnector},
    rt::TokioExecutor,
};

use crate::{config::Config, database::Database, errors::Result};

/// The core type through which handler functions can access common API state.
pub struct Context {
    /// Application configuration settings
    pub config: Config,

    /// Database connection pool and operations
    pub database: Database,

    /// Client for interacting with the Gitea API
    pub git: GiteaClient,

    /// Kubernetes client for cluster operations
    pub k8s: kube::Client,

    /// HTTP client for making external requests
    pub http: Client<HttpConnector, Body>,
}

impl Context {
    pub async fn new(config: Config) -> Result<Context> {
        let database = Database::new(&config.database_url).await?;
        let git = GiteaClient::new(
            config.git_server_endpoint.clone(),
            config.git_server_username.clone(),
            config.git_server_password.clone(),
        );
        let k8s = kube::Client::try_default().await?;
        let http = Client::builder(TokioExecutor::new()).build_http();

        Ok(Context { config, database, git, k8s, http })
    }
}
