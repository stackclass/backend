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
use tracing::{debug, info};

use crate::{
    config::Config,
    context::Context,
    errors::Result,
    model::{CourseModel, ExtensionModel, SolutionModel, StageModel},
    repository::{CourseRepository, ExtensionRepository, SolutionRepository, StageRepository},
    response::CourseResponse,
    schema,
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
        debug!("Parsed course: {:?}", course.name);

        if let Ok(course) = Self::get(ctx.clone(), &course.slug).await {
            info!("Course already exists: {:?}", course.name);
            return Ok(course);
        }

        Self::create_course(ctx, course).await
    }

    async fn create_course(ctx: Arc<Context>, course: schema::Course) -> Result<CourseResponse> {
        let mut tx = ctx.database.pool().begin().await?;

        // Persist the course
        let course_model = CourseModel::from(&course);
        let course_model = CourseRepository::create(&mut tx, &course_model).await?;

        // Persist stages and their solutions
        for stage in course.stages.values() {
            let stage_model = StageModel::from(stage.clone()).with_course(course_model.id);
            let stage_model = StageRepository::create(&mut tx, &stage_model).await?;

            if let Some(sol) = &stage.solution {
                let solution_model = SolutionModel::from(sol.clone()).with_stage(stage_model.id);
                SolutionRepository::create(&mut tx, &solution_model).await?;
            }
        }

        // Persist extensions and their stages
        if let Some(extensions) = course.extensions {
            for ext in extensions.values() {
                let ext_model = ExtensionModel::from(ext.clone()).with_course(course_model.id);
                let ext_model = ExtensionRepository::create(&mut tx, &ext_model).await?;

                for stage in ext.stages.values() {
                    let stage_model = StageModel::from(stage.clone())
                        .with_course(course_model.id)
                        .with_extension(ext_model.id);
                    let stage_model = StageRepository::create(&mut tx, &stage_model).await?;

                    if let Some(sol) = &stage.solution {
                        let sol_model = SolutionModel::from(sol.clone()).with_stage(stage_model.id);
                        SolutionRepository::create(&mut tx, &sol_model).await?;
                    }
                }
            }
        }
        tx.commit().await?;

        Ok(course_model.into())
    }

    pub async fn get(ctx: Arc<Context>, slug: &str) -> Result<CourseResponse> {
        let course = CourseRepository::get_by_slug(&ctx.database, slug).await?;
        Ok(course.into())
    }

    pub async fn update(ctx: Arc<Context>, url: &str) -> Result<bool> {
        let Config { cache_dir, github_token, .. } = &ctx.config;

        let storage = StorageService::new(cache_dir, github_token)?;
        let dir = storage.fetch(url).await?;

        let course = schema::parse(&cache_dir.join(dir))?;
        debug!("Parsed course: {:?}", course.name);

        let model = CourseModel::from(&course);
        CourseRepository::update(&ctx.database, &model).await?;

        Ok(true)
    }

    pub(crate) async fn delete(ctx: Arc<Context>, slug: &str) -> Result<()> {
        CourseRepository::delete(&ctx.database, slug).await
    }
}
