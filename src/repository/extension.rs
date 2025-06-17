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

use crate::{
    database::{Database, Transaction},
    errors::Result,
    model::ExtensionModel,
};

/// Repository for managing extensions in the database.
pub struct ExtensionRepository;

impl ExtensionRepository {
    /// Create a new extension in the database.
    pub async fn create(tx: &mut Transaction<'_>, ext: &ExtensionModel) -> Result<ExtensionModel> {
        let row = sqlx::query_as::<_, ExtensionModel>(
            r#"
            INSERT INTO extensions (
                id, course_id, slug, name, description, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(ext.id)
        .bind(ext.course_id)
        .bind(&ext.slug)
        .bind(&ext.name)
        .bind(&ext.description)
        .bind(ext.created_at)
        .bind(ext.updated_at)
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    /// Fetch an extension by its slug.
    pub async fn get_by_slug(db: &Database, slug: &str) -> Result<ExtensionModel> {
        let row =
            sqlx::query_as::<_, ExtensionModel>(r#"SELECT * FROM extensions WHERE slug = $1"#)
                .bind(slug)
                .fetch_one(db.pool())
                .await?;

        Ok(row)
    }

    /// Fetch an extension by its internal ID.
    pub async fn get_by_id(db: &Database, id: Uuid) -> Result<ExtensionModel> {
        let row = sqlx::query_as::<_, ExtensionModel>(r#"SELECT * FROM extensions WHERE id = $1"#)
            .bind(id)
            .fetch_one(db.pool())
            .await?;

        Ok(row)
    }

    /// Find all extensions for a course.
    pub async fn find_by_course(db: &Database, course_slug: &str) -> Result<Vec<ExtensionModel>> {
        let rows = sqlx::query_as::<_, ExtensionModel>(
            r#"
            SELECT e.* FROM extensions e
            JOIN courses c ON e.course_id = c.id
            WHERE c.slug = $1
            ORDER BY e.created_at DESC
            "#,
        )
        .bind(course_slug)
        .fetch_all(db.pool())
        .await?;

        Ok(rows)
    }

    /// Update an extension in the database.
    pub async fn update(db: &Database, extension: &ExtensionModel) -> Result<ExtensionModel> {
        let row = sqlx::query_as::<_, ExtensionModel>(
            r#"
            UPDATE extensions
            SET course_id = $2, name = $3, description = $4, updated_at = $5
            WHERE slug = $1
            RETURNING *
            "#,
        )
        .bind(&extension.slug)
        .bind(extension.course_id)
        .bind(&extension.name)
        .bind(&extension.description)
        .bind(extension.updated_at)
        .fetch_one(db.pool())
        .await?;

        Ok(row)
    }

    /// Delete an extension by its slug.
    pub async fn delete(db: &Database, slug: &str) -> Result<()> {
        sqlx::query(r#"DELETE FROM extensions WHERE slug = $1"#)
            .bind(slug)
            .execute(db.pool())
            .await?;

        Ok(())
    }
}
