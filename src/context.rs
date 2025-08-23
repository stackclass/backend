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

use gitea_client::GiteaClient;
use harbor_client::HarborClient;
use reqwest::Client;

use crate::{config::Config, database::Database, errors::Result};

/// The core type through which handler functions can access common API state.
pub struct Context {
    /// Application configuration settings
    pub config: Config,

    /// Database connection pool and operations
    pub database: Database,

    /// Client for interacting with the Gitea API
    pub git: GiteaClient,

    /// Client for interacting with the Harbor API
    pub harbor: HarborClient,

    /// Kubernetes client for cluster operations
    pub k8s: kube::Client,

    /// HTTP client for making external requests
    pub http: Client,
}

impl Context {
    pub async fn new(config: Config) -> Result<Context> {
        let database = Database::new(&config.database_url).await?;

        // Initialize Gitea client for source control operations
        let git = GiteaClient::new(
            config.git_server_endpoint.clone(),
            config.git_server_username.clone(),
            config.git_server_password.clone(),
        );

        // Initialize Harbor client for container registry operations
        let harbor = HarborClient::new(
            config.docker_registry_endpoint.clone(),
            config.docker_registry_username.clone(),
            config.docker_registry_password.clone(),
        );

        let k8s = kube::Client::try_default().await?;
        let http = Client::new();

        Ok(Context { config, database, git, harbor, k8s, http })
    }
}
