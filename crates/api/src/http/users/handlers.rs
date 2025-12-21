use crate::http::ApiResponse;
use crate::http::AppError;
use crate::http::state::AppState;
use crate::http::users::dtos::{CreateUserRequest, UserResponse};
use application::use_cases::CreateUserInput;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = crate::http::ApiResponseUser),
        (status = 400, description = "Invalid request payload", body = crate::http::ApiErrorResponse),
        (status = 409, description = "User already exists", body = crate::http::ApiErrorResponse),
        (status = 500, description = "Internal server error", body = crate::http::ApiErrorResponse)
    )
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;

    let input = CreateUserInput {
        first_name: payload.first_name,
        last_name: payload.last_name,
        email: payload.email,
        phone_number: payload.phone_number,
        password: payload.password,
        birth_date: payload.birth_date,
    };

    let user = state.create_user_use_case.execute(input).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(UserResponse::from(user))),
    ))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = crate::http::ApiResponseUser),
        (status = 404, description = "User not found", body = crate::http::ApiErrorResponse),
        (status = 500, description = "Internal server error", body = crate::http::ApiErrorResponse)
    )
)]
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.get_user_use_case.execute(id).await?;

    match user {
        Some(user) => Ok((
            StatusCode::OK,
            Json(ApiResponse::success(UserResponse::from(user))),
        )),
        None => Err(AppError::NotFound(format!("User with ID {} not found", id))),
    }
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List of users", body = Vec<crate::http::ApiResponseUser>),
        (status = 500, description = "Internal server error", body = crate::http::ApiErrorResponse)
    )
)]
pub async fn list_users(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let users = state.list_users_use_case.execute().await?;

    let response: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(response)),
    ))
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 404, description = "User not found", body = crate::http::ApiErrorResponse),
        (status = 500, description = "Internal server error", body = crate::http::ApiErrorResponse)
    )
)]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.delete_user_use_case.execute(id).await?;

    Ok(StatusCode::NO_CONTENT)
}
