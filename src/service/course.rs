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

use std::{collections::HashSet, sync::Arc};
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::{
    config::Config,
    context::Context,
    database::Transaction,
    errors::{ApiError, Result},
    model::{CourseModel, ExtensionModel, StageModel, UserCourseModel, UserStageModel},
    repository::{CourseRepository, ExtensionRepository, StageRepository, UserRepository},
    request::{CreateUserCourseRequest, UpdateUserCourseRequest},
    response::{AttemptResponse, CourseDetailResponse, CourseResponse, UserCourseResponse},
    schema::{self, Course, Stage},
    service::storage::StorageService,
};

use super::RepoService;

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

        let model = Self::create_course(ctx.clone(), &course, repository).await?;
        info!("Successfully created course: {:?}", course.name);

        RepoService::new(ctx.clone()).init(&course.slug, repository).await?;
        info!("Successfully initialized template repository for course: {:?}", course.name);

        Ok(model.into())
    }

    /// Create course with all related entities in transaction
    async fn create_course(ctx: Arc<Context>, course: &Course, url: &str) -> Result<CourseModel> {
        let mut tx = ctx.database.pool().begin().await?;

        // Persist the course
        let course_model = CourseModel::from(course)
            .with_repository(url)
            .with_stage_count(calculate_total_stages(course));
        let course_model = CourseRepository::create(&mut tx, &course_model).await?;

        // Persist stages and their solutions with weight
        for (index, (_, stage)) in course.stages.iter().enumerate() {
            Self::create_stage(&mut tx, stage, course_model.id, None, index as i32).await?;
        }

        // Persist extensions and their stages with weight
        if let Some(extensions) = &course.extensions {
            for (index, (_, ext)) in extensions.iter().enumerate() {
                let ext_model = ExtensionModel::from(ext.clone())
                    .with_course(course_model.id)
                    .with_stage_count(ext.stages.len() as i32)
                    .with_weight(index as i32);
                let ext_model = ExtensionRepository::create(&mut tx, &ext_model).await?;

                for (stage_index, (_, stage)) in ext.stages.iter().enumerate() {
                    let weight = ((index + 1) * 1000 + stage_index) as i32;
                    Self::create_stage(&mut tx, stage, course_model.id, Some(ext_model.id), weight)
                        .await?;
                }
            }
        }

        // Commits this transaction
        tx.commit().await?;

        Ok(course_model)
    }

    /// Create stage
    async fn create_stage(
        tx: &mut Transaction<'_>,
        stage: &Stage,
        course_id: Uuid,
        ext_id: Option<Uuid>,
        weight: i32,
    ) -> Result<()> {
        let mut stage_model =
            StageModel::from(stage.clone()).with_course(course_id).with_weight(weight);

        if let Some(extension_id) = ext_id {
            stage_model = stage_model.with_extension(extension_id);
        }

        let _ = StageRepository::create(tx, &stage_model).await?;

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

        Self::update_course(ctx.clone(), &course).await?;
        info!("Successfully updated course: {:?}", model.name);

        RepoService::new(ctx).init(&course.slug, &model.repository).await?;
        info!("Template repository for course {:?} has been synced", course.name);

        Ok(true)
    }

    /// Update course and related entities with cleanup
    async fn update_course(ctx: Arc<Context>, course: &Course) -> Result<()> {
        let mut tx = ctx.database.pool().begin().await?;

        // Fetch existing stages and extensions
        let slug = &course.slug;
        let existing_stages = StageRepository::find_by_course(&ctx.database, slug).await?;
        let existing_exts = ExtensionRepository::find_by_course(&ctx.database, slug).await?;

        // Update the course
        let course_model =
            CourseModel::from(course).with_stage_count(calculate_total_stages(course));
        let course_model = CourseRepository::update(&mut tx, &course_model).await?;

        // Track current slugs for cleanup
        let mut current_stage_slugs = HashSet::new();
        let mut current_extension_slugs = HashSet::new();

        // Update and track base stages with weight
        for (index, (_, stage)) in course.stages.iter().enumerate() {
            Self::update_stage(&mut tx, stage, course_model.id, None, index as i32).await?;
            current_stage_slugs.insert(stage.slug.clone());
        }

        // Update and track extension stages with weight
        if let Some(extensions) = &course.extensions {
            for (index, (_, ext)) in extensions.iter().enumerate() {
                let ext_model = ExtensionModel::from(ext.clone())
                    .with_course(course_model.id)
                    .with_stage_count(ext.stages.len() as i32)
                    .with_weight(index as i32);
                let ext_model = ExtensionRepository::upsert(&mut tx, &ext_model).await?;

                // Upsert extension stages and their solutions
                for (stage_index, (_, stage)) in ext.stages.iter().enumerate() {
                    let weight = ((index + 1) * 1000 + stage_index) as i32;
                    Self::update_stage(&mut tx, stage, course_model.id, Some(ext_model.id), weight)
                        .await?;
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
        weight: i32,
    ) -> Result<()> {
        let mut stage_model =
            StageModel::from(stage.clone()).with_course(course_id).with_weight(weight);

        if let Some(extension_id) = ext_id {
            stage_model = stage_model.with_extension(extension_id);
        }

        let _ = StageRepository::upsert(tx, &stage_model).await?;

        Ok(())
    }

    /// Delete course by slug
    pub(crate) async fn delete(ctx: Arc<Context>, slug: &str) -> Result<()> {
        CourseRepository::delete(&ctx.database, slug).await.map_err(ApiError::DatabaseError)
    }

    /// Fetch all courses for the user.
    pub async fn find_user_courses(
        ctx: Arc<Context>,
        user_id: &str,
    ) -> Result<Vec<UserCourseResponse>> {
        let courses = CourseRepository::find_user_courses(&ctx.database, user_id).await?;

        let endpoint = &ctx.config.git_server_endpoint;
        Ok(courses.into_iter().map(|model| UserCourseResponse::from((endpoint, model))).collect())
    }

    /// Enroll a user in a course.
    pub async fn create_user_course(
        ctx: Arc<Context>,
        user_id: &str,
        req: &CreateUserCourseRequest,
    ) -> Result<UserCourseResponse> {
        let mut tx = ctx.database.pool().begin().await?;

        // Fetch the user and course
        let user = UserRepository::get_by_id(&ctx.database, user_id).await?;
        let course = CourseRepository::get_by_slug(&ctx.database, &req.course_slug).await?;

        // Create a new user course enrollment
        let user_course = UserCourseModel::new(user_id, &course.id)
            .with_proficiency(&req.proficiency)
            .with_cadence(&req.cadence)
            .with_accountability(req.accountability);

        let user_course =
            CourseRepository::create_user_course(&mut tx, &user_course).await.map_err(|e| {
                if let sqlx::Error::Database(db_err) = &e {
                    if db_err.is_unique_violation() {
                        return ApiError::Conflict("User is already enrolled in this course".into());
                    }
                }
                e.into()
            })?;
        tx.commit().await?;

        // Generate Git repository from course template
        RepoService::new(ctx.clone()).generate(&user, &course.slug).await?;

        let endpoint = &ctx.config.git_server_endpoint;
        Ok(UserCourseResponse::from((endpoint, user_course)))
    }

    /// Fetch the course detail for the user.
    pub async fn get_user_course(
        ctx: Arc<Context>,
        user_id: &str,
        course_slug: &str,
    ) -> Result<UserCourseResponse> {
        let user_course =
            CourseRepository::get_user_course(&ctx.database, user_id, course_slug).await?;

        let endpoint = &ctx.config.git_server_endpoint;
        Ok(UserCourseResponse::from((endpoint, user_course)))
    }

    /// Update the user course for the user.
    pub async fn update_user_course(
        ctx: Arc<Context>,
        user_id: &str,
        course_slug: &str,
        req: &UpdateUserCourseRequest,
    ) -> Result<()> {
        let mut user_course =
            CourseRepository::get_user_course(&ctx.database, user_id, course_slug).await?;
        user_course.proficiency = req.proficiency.clone();
        user_course.cadence = req.cadence.clone();
        user_course.accountability = req.accountability;

        let mut tx = ctx.database.pool().begin().await?;
        CourseRepository::update_user_course(&mut tx, &user_course).await?;
        tx.commit().await?;

        Ok(())
    }

    /// Activates a user course by setting activated flag and creating first stage
    pub async fn activate(
        ctx: Arc<Context>,
        user_course: &mut UserCourseModel,
    ) -> Result<(), ApiError> {
        let mut tx = ctx.database.pool().begin().await?;

        user_course.activated = true;

        // Find first stage by weight
        if let Some(stage) = StageRepository::first(&ctx.database, &user_course.course_slug).await?
        {
            // Create user stage
            let user_stage = UserStageModel::new(user_course.id, stage.id);
            StageRepository::create_user_stage(&mut tx, &user_stage).await?;

            user_course.current_stage_id = Some(stage.id);
        }

        CourseRepository::update_user_course(&mut tx, user_course).await?;
        tx.commit().await?;

        Ok(())
    }

    /// Fetch all attempts for a course.
    pub async fn find_attempts(ctx: Arc<Context>, slug: &str) -> Result<Vec<AttemptResponse>> {
        let attempts = CourseRepository::find_attempts(&ctx.database, slug).await?;
        Ok(attempts.into_iter().map(Into::into).collect())
    }
}

fn calculate_total_stages(course: &Course) -> i32 {
    let mut total = course.stages.len() as i32;
    if let Some(extensions) = &course.extensions {
        total += extensions.iter().map(|(_, ext)| ext.stages.len() as i32).sum::<i32>();
    }
    total
}
