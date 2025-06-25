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

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    context::Context,
    errors::Result,
    request::CreateCourseRequest,
    response::{CourseDetailResponse, CourseResponse, UserCourseResponse},
    service::CourseService,
};

// The Course Service Handlers.

/// Find all courses.
#[utoipa::path(
    operation_id = "find-all-courses",
    get, path = "/v1/courses",
    responses(
        (status = 200, description = "Courses retrieved successfully", body = Vec<CourseResponse>),
    ),
    tag = "Course"
)]
pub async fn find(State(ctx): State<Arc<Context>>) -> Result<impl IntoResponse> {
    Ok((StatusCode::OK, Json(CourseService::find(ctx).await?)))
}

/// Create a course.
#[utoipa::path(
    operation_id = "create-course",
    post, path = "/v1/courses",
    request_body(
        content = inline(CreateCourseRequest),
        description = "Create course request",
        content_type = "application/json"
    ),
    responses(
        (status = 201, description = "Course created successfully", body = CourseResponse),
        (status = 500, description = "Failed to create course")
    ),
    tag = "Course"
)]
pub async fn create(
    State(ctx): State<Arc<Context>>,
    Json(req): Json<CreateCourseRequest>,
) -> Result<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(CourseService::create(ctx, &req.repository).await?)))
}

/// Get a course.
#[utoipa::path(
    operation_id = "get-course-detail",
    get, path = "/v1/courses/{slug}",
    params(
        ("slug" = String, description = "The slug of course"),
    ),
    responses(
        (status = 200, description = "Course retrieved successfully", body = CourseDetailResponse),
        (status = 404, description = "Course not found"),
        (status = 500, description = "Failed to get course")
    ),
    tag = "Course"
)]
pub async fn get(
    State(ctx): State<Arc<Context>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse> {
    Ok((StatusCode::OK, Json(CourseService::get(ctx, &slug).await?)))
}

/// Delete a course.
#[utoipa::path(
     operation_id = "delete-course",
    delete, path = "/v1/courses/{slug}",
    params(
        ("slug" = String, description = "The slug of course"),
    ),
    responses(
        (status = 204, description = "Course deleted successfully"),
        (status = 404, description = "Course not found"),
        (status = 500, description = "Failed to delete course")
    ),
    tag = "Course"
)]
pub async fn delete(
    State(ctx): State<Arc<Context>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse> {
    CourseService::delete(ctx, &slug).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Update course from git repository
#[utoipa::path(
    operation_id = "update-course",
    patch, path = "/v1/courses/{slug}",
    params(
        ("slug" = String, description = "The slug of course"),
    ),
    responses(
        (status = 204, description = "Course updated successfully"),
        (status = 404, description = "Course not found"),
        (status = 500, description = "Failed to update course")
    ),
    tag = "Course"
)]
pub async fn update(
    State(ctx): State<Arc<Context>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse> {
    CourseService::update(ctx, &slug).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Find all courses for the current user.
#[utoipa::path(
    operation_id = "find-user-courses",
    get, path = "/v1/user/courses",
    responses(
        (status = 200, description = "User courses retrieved successfully", body = Vec<UserCourseResponse>),
    ),
    tags = ["User", "Course"]
)]
pub async fn find_user_courses(State(ctx): State<Arc<Context>>) -> Result<impl IntoResponse> {
    Ok((StatusCode::OK, Json(CourseService::find_user_courses(ctx, "<user_id>").await?)))
}
