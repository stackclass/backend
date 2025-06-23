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

use sqlx::Error;
use tracing::debug;
use uuid::Uuid;

use crate::{
    database::{Database, Transaction},
    model::StageModel,
    repository::Result,
};

/// Repository for managing stages in the database.
pub struct StageRepository;

impl StageRepository {
    /// Create a new stage in the database.
    pub async fn create(tx: &mut Transaction<'_>, stage: &StageModel) -> Result<StageModel> {
        debug!("Creating stage with slug: {}", stage.slug);

        let row = sqlx::query_as::<_, StageModel>(
            r#"
            WITH inserted_stage AS (
                INSERT INTO stages (
                    id, course_id, extension_id, slug, name, difficulty, description, instruction, weight, created_at, updated_at
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                RETURNING *
            )
            SELECT s.*, e.slug as extension_slug
            FROM inserted_stage s
            LEFT JOIN extensions e ON s.extension_id = e.id
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
        .bind(stage.weight)
        .bind(stage.created_at)
        .bind(stage.updated_at)
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    /// Fetch a stage by course slug and its slug.
    pub async fn get_by_slug(
        db: &Database,
        course_slug: &str,
        stage_slug: &str,
    ) -> Result<StageModel> {
        let row = sqlx::query_as::<_, StageModel>(
            r#"
            SELECT s.*, e.slug as extension_slug FROM stages s
            JOIN courses c ON s.course_id = c.id
            LEFT JOIN extensions e ON s.extension_id = e.id
            WHERE c.slug = $1 AND s.slug = $2
            "#,
        )
        .bind(course_slug)
        .bind(stage_slug)
        .fetch_one(db.pool())
        .await?;

        Ok(row)
    }

    /// Fetch a stage by its internal ID.
    pub async fn get_by_id(db: &Database, id: Uuid) -> Result<StageModel> {
        let row = sqlx::query_as::<_, StageModel>(
            r#"
            SELECT s.*, e.slug as extension_slug FROM stages s
            LEFT JOIN extensions e ON s.extension_id = e.id
            WHERE s.id = $1
            "#,
        )
        .bind(id)
        .fetch_one(db.pool())
        .await?;

        Ok(row)
    }

    /// Find all stages for a course (including extensions)
    pub async fn find_by_course(db: &Database, course_slug: &str) -> Result<Vec<StageModel>> {
        let rows = sqlx::query_as::<_, StageModel>(
            r#"
            SELECT s.*, e.slug as extension_slug
            FROM stages s
            JOIN courses c ON s.course_id = c.id
            LEFT JOIN extensions e ON s.extension_id = e.id
            WHERE c.slug = $1
            ORDER BY
                s.extension_id IS NULL DESC,
                e.weight ASC NULLS FIRST,
                s.weight ASC
            "#,
        )
        .bind(course_slug)
        .fetch_all(db.pool())
        .await?;

        Ok(rows)
    }

    /// Find only base stages for a course (excluding extensions).
    pub async fn find_base_by_course(db: &Database, course_slug: &str) -> Result<Vec<StageModel>> {
        let rows = sqlx::query_as::<_, StageModel>(
            r#"
            SELECT s.*, e.slug as extension_slug FROM stages s
            JOIN courses c ON s.course_id = c.id
            LEFT JOIN extensions e ON s.extension_id = e.id
            WHERE c.slug = $1 AND s.extension_id IS NULL
            ORDER BY s.weight ASC
            "#,
        )
        .bind(course_slug)
        .fetch_all(db.pool())
        .await?;

        Ok(rows)
    }

    /// Find only extended stages for a course.
    pub async fn find_extended_by_course(
        db: &Database,
        course_slug: &str,
    ) -> Result<Vec<StageModel>> {
        let rows = sqlx::query_as::<_, StageModel>(
            r#"
            SELECT s.*, e.slug as extension_slug
            FROM stages s
            JOIN courses c ON s.course_id = c.id
            JOIN extensions e ON s.extension_id = e.id
            WHERE c.slug = $1 AND s.extension_id IS NOT NULL
            ORDER BY
                e.weight,
                s.weight
            "#,
        )
        .bind(course_slug)
        .fetch_all(db.pool())
        .await?;

        Ok(rows)
    }

    /// Find stages for a specific extension.
    pub async fn find_by_extension(db: &Database, extension_slug: &str) -> Result<Vec<StageModel>> {
        let rows = sqlx::query_as::<_, StageModel>(
            r#"
            SELECT s.*, e.slug as extension_slug
            FROM stages s
            JOIN extensions e ON s.extension_id = e.id
            WHERE e.slug = $1
            ORDER BY s.weight ASC
            "#,
        )
        .bind(extension_slug)
        .fetch_all(db.pool())
        .await?;

        Ok(rows)
    }

    /// Update a stage in the database.
    pub async fn update(tx: &mut Transaction<'_>, stage: &StageModel) -> Result<StageModel> {
        debug!("Updating stage with slug: {}", stage.slug);

        let row = sqlx::query_as::<_, StageModel>(
            r#"
            WITH updated_stage AS (
                UPDATE stages
                SET course_id = $2, extension_id = $3, name = $4, difficulty = $5, description = $6, instruction = $7, weight = $8, updated_at = $9
                WHERE slug = $1
                RETURNING *
            )
            SELECT s.*, e.slug as extension_slug
            FROM updated_stage s
            LEFT JOIN extensions e ON s.extension_id = e.id
            "#,
        )
        .bind(&stage.slug)
        .bind(stage.course_id)
        .bind(stage.extension_id)
        .bind(&stage.name)
        .bind(&stage.difficulty)
        .bind(&stage.description)
        .bind(&stage.instruction)
        .bind(stage.weight)
        .bind(stage.updated_at)
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    /// Updates an existing record or inserts a new one if it doesn't exist.
    pub async fn upsert(tx: &mut Transaction<'_>, stage: &StageModel) -> Result<StageModel> {
        match Self::update(tx, stage).await {
            Ok(stage) => Ok(stage),
            Err(Error::RowNotFound) => Self::create(tx, stage).await,
            Err(e) => Err(e),
        }
    }

    /// Delete a stage by its slug.
    pub async fn delete(tx: &mut Transaction<'_>, slug: &str) -> Result<()> {
        debug!("Deleting stage with slug: {}", slug);
        sqlx::query(r#"DELETE FROM stages WHERE slug = $1"#).bind(slug).execute(&mut **tx).await?;

        Ok(())
    }
}
