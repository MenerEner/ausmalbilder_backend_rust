use crate::http::AppError;
use crate::http::state::AppState;
use crate::http::users::dtos::{
    CreateUserRequest, ForgotPasswordRequest, PaginationParams, ResetPasswordRequest, UserResponse,
    VerifyEmailRequest,
};
use crate::http::{ApiResponse, PaginatedResponse};
use application::use_cases::{CreateUserInput, SignupInput};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
};
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/users",
    tag = "users",
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
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let input = CreateUserInput {
        first_name: payload.first_name,
        last_name: payload.last_name,
        email: payload.email,
        phone_number: payload.phone_number,
        password: payload.password,
        birth_date: payload.birth_date,
    };

    let user = state.create_user_use_case.execute(input).await?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::LOCATION,
        format!("/users/{}", user.id).parse().unwrap(),
    );

    Ok((
        StatusCode::CREATED,
        headers,
        Json(ApiResponse::success(UserResponse::from(user))),
    ))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "users",
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
    tag = "users",
    params(
        PaginationParams
    ),
    responses(
        (status = 200, description = "List of users", body = PaginatedResponse<UserResponse>),
        (status = 500, description = "Internal server error", body = crate::http::ApiErrorResponse)
    )
)]
pub async fn list_users(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationParams>,
) -> Result<impl IntoResponse, AppError> {
    pagination
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let (users, total) = state
        .list_users_use_case
        .execute(pagination.page(), pagination.page_size())
        .await?;

    let user_responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

    Ok((
        StatusCode::OK,
        Json(PaginatedResponse::success(
            user_responses,
            pagination.page(),
            pagination.page_size(),
            total,
        )),
    ))
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "users",
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

#[utoipa::path(
    post,
    path = "/auth/signup",
    tag = "users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User signed up successfully", body = crate::http::ApiResponseUser),
        (status = 400, description = "Invalid request payload", body = crate::http::ApiErrorResponse),
        (status = 409, description = "User already exists", body = crate::http::ApiErrorResponse),
        (status = 500, description = "Internal server error", body = crate::http::ApiErrorResponse)
    )
)]
pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let input = SignupInput {
        first_name: payload.first_name,
        last_name: payload.last_name,
        email: payload.email,
        phone_number: payload.phone_number,
        password: payload.password,
        birth_date: payload.birth_date,
    };

    let user = state.signup_use_case.execute(input).await?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::LOCATION,
        format!("/users/{}", user.id).parse().unwrap(),
    );

    Ok((
        StatusCode::CREATED,
        headers,
        Json(ApiResponse::success(UserResponse::from(user))),
    ))
}

#[utoipa::path(
    post,
    path = "/auth/verify-email",
    tag = "users",
    request_body = VerifyEmailRequest,
    responses(
        (status = 204, description = "Email verified successfully"),
        (status = 400, description = "Invalid token", body = crate::http::ApiErrorResponse),
        (status = 500, description = "Internal server error", body = crate::http::ApiErrorResponse)
    )
)]
pub async fn verify_email(
    State(state): State<AppState>,
    Json(payload): Json<VerifyEmailRequest>,
) -> Result<impl IntoResponse, AppError> {
    state.verify_email_use_case.execute(&payload.token).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/auth/forgot-password",
    tag = "users",
    request_body = ForgotPasswordRequest,
    responses(
        (status = 202, description = "Password reset email sent (accepted)"),
        (status = 400, description = "Invalid request payload", body = crate::http::ApiErrorResponse),
        (status = 500, description = "Internal server error", body = crate::http::ApiErrorResponse)
    )
)]
pub async fn forgot_password(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // We use 202 Accepted to avoid user enumeration if we want,
    // but here we just follow the use case.
    // If user is not found, we still return 202 or handled error.
    match state
        .request_password_reset_use_case
        .execute(&payload.email)
        .await
    {
        Ok(_) => Ok(StatusCode::ACCEPTED),
        // If user not found, we might want to return 202 anyway to avoid enumeration
        Err(application::use_cases::RequestPasswordResetError::UserNotFound) => {
            Ok(StatusCode::ACCEPTED)
        }
        Err(e) => Err(e.into()),
    }
}

#[utoipa::path(
    post,
    path = "/auth/reset-password",
    tag = "users",
    request_body = ResetPasswordRequest,
    responses(
        (status = 204, description = "Password reset successfully"),
        (status = 400, description = "Invalid or expired token", body = crate::http::ApiErrorResponse),
        (status = 500, description = "Internal server error", body = crate::http::ApiErrorResponse)
    )
)]
pub async fn reset_password(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    state
        .reset_password_use_case
        .execute(&payload.token, &payload.new_password)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
