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

// The configuration parameters for the application.
// These can either be passed on the command line, or pulled from environment variables.
// The latter is preferred as environment variables are one of the recommended ways to
// get configuration from Kubernetes Secrets in deployment.
//
// This is a pretty simple configuration struct as far as backend APIs go. You could imagine
// a bunch of other parameters going here, like API keys for external services
// or flags enabling or disabling certain features or test modes of the API.
//
// For development convenience, these can also be read from a `.env` file in the working
// directory where the application is started.
//
// See `.env.example` in the repository root for details.

use std::path::PathBuf;

#[derive(Clone, clap::Parser)]
pub struct Config {
    /// The server port.
    #[clap(long, env, default_value = "8080")]
    pub port: u16,

    /// Base directory for storing cached repositories.
    #[clap(long, env)]
    pub cache_dir: PathBuf,

    /// A personal token to use for authentication.
    #[clap(long, env)]
    pub github_token: Option<String>,

    /// Database connection URL.
    #[clap(long, env)]
    pub database_url: String,

    /// Allowed CORS origin.
    #[clap(long, env, value_delimiter = ',')]
    pub allowed_origin: Option<Vec<String>>,

    /// Git proxy endpoint.
    #[clap(long, env)]
    pub git_proxy_endpoint: String,

    /// Git server endpoint.
    #[clap(long, env)]
    pub git_server_endpoint: String,

    /// Username for authenticating with the git server.
    #[clap(long, env)]
    pub git_server_username: String,

    /// Password for authenticating with the git server.
    #[clap(long, env)]
    pub git_server_password: String,

    /// Webhook handler endpoint.
    #[clap(long, env)]
    pub webhook_endpoint: String,

    /// Git committer name.
    #[clap(long, env, default_value = "StackClass")]
    pub git_committer_name: String,

    /// Git committer email.
    #[clap(long, env, default_value = "hello@stackclass.dev")]
    pub git_committer_email: String,

    /// Kubernetes namespace where StackClass is running.
    #[clap(long, env)]
    pub namespace: String,

    /// Docker registry endpoint.
    #[clap(long, env)]
    pub docker_registry_endpoint: String,

    /// Secret used for hashing user passwords.
    #[clap(long, env)]
    pub auth_secret: String,
}
