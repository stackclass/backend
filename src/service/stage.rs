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

use std::sync::Arc;

use crate::{
    context::Context,
    database::{Database, Transaction},
    errors::{ApiError, Result},
    model::{UserCourseModel, UserStageModel},
    repository::{CourseRepository, StageRepository},
    response::{StageDetailResponse, StageResponse, UserStageResponse, UserStageStatusResponse},
};

/// Service for managing stages
pub struct StageService;

impl StageService {
    /// Find all stages for a course (including extensions)
    pub async fn find_all_stages(ctx: Arc<Context>, slug: &str) -> Result<Vec<StageResponse>> {
        let stages = StageRepository::find_by_course(&ctx.database, slug).await?;
        Ok(stages.into_iter().map(Into::into).collect())
    }

    /// Find only base stages for a course (excluding extensions).
    pub async fn find_base_stages(ctx: Arc<Context>, slug: &str) -> Result<Vec<StageResponse>> {
        let stages = StageRepository::find_base_by_course(&ctx.database, slug).await?;
        Ok(stages.into_iter().map(Into::into).collect())
    }

    /// Find only extended stages for a course.
    pub async fn find_extended_stages(ctx: Arc<Context>, slug: &str) -> Result<Vec<StageResponse>> {
        let stages = StageRepository::find_extended_by_course(&ctx.database, slug).await?;
        Ok(stages.into_iter().map(Into::into).collect())
    }

    /// Get the details of the stage.
    pub async fn get(
        ctx: Arc<Context>,
        course_slug: &str,
        stage_slug: &str,
    ) -> Result<StageDetailResponse> {
        let stage = StageRepository::get_by_slug(&ctx.database, course_slug, stage_slug).await?;
        Ok(stage.into())
    }

    /// Fetch user stages for the user.
    pub async fn find_user_stages(
        ctx: Arc<Context>,
        user_id: &str,
        course_slug: &str,
    ) -> Result<Vec<UserStageResponse>> {
        let stages = StageRepository::find_user_stages(&ctx.database, user_id, course_slug).await?;
        Ok(stages.into_iter().map(Into::into).collect())
    }

    /// Get the details of the stage for the user.
    pub async fn get_user_stage(
        ctx: Arc<Context>,
        user_id: &str,
        course_slug: &str,
        stage_slug: &str,
    ) -> Result<UserStageResponse> {
        let stage =
            StageRepository::get_user_stage(&ctx.database, user_id, course_slug, stage_slug)
                .await?;
        Ok(stage.into())
    }

    /// Mark a stage as completed for a user.
    pub async fn complete(
        ctx: Arc<Context>,
        user_id: &str,
        course_slug: &str,
        stage_slug: &str,
    ) -> Result<UserStageResponse> {
        let db = &ctx.database;

        //  Fetch the user's course enrollment and current user stage.
        let user_course = CourseRepository::get_user_course(db, user_id, course_slug).await?;
        let mut user_stage =
            StageRepository::get_user_stage(db, user_id, course_slug, stage_slug).await?;

        //  Validate the stage can be completed.
        if user_stage.status == "completed" {
            return Err(ApiError::StageAlreadyCompleted);
        }
        if user_stage.status != "in_progress" {
            return Err(ApiError::StageNotInProgress);
        }
        if user_course.current_stage_id != Some(user_stage.stage_id) {
            return Err(ApiError::StageOutOfOrder);
        }

        // Begins a new transaction.
        let mut tx = ctx.database.pool().begin().await?;

        // Mark the stage as completed.
        user_stage = user_stage.passed().complete();
        let completed_stage = StageRepository::update_user_stage(&mut tx, &user_stage).await?;

        // Update user course and create next stage if needed.
        Self::start_next_stage(&mut tx, db, user_course, course_slug, stage_slug).await?;

        // Commits this transaction.
        tx.commit().await?;

        Ok(completed_stage.into())
    }

    /// Update user course and create next stage if needed.
    async fn start_next_stage(
        tx: &mut Transaction<'_>,
        db: &Database,
        user_course: UserCourseModel,
        course_slug: &str,
        stage_slug: &str,
    ) -> Result<()> {
        let mut updated_user_course = user_course;

        // Find the next stage (if any) by current stage slug.
        let next_stage =
            StageRepository::find_next_stage_by_slug(db, course_slug, stage_slug).await?;

        // If there is a next stage, create a new instance for it
        if let Some(next_stage) = next_stage {
            let user_stage = UserStageModel::new(updated_user_course.id, next_stage.id);
            match StageRepository::create_user_stage(tx, &user_stage).await {
                Ok(model) => model,
                Err(e) if e.as_database_error().is_some() => {
                    return Err(ApiError::Conflict(
                        "User already has a record for this stage".into(),
                    ));
                }
                Err(e) => return Err(e.into()),
            };
            updated_user_course.current_stage_id = Some(next_stage.id);
        }

        updated_user_course.completed_stage_count += 1;
        CourseRepository::update_user_course(tx, &updated_user_course).await?;

        Ok(())
    }

    /// Get the current status of a stage for the user.
    pub async fn get_user_stage_status(
        ctx: &Arc<Context>,
        user_id: &str,
        course_slug: &str,
        stage_slug: &str,
    ) -> Result<UserStageStatusResponse> {
        // Fetch the user's stage record from the database.
        let user_stage =
            StageRepository::get_user_stage(&ctx.database, user_id, course_slug, stage_slug)
                .await?;

        Ok(UserStageStatusResponse { status: user_stage.status, test: user_stage.test })
    }
}
