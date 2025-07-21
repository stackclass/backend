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

use std::{convert::Infallible, sync::Arc};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{
        IntoResponse, Sse,
        sse::{Event, KeepAlive},
    },
};
use futures::{Stream, StreamExt};
use tokio_stream::wrappers::ReceiverStream;
use tracing::{error, info};

use crate::{
    context::Context,
    errors::Result,
    extractor::Claims,
    request::{CreateCourseRequest, CreateUserCourseRequest, UpdateUserCourseRequest},
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
        content = CreateCourseRequest,
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
    security(("JWTBearerAuth" = [])),
    tags = ["User", "Course"]
)]
pub async fn find_user_courses(
    claims: Claims,
    State(ctx): State<Arc<Context>>,
) -> Result<impl IntoResponse> {
    Ok((StatusCode::OK, Json(CourseService::find_user_courses(ctx, &claims.id).await?)))
}

/// Enroll the current user in a course.
#[utoipa::path(
    operation_id = "enroll-user-in-course",
    post, path = "/v1/user/courses",
    request_body(
        content = CreateUserCourseRequest,
        description = "Enroll user in course request",
        content_type = "application/json"
    ),
    responses(
        (status = 201, description = "User enrolled in course successfully", body = UserCourseResponse),
        (status = 404, description = "Course not found"),
        (status = 500, description = "Failed to enroll user in course")
    ),
    security(("JWTBearerAuth" = [])),
    tags = ["User", "Course"]
)]
pub async fn create_user_course(
    claims: Claims,
    State(ctx): State<Arc<Context>>,
    Json(req): Json<CreateUserCourseRequest>,
) -> Result<impl IntoResponse> {
    let res = CourseService::create_user_course(ctx, &claims.id, &req).await?;
    Ok((StatusCode::CREATED, Json(res)))
}

/// Get the course detail for the current user.
#[utoipa::path(
    operation_id = "get-user-course-detail",
    get, path = "/v1/user/courses/{slug}",
    params(
        ("slug" = String, description = "The slug of course"),
    ),
    responses(
        (status = 200, description = "Course retrieved successfully", body = UserCourseResponse),
        (status = 404, description = "Course not found"),
        (status = 500, description = "Failed to get course")
    ),
    security(("JWTBearerAuth" = [])),
    tags = ["User", "Course"]
)]
pub async fn get_user_course(
    claims: Claims,
    State(ctx): State<Arc<Context>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse> {
    Ok((StatusCode::OK, Json(CourseService::get_user_course(ctx, &claims.id, &slug).await?)))
}

/// Update this course for the current user.
#[utoipa::path(
    operation_id = "update-user-course",
    patch, path = "/v1/user/courses/{slug}",
    params(
        ("slug" = String, description = "The slug of course"),
    ),
    request_body(
        content = UpdateUserCourseRequest,
        description = "Update this course for the current user",
        content_type = "application/json"
    ),
    responses(
        (status = 204, description = "User course updated successfully"),
        (status = 404, description = "Course not found"),
        (status = 500, description = "Failed to enroll user in course")
    ),
    security(("JWTBearerAuth" = [])),
    tags = ["User", "Course"]
)]
pub async fn update_user_course(
    claims: Claims,
    State(ctx): State<Arc<Context>>,
    Path(slug): Path<String>,
    Json(req): Json<UpdateUserCourseRequest>,
) -> Result<impl IntoResponse> {
    CourseService::update_user_course(ctx, &claims.id, &slug, &req).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Stream the status of a specific course for the current user.
#[utoipa::path(
    operation_id = "stream_user_course_status",
    get, path = "/v1/user/courses/{slug}/status",
    params(
        ("slug" = String, description = "The slug of course")
    ),
    responses(
        (status = 200, description = "Successfully started streaming course status updates"),
        (status = 404, description = "Course or stage not found"),
        (status = 500, description = "Failed to stream course status")
    ),
    security(("JWTBearerAuth" = [])),
    tags = ["User", "Course"]
)]
pub async fn stream_user_course_status(
    claims: Claims,
    State(ctx): State<Arc<Context>>,
    Path(slug): Path<String>,
) -> Sse<impl Stream<Item = axum::response::Result<Event, Infallible>>> {
    info!("Starting to stream status updates for course {} for user {}...", slug, claims.id);

    // Create a channel for sending status updates.
    let (sender, receiver) = tokio::sync::mpsc::channel(100);

    // Spawn a background task to fetch and send status updates.
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            let status = CourseService::get_user_course(ctx.clone(), &claims.id, &slug).await;
            if let Ok(status) = status {
                let event = Event::default().json_data(status).unwrap_or_else(|e| {
                    error!("Failed to serialize status update: {}", e);
                    Event::default().data("status update error")
                });
                if sender.send(event).await.is_err() {
                    break;
                }
            }
        }
    });

    // Convert the receiver into a stream.
    let stream = ReceiverStream::new(receiver);
    let stream = stream.map(Ok);

    // Return the SSE stream with keep-alive.
    Sse::new(stream).keep_alive(KeepAlive::default())
}
