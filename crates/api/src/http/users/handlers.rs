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

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = crate::http::ApiResponseUser),
        (status = 409, description = "User already exists", body = crate::http::ApiErrorResponse),
        (status = 500, description = "Internal server error", body = crate::http::ApiErrorResponse)
    )
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let input = CreateUserInput {
        name: payload.name,
        email: payload.email,
        phone_number: payload.phone_number,
        password: payload.password,
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
