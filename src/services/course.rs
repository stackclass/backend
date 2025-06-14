// Copyright (c) wangeguo. All rights reserved.
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

use std::sync::Arc;

use tracing::debug;

use crate::{context::Context, errors::Result, schema, services::storage::StorageService};

pub struct CourseService;

impl CourseService {
    pub async fn sync(ctx: Arc<Context>, url: &str) -> Result<bool> {
        let service = StorageService::new(&ctx.config.cache_dir, &ctx.config.github_token)?;
        let dir = service.fetch(url).await?;

        let course = schema::parse(&dir)?;

        debug!("parsed course: {:?}", course);

        Ok(true)
    }
}
