use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiErrorResponse {
    #[schema(example = false, default = false)]
    pub success: bool,
    pub error: ApiErrorDetail,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiErrorDetail {
    pub code: String,
    pub message: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    BadRequest(String),
    NotFound(String),
    Conflict(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg),
            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR",
                msg,
            ),
        };

        let body = Json(ApiErrorResponse {
            success: false,
            error: ApiErrorDetail {
                code: code.to_string(),
                message,
            },
        });

        (status, body).into_response()
    }
}

impl From<application::use_cases::CreateUserError> for AppError {
    fn from(err: application::use_cases::CreateUserError) -> Self {
        match err {
            application::use_cases::CreateUserError::AlreadyExists(email) => {
                Self::Conflict(format!("User with email {} already exists", email))
            }
            application::use_cases::CreateUserError::RepositoryError(msg) => {
                tracing::error!(error = %msg, "Repository error");
                Self::Internal("Internal server error".to_string())
            }
            application::use_cases::CreateUserError::InternalError(msg) => {
                tracing::error!(error = %msg, "Internal application error");
                Self::Internal("Internal server error".to_string())
            }
        }
    }
}

impl From<application::use_cases::GetUserError> for AppError {
    fn from(err: application::use_cases::GetUserError) -> Self {
        match err {
            application::use_cases::GetUserError::RepositoryError(msg) => {
                tracing::error!(error = %msg, "Repository error");
                Self::Internal("Internal server error".to_string())
            }
        }
    }
}
