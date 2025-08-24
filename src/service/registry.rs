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

use harbor_client::{ClientError, types::CreateProjectRequest};
use tracing::info;

use crate::{context::Context, errors::Result};

/// Service for container registry operations
pub struct RegistryService;

impl RegistryService {
    /// Ensure a project exists in the Harbor registry
    /// Checks if the project exists, and creates it if not
    pub async fn ensure_project(ctx: &Context, name: &str) -> Result<()> {
        match ctx.harbor.head_project(name).await {
            Ok(_) => Ok(()), // Project exists, nothing to do
            Err(ClientError::NotFound) => {
                let request = CreateProjectRequest::new(name).with_public(true);
                ctx.harbor.create_project(request).await?;
                info!("Project '{}' created successfully in Harbor registry", name);

                Ok(())
            }
            Err(e) => Err(e.into()), // Propagate other errors
        }
    }
}
