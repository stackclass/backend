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

use crate::{
    config::Config, context::Context, errors::Result, model::CourseModel,
    repository::CourseRepository, response::CourseResponse, schema,
    service::storage::StorageService,
};

pub struct CourseService;

impl CourseService {
    pub async fn find(ctx: Arc<Context>) -> Result<Vec<CourseResponse>> {
        let courses = CourseRepository::find(&ctx.database).await?;
        Ok(courses.into_iter().map(Into::into).collect())
    }

    pub async fn create(ctx: Arc<Context>, url: &str) -> Result<CourseResponse> {
        let Config { cache_dir, github_token, .. } = &ctx.config;

        let storage = StorageService::new(cache_dir, github_token)?;
        let dir = storage.fetch(url).await?;

        let course = schema::parse(&cache_dir.join(dir))?;
        debug!("parsed course: {:?}", course);

        let model = CourseModel::from(course);
        let course = CourseRepository::create(&ctx.database, &model).await?;

        Ok(course.into())
    }

    pub async fn get(ctx: Arc<Context>, slug: &str) -> Result<CourseResponse> {
        let course = CourseRepository::get_by_slug(&ctx.database, slug).await?;
        Ok(course.into())
    }

    pub async fn sync(ctx: Arc<Context>, url: &str) -> Result<bool> {
        let Config { cache_dir, github_token, .. } = &ctx.config;

        let storage = StorageService::new(cache_dir, github_token)?;
        let dir = storage.fetch(url).await?;

        let course = schema::parse(&cache_dir.join(dir))?;
        debug!("parsed course: {:?}", course);

        let model = CourseModel::from(course);
        CourseRepository::update(&ctx.database, &model).await?;

        Ok(true)
    }

    pub(crate) async fn delete(ctx: Arc<Context>, slug: &str) -> Result<()> {
        CourseRepository::delete(&ctx.database, slug).await
    }
}
