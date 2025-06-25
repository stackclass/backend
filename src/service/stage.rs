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

use crate::{
    context::Context,
    errors::Result,
    repository::{SolutionRepository, StageRepository},
    response::{StageDetailResponse, StageResponse, UserStageResponse},
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
        // Load stage and solution
        let stage = StageRepository::get_by_slug(&ctx.database, course_slug, stage_slug).await?;
        let solution = SolutionRepository::get_by_stage(&ctx.database, stage_slug).await.ok();

        // Build response
        let mut response: StageDetailResponse = stage.into();
        response.solution = solution.map(|s| s.into());

        Ok(response)
    }

    /// Fetch user stages for the current user.
    pub async fn find_user_stages(
        ctx: Arc<Context>,
        user_id: &str,
        course_slug: &str,
    ) -> Result<Vec<UserStageResponse>> {
        let stages = StageRepository::find_user_stages(&ctx.database, user_id, course_slug).await?;
        Ok(stages.into_iter().map(Into::into).collect())
    }
}
