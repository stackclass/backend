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

use tracing::debug;
use uuid::Uuid;

use crate::{
    database::{Database, Transaction},
    model::{CourseModel, UserCourseModel},
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
                id, slug, name, short_name, release_status, description, summary, repository, logo, stage_count, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
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
        .bind(&course.logo)
        .bind(course.stage_count)
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
    pub(crate) async fn find(db: &Database) -> Result<Vec<CourseModel>> {
        let rows = sqlx::query_as::<_, CourseModel>(r#"SELECT * FROM courses"#)
            .fetch_all(db.pool())
            .await?;

        Ok(rows)
    }

    /// Update a course in the database.
    pub async fn update(tx: &mut Transaction<'_>, course: &CourseModel) -> Result<CourseModel> {
        let row = sqlx::query_as::<_, CourseModel>(
            r#"
            UPDATE courses
            SET name = $2, short_name = $3, release_status = $4, description = $5, summary = $6, stage_count = $7, updated_at = $8
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
        .bind(course.stage_count)
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

    /// Find all courses for the current user.
    pub async fn find_user_courses(db: &Database, user_id: &str) -> Result<Vec<UserCourseModel>> {
        let rows = sqlx::query_as::<_, UserCourseModel>(
            r#"
            SELECT
                uc.*,
                c.slug AS course_slug,
                s.slug AS current_stage_slug
            FROM user_courses uc
            LEFT JOIN courses c ON uc.course_id = c.id
            LEFT JOIN stages s ON uc.current_stage_id = s.id
            WHERE uc.user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_all(db.pool())
        .await?;

        Ok(rows)
    }

    /// Find the course detail for the current user.
    pub async fn get_user_course(
        db: &Database,
        user_id: &str,
        course_slug: &str,
    ) -> Result<UserCourseModel> {
        let row = sqlx::query_as::<_, UserCourseModel>(
            r#"
            SELECT
                uc.*,
                c.slug AS course_slug,
                s.slug AS current_stage_slug
            FROM user_courses uc
            LEFT JOIN courses c ON uc.course_id = c.id
            LEFT JOIN stages s ON uc.current_stage_id = s.id
            WHERE uc.user_id = $1 AND c.slug = $2
            "#,
        )
        .bind(user_id)
        .bind(course_slug)
        .fetch_one(db.pool())
        .await?;

        Ok(row)
    }

    /// Find the course detail by its internal ID.
    pub async fn get_user_course_by_id(db: &Database, id: Uuid) -> Result<UserCourseModel> {
        let row = sqlx::query_as::<_, UserCourseModel>(
            r#"
            SELECT
                uc.*,
                c.slug AS course_slug,
                s.slug AS current_stage_slug
            FROM user_courses uc
            LEFT JOIN courses c ON uc.course_id = c.id
            LEFT JOIN stages s ON uc.current_stage_id = s.id
            WHERE uc.id = $1
            "#,
        )
        .bind(id)
        .fetch_one(db.pool())
        .await?;

        Ok(row)
    }

    /// Create a new user course enrollment.
    pub async fn create_user_course(
        tx: &mut Transaction<'_>,
        user_course: &UserCourseModel,
    ) -> Result<UserCourseModel> {
        debug!(
            "Creating new user course enrollment for user {} and course {}",
            &user_course.user_id, user_course.course_id
        );

        let row = sqlx::query_as::<_, UserCourseModel>(
            r#"
            WITH inserted AS (
                INSERT INTO user_courses (
                    id, user_id, course_id, started_at, current_stage_id, completed_stage_count, proficiency, cadence, accountability, activated
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                RETURNING *
            )
            SELECT
                i.*,
                c.slug AS course_slug,
                s.slug AS current_stage_slug
            FROM inserted i
            JOIN courses c ON i.course_id = c.id
            LEFT JOIN stages s ON i.current_stage_id = s.id
            "#,
        )
        .bind(user_course.id)
        .bind(&user_course.user_id)
        .bind(user_course.course_id)
        .bind(user_course.started_at)
        .bind(user_course.current_stage_id)
        .bind(user_course.completed_stage_count)
        .bind(&user_course.proficiency)
        .bind(&user_course.cadence)
        .bind(user_course.accountability)
        .bind(user_course.activated)
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    /// Update a user course in the database.
    pub async fn update_user_course(
        tx: &mut Transaction<'_>,
        user_course: &UserCourseModel,
    ) -> Result<UserCourseModel> {
        let row = sqlx::query_as::<_, UserCourseModel>(
            r#"
            WITH updated AS (
                UPDATE user_courses
                SET
                    current_stage_id = $2,
                    completed_stage_count = $3,
                    proficiency = $4,
                    cadence = $5,
                    accountability = $6,
                    activated = $7
                WHERE id = $1
                RETURNING *
            )
            SELECT
                u.*,
                c.slug AS course_slug,
                s.slug AS current_stage_slug
            FROM updated u
            LEFT JOIN courses c ON u.course_id = c.id
            LEFT JOIN stages s ON u.current_stage_id = s.id
            "#,
        )
        .bind(user_course.id)
        .bind(user_course.current_stage_id)
        .bind(user_course.completed_stage_count)
        .bind(&user_course.proficiency)
        .bind(&user_course.cadence)
        .bind(user_course.accountability)
        .bind(user_course.activated)
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }
}
