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

use uuid::Uuid;

use crate::{database::Database, errors::Result, model::SolutionModel};

/// Repository for managing solutions in the database.
pub struct SolutionRepository;

impl SolutionRepository {
    /// Create a new solution in the database.
    pub async fn create(db: &Database, solution: &SolutionModel) -> Result<SolutionModel> {
        let row = sqlx::query_as::<_, SolutionModel>(
            r#"
            INSERT INTO solutions (
                id, stage_id, explanation, patches, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(solution.id)
        .bind(solution.stage_id)
        .bind(&solution.explanation)
        .bind(&solution.patches)
        .bind(solution.created_at)
        .bind(solution.updated_at)
        .fetch_one(db.pool())
        .await?;

        Ok(row)
    }

    /// Fetch a solution by its stage slug.
    pub async fn get_by_stage(db: &Database, stage_slug: &str) -> Result<SolutionModel> {
        let row = sqlx::query_as::<_, SolutionModel>(
            r#"
            SELECT s.* FROM solutions s
            JOIN stages st ON s.stage_id = st.id
            WHERE st.slug = $1
            "#,
        )
        .bind(stage_slug)
        .fetch_one(db.pool())
        .await?;

        Ok(row)
    }

    /// Fetch a solution by its internal ID.
    pub async fn get_by_id(db: &Database, id: Uuid) -> Result<SolutionModel> {
        let row = sqlx::query_as::<_, SolutionModel>(r#"SELECT * FROM solutions WHERE id = $1"#)
            .bind(id)
            .fetch_one(db.pool())
            .await?;

        Ok(row)
    }

    /// Update a solution by its stage slug.
    pub async fn update(
        db: &Database,
        stage_slug: &str,
        solution: &SolutionModel,
    ) -> Result<SolutionModel> {
        let row = sqlx::query_as::<_, SolutionModel>(
            r#"
            UPDATE solutions
            SET explanation = $2, patches = $3, updated_at = $4
            FROM stages st
            WHERE solutions.stage_id = st.id AND st.slug = $1
            RETURNING solutions.*
            "#,
        )
        .bind(stage_slug)
        .bind(&solution.explanation)
        .bind(&solution.patches)
        .bind(solution.updated_at)
        .fetch_one(db.pool())
        .await?;

        Ok(row)
    }

    /// Delete a solution by its stage slug.
    pub async fn delete(db: &Database, stage_slug: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM solutions
            USING stages st
            WHERE solutions.stage_id = st.id AND st.slug = $1
            "#,
        )
        .bind(stage_slug)
        .execute(db.pool())
        .await?;

        Ok(())
    }
}
