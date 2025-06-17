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
    model::CourseModel,
    repository::Result,
};

/// Repository for managing courses in the database.
pub struct CourseRepository;

impl CourseRepository {
    /// Create a new course in the database.
    pub async fn create(tx: &mut Transaction<'_>, course: &CourseModel) -> Result<CourseModel> {
        let row = sqlx::query_as::<_, CourseModel>(
            r#"
            INSERT INTO courses (
                id, slug, name, short_name, release_status, description, summary, repository, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
        )
        .bind(course.id)
        .bind(&course.slug)
        .bind(&course.name)
        .bind(&course.short_name)
        .bind(&course.release_status)
        .bind(&course.description)
        .bind(&course.summary)
        .bind(&course.repository)
        .bind(course.created_at)
        .bind(course.updated_at)
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    /// Fetch a course by its slug (external identifier).
    pub async fn get_by_slug(db: &Database, slug: &str) -> Result<CourseModel> {
        let row = sqlx::query_as::<_, CourseModel>(r#"SELECT * FROM courses WHERE slug = $1"#)
            .bind(slug)
            .fetch_one(db.pool())
            .await?;

        Ok(row)
    }

    /// Fetch a course by its internal ID.
    pub async fn get_by_id(db: &Database, id: Uuid) -> Result<CourseModel> {
        let row = sqlx::query_as::<_, CourseModel>(r#"SELECT * FROM courses WHERE id = $1"#)
            .bind(id)
            .fetch_one(db.pool())
            .await?;

        Ok(row)
    }

    /// Find all courses
    pub(crate) async fn find(database: &Database) -> Result<Vec<CourseModel>> {
        let rows = sqlx::query_as::<_, CourseModel>(r#"SELECT * FROM courses"#)
            .fetch_all(database.pool())
            .await?;

        Ok(rows)
    }

    /// Update a course in the database.
    pub async fn update(tx: &mut Transaction<'_>, course: &CourseModel) -> Result<CourseModel> {
        let row = sqlx::query_as::<_, CourseModel>(
            r#"
            UPDATE courses
            SET name = $2, short_name = $3, release_status = $4, description = $5, summary = $6, updated_at = $7
            WHERE slug = $1
            RETURNING *
            "#,
        )
        .bind(&course.slug)
        .bind(&course.name)
        .bind(&course.short_name)
        .bind(&course.release_status)
        .bind(&course.description)
        .bind(&course.summary)
        .bind(course.updated_at)
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    /// Delete a course by its slug.
    pub async fn delete(db: &Database, slug: &str) -> Result<()> {
        sqlx::query(r#"DELETE FROM courses WHERE slug = $1"#).bind(slug).execute(db.pool()).await?;

        Ok(())
    }
}
