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

use crate::{database::Database, errors::Result, model::StageModel};

/// Repository for managing stages in the database.
pub struct StageRepository;

impl StageRepository {
    /// Create a new stage in the database.
    pub async fn create(db: &Database, stage: &StageModel) -> Result<StageModel> {
        let row = sqlx::query_as::<_, StageModel>(
            r#"
            INSERT INTO stages (
                id, course_id, extension_id, slug, name, difficulty, description, instruction, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
        )
        .bind(stage.id)
        .bind(stage.course_id)
        .bind(stage.extension_id)
        .bind(&stage.slug)
        .bind(&stage.name)
        .bind(&stage.difficulty)
        .bind(&stage.description)
        .bind(&stage.instruction)
        .bind(stage.created_at)
        .bind(stage.updated_at)
        .fetch_one(db.pool())
        .await?;

        Ok(row)
    }

    /// Fetch a stage by its slug.
    pub async fn get_by_slug(db: &Database, slug: &str) -> Result<StageModel> {
        let row = sqlx::query_as::<_, StageModel>(r#"SELECT * FROM stages WHERE slug = $1"#)
            .bind(slug)
            .fetch_one(db.pool())
            .await?;

        Ok(row)
    }

    /// Fetch a stage by its internal ID.
    pub async fn get_by_id(db: &Database, id: Uuid) -> Result<StageModel> {
        let row = sqlx::query_as::<_, StageModel>(r#"SELECT * FROM stages WHERE id = $1"#)
            .bind(id)
            .fetch_one(db.pool())
            .await?;

        Ok(row)
    }

    /// Find all stages for a course (including extensions)
    pub async fn find_by_course(db: &Database, course_slug: &str) -> Result<Vec<StageModel>> {
        let rows = sqlx::query_as::<_, StageModel>(
            r#"SELECT s.* FROM stages s
                JOIN courses c ON s.course_id = c.id
                WHERE c.slug = $1
                ORDER BY s.created_at ASC"#,
        )
        .bind(course_slug)
        .fetch_all(db.pool())
        .await?;

        Ok(rows)
    }

    /// Find only base stages for a course (excluding extensions).
    pub async fn find_base_by_course(db: &Database, course_slug: &str) -> Result<Vec<StageModel>> {
        let rows = sqlx::query_as::<_, StageModel>(
            r#"SELECT s.* FROM stages s
                JOIN courses c ON s.course_id = c.id
                WHERE c.slug = $1 AND s.extension_id IS NULL
                ORDER BY s.created_at ASC"#,
        )
        .bind(course_slug)
        .fetch_all(db.pool())
        .await?;

        Ok(rows)
    }

    /// Find stages for a specific extension.
    pub async fn find_by_extension(db: &Database, extension_slug: &str) -> Result<Vec<StageModel>> {
        let rows = sqlx::query_as::<_, StageModel>(
            r#"SELECT s.* FROM stages s
                JOIN extensions e ON s.extension_id = e.id
                WHERE e.slug = $1
                ORDER BY s.created_at ASC"#,
        )
        .bind(extension_slug)
        .fetch_all(db.pool())
        .await?;

        Ok(rows)
    }

    /// Update a stage in the database.
    pub async fn update(db: &Database, stage: &StageModel) -> Result<StageModel> {
        let row = sqlx::query_as::<_, StageModel>(
            r#"
            UPDATE stages
            SET course_id = $2, extension_id = $3, name = $4, difficulty = $5, description = $6, instruction = $7, updated_at = $8
            WHERE slug = $1
            RETURNING *
            "#,
        )
        .bind(&stage.slug)
        .bind(stage.course_id)
        .bind(stage.extension_id)
        .bind(&stage.name)
        .bind(&stage.difficulty)
        .bind(&stage.description)
        .bind(&stage.instruction)
        .bind(stage.updated_at)
        .fetch_one(db.pool())
        .await?;

        Ok(row)
    }

    /// Delete a stage by its slug.
    pub async fn delete(db: &Database, slug: &str) -> Result<()> {
        sqlx::query(r#"DELETE FROM stages WHERE slug = $1"#).bind(slug).execute(db.pool()).await?;

        Ok(())
    }
}
