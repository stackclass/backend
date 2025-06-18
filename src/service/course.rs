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

use std::{collections::HashSet, sync::Arc};
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::{
    config::Config,
    context::Context,
    database::Transaction,
    errors::{ApiError, Result},
    model::{CourseModel, ExtensionModel, SolutionModel, StageModel},
    repository::{CourseRepository, ExtensionRepository, SolutionRepository, StageRepository},
    response::{CourseDetailResponse, CourseResponse},
    schema::{self, Course, Stage},
    service::storage::StorageService,
};

/// Service for managing courses and related entities
pub struct CourseService;

impl CourseService {
    /// Fetch all courses from repository
    pub async fn find(ctx: Arc<Context>) -> Result<Vec<CourseResponse>> {
        let courses = CourseRepository::find(&ctx.database).await?;
        Ok(courses.into_iter().map(Into::into).collect())
    }

    /// Create new course from git repository URL
    pub async fn create(ctx: Arc<Context>, repository: &str) -> Result<CourseResponse> {
        let Config { cache_dir, github_token, .. } = &ctx.config;

        let storage = StorageService::new(cache_dir, github_token)?;
        let dir = storage.fetch(repository).await?;

        let course = schema::parse(&cache_dir.join(dir))?;
        debug!("Parsed course: {:?}", course.name);

        if let Ok(model) = CourseRepository::get_by_slug(&ctx.database, &course.slug).await {
            info!("Course already exists: {:?}", course.name);
            return Ok(model.into());
        }

        let model = Self::create_course(ctx, course, repository).await?;
        info!("Successfully created course: {:?}", model.name);

        Ok(model.into())
    }

    /// Create course with all related entities in transaction
    async fn create_course(ctx: Arc<Context>, course: Course, url: &str) -> Result<CourseModel> {
        let mut tx = ctx.database.pool().begin().await?;

        // Persist the course
        let course_model = CourseModel::from(&course).with_repository(url);
        let course_model = CourseRepository::create(&mut tx, &course_model).await?;

        // Persist stages and their solutions
        for stage in course.stages.values() {
            Self::create_stage(&mut tx, stage, course_model.id, None).await?;
        }

        // Persist extensions and their stages
        if let Some(extensions) = course.extensions {
            for ext in extensions.values() {
                let ext_model = ExtensionModel::from(ext.clone()).with_course(course_model.id);
                let ext_model = ExtensionRepository::create(&mut tx, &ext_model).await?;

                for stage in ext.stages.values() {
                    Self::create_stage(&mut tx, stage, course_model.id, Some(ext_model.id)).await?;
                }
            }
        }

        // Commits this transaction
        tx.commit().await?;

        Ok(course_model)
    }

    /// Create stage and optionally its solution
    async fn create_stage(
        tx: &mut Transaction<'_>,
        stage: &Stage,
        course_id: Uuid,
        ext_id: Option<Uuid>,
    ) -> Result<()> {
        let mut stage_model = StageModel::from(stage.clone()).with_course(course_id);
        if let Some(extension_id) = ext_id {
            stage_model = stage_model.with_extension(extension_id);
        }
        let stage_model = StageRepository::create(tx, &stage_model).await?;

        if let Some(sol) = &stage.solution {
            let solution_model = SolutionModel::from(sol.clone()).with_stage(stage_model.id);
            SolutionRepository::create(tx, &solution_model).await?;
        }

        Ok(())
    }

    /// Get course by slug
    pub async fn get(ctx: Arc<Context>, slug: &str) -> Result<CourseDetailResponse> {
        let course = CourseRepository::get_by_slug(&ctx.database, slug).await?;
        Ok(course.into())
    }

    /// Update course from git repository URL
    pub async fn update(ctx: Arc<Context>, slug: &str) -> Result<bool> {
        let Ok(model) = CourseRepository::get_by_slug(&ctx.database, slug).await else {
            error!("Course not found: {:?}", slug);
            return Err(ApiError::NotFound);
        };

        let Config { cache_dir, github_token, .. } = &ctx.config;

        let storage = StorageService::new(cache_dir, github_token)?;
        let dir = storage.fetch(&model.repository).await?;

        let course = schema::parse(&cache_dir.join(dir))?;
        debug!("Parsed course: {:?}", course.name);

        Self::update_course(ctx, course).await?;
        info!("Successfully updated course: {:?}", model.name);

        Ok(true)
    }

    /// Update course and related entities with cleanup
    async fn update_course(ctx: Arc<Context>, course: Course) -> Result<()> {
        let mut tx = ctx.database.pool().begin().await?;

        // Fetch existing stages and extensions
        let slug = &course.slug;
        let existing_stages = StageRepository::find_by_course(&ctx.database, slug).await?;
        let existing_exts = ExtensionRepository::find_by_course(&ctx.database, slug).await?;

        // Update the course
        let course_model = CourseModel::from(&course);
        let course_model = CourseRepository::update(&mut tx, &course_model).await?;

        // Track current slugs for cleanup
        let mut current_stage_slugs = HashSet::new();
        let mut current_extension_slugs = HashSet::new();

        // Update and track base stages
        for stage in course.stages.values() {
            Self::update_stage(&mut tx, stage, course_model.id, None).await?;
            current_stage_slugs.insert(stage.slug.clone());
        }

        // Update and track extension stages
        if let Some(extensions) = &course.extensions {
            for ext in extensions.values() {
                let ext_model = ExtensionModel::from(ext.clone()).with_course(course_model.id);
                let ext_model = ExtensionRepository::upsert(&mut tx, &ext_model).await?;

                // Upsert extension stages and their solutions
                for stage in ext.stages.values() {
                    Self::update_stage(&mut tx, stage, course_model.id, Some(ext_model.id)).await?;
                    current_stage_slugs.insert(stage.slug.clone());
                }

                current_extension_slugs.insert(ext.slug.clone());
            }
        }

        // Cleanup orphaned stages (both base and extension stages)
        debug!(
            "Existing stage slugs: {:?}, Current stage slugs: {:?}",
            existing_stages.iter().map(|s| &s.slug).collect::<Vec<_>>(),
            current_stage_slugs
        );
        for existing_stage in existing_stages {
            if !current_stage_slugs.contains(&existing_stage.slug) {
                StageRepository::delete(&mut tx, &existing_stage.slug).await?;
            }
        }

        // Cleanup orphaned extensions
        debug!(
            "Existing extension slugs: {:?}, Current extension slugs: {:?}",
            existing_exts.iter().map(|e| &e.slug).collect::<Vec<_>>(),
            current_extension_slugs
        );
        for existing_extension in existing_exts {
            if !current_extension_slugs.contains(&existing_extension.slug) {
                ExtensionRepository::delete(&mut tx, &existing_extension.slug).await?;
            }
        }

        // Commits this transaction
        tx.commit().await?;

        Ok(())
    }

    /// Update stage and handle solution changes
    async fn update_stage(
        tx: &mut Transaction<'_>,
        stage: &Stage,
        course_id: Uuid,
        ext_id: Option<Uuid>,
    ) -> Result<()> {
        let mut stage_model = StageModel::from(stage.clone()).with_course(course_id);
        if let Some(extension_id) = ext_id {
            stage_model = stage_model.with_extension(extension_id);
        }
        let stage_model = StageRepository::upsert(tx, &stage_model).await?;

        // Upsert solutions
        if let Some(sol) = &stage.solution {
            let solution_model = SolutionModel::from(sol.clone()).with_stage(stage_model.id);
            SolutionRepository::upsert(tx, &stage.slug, &solution_model).await?;
        } else {
            // Delete solution if it exists but stage no longer has one
            SolutionRepository::delete(tx, &stage.slug).await?;
        }

        Ok(())
    }

    /// Delete course by slug
    pub(crate) async fn delete(ctx: Arc<Context>, slug: &str) -> Result<()> {
        CourseRepository::delete(&ctx.database, slug).await.map_err(ApiError::DatabaseError)
    }
}
