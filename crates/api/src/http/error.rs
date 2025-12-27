use crate::http::middleware::get_correlation_id;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiErrorResponse {
    pub error: ApiErrorDetail,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiErrorDetail {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
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
            error: ApiErrorDetail {
                code: code.to_string(),
                message,
                correlation_id: get_correlation_id(),
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

impl From<application::use_cases::DeleteUserError> for AppError {
    fn from(err: application::use_cases::DeleteUserError) -> Self {
        match err {
            application::use_cases::DeleteUserError::NotFound(id) => {
                Self::NotFound(format!("User with ID {} not found", id))
            }
            application::use_cases::DeleteUserError::RepositoryError(msg) => {
                tracing::error!(error = %msg, "Repository error");
                Self::Internal("Internal server error".to_string())
            }
        }
    }
}

impl From<application::use_cases::ListUsersError> for AppError {
    fn from(err: application::use_cases::ListUsersError) -> Self {
        match err {
            application::use_cases::ListUsersError::RepositoryError(msg) => {
                tracing::error!(error = %msg, "Repository error");
                Self::Internal("Internal server error".to_string())
            }
        }
    }
}

impl From<application::use_cases::SignupError> for AppError {
    fn from(err: application::use_cases::SignupError) -> Self {
        match err {
            application::use_cases::SignupError::AlreadyExists(email) => {
                Self::Conflict(format!("User with email {} already exists", email))
            }
            application::use_cases::SignupError::RepositoryError(msg) => {
                tracing::error!(error = %msg, "Repository error");
                Self::Internal("Internal server error".to_string())
            }
            application::use_cases::SignupError::EmailError(msg) => {
                tracing::error!(error = %msg, "Email error");
                Self::Internal("Internal server error".to_string())
            }
            application::use_cases::SignupError::InternalError(msg) => {
                tracing::error!(error = %msg, "Internal error");
                Self::Internal("Internal server error".to_string())
            }
        }
    }
}

impl From<application::use_cases::VerifyEmailError> for AppError {
    fn from(err: application::use_cases::VerifyEmailError) -> Self {
        match err {
            application::use_cases::VerifyEmailError::InvalidToken => {
                Self::BadRequest("Invalid or expired token".to_string())
            }
            application::use_cases::VerifyEmailError::UserNotFound => {
                Self::NotFound("User not found".to_string())
            }
            application::use_cases::VerifyEmailError::RepositoryError(msg) => {
                tracing::error!(error = %msg, "Repository error");
                Self::Internal("Internal server error".to_string())
            }
        }
    }
}

impl From<application::use_cases::RequestPasswordResetError> for AppError {
    fn from(err: application::use_cases::RequestPasswordResetError) -> Self {
        match err {
            application::use_cases::RequestPasswordResetError::UserNotFound => {
                Self::NotFound("User not found".to_string())
            }
            application::use_cases::RequestPasswordResetError::RepositoryError(msg) => {
                tracing::error!(error = %msg, "Repository error");
                Self::Internal("Internal server error".to_string())
            }
            application::use_cases::RequestPasswordResetError::EmailError(msg) => {
                tracing::error!(error = %msg, "Email error");
                Self::Internal("Internal server error".to_string())
            }
        }
    }
}

impl From<application::use_cases::ResetPasswordError> for AppError {
    fn from(err: application::use_cases::ResetPasswordError) -> Self {
        match err {
            application::use_cases::ResetPasswordError::InvalidToken => {
                Self::BadRequest("Invalid token".to_string())
            }
            application::use_cases::ResetPasswordError::ExpiredToken => {
                Self::BadRequest("Expired token".to_string())
            }
            application::use_cases::ResetPasswordError::UserNotFound => {
                Self::NotFound("User not found".to_string())
            }
            application::use_cases::ResetPasswordError::RepositoryError(msg) => {
                tracing::error!(error = %msg, "Repository error");
                Self::Internal("Internal server error".to_string())
            }
            application::use_cases::ResetPasswordError::InternalError(msg) => {
                tracing::error!(error = %msg, "Internal error");
                Self::Internal("Internal server error".to_string())
            }
        }
    }
}
