mod error;
mod healthcheck;
mod middleware;
mod response;
pub mod state;
mod users;

pub use error::{ApiErrorDetail, ApiErrorResponse, AppError};
pub use response::{ApiResponse, ApiResponseUser, PaginatedResponse};

use self::state::AppState;
use crate::http::healthcheck::HealthResponse;
use crate::http::users::dtos::{
    CreateUserRequest, ForgotPasswordRequest, PaginationParams, ResetPasswordRequest, UserResponse,
    VerifyEmailRequest,
};
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        healthcheck::health,
        users::handlers::create_user,
        users::handlers::get_user,
        users::handlers::list_users,
        users::handlers::delete_user,
        users::handlers::signup,
        users::handlers::verify_email,
        users::handlers::forgot_password,
        users::handlers::reset_password,
    ),
    components(
        schemas(
            HealthResponse,
            CreateUserRequest,
            UserResponse,
            VerifyEmailRequest,
            ForgotPasswordRequest,
            ResetPasswordRequest,
            PaginationParams,
            ApiResponseUser,
            ApiErrorResponse,
            ApiErrorDetail
        )
    ),
    tags(
        (name = "users", description = "User management endpoints")
    )
)]
pub struct ApiDoc;

pub fn router(state: AppState) -> Router {
    log_routes();
    Router::new()
        .merge(healthcheck::router())
        .merge(users::router())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(axum::middleware::from_fn(
            middleware::correlation_id_middleware,
        ))
        .with_state(state)
}

fn log_routes() {
    let openapi = ApiDoc::openapi();
    for (path, path_item) in openapi.paths.paths {
        if path_item.get.is_some() {
            tracing::info!(method = "GET", path = %path, "route registered");
        }
        if path_item.post.is_some() {
            tracing::info!(method = "POST", path = %path, "route registered");
        }
        if path_item.put.is_some() {
            tracing::info!(method = "PUT", path = %path, "route registered");
        }
        if path_item.delete.is_some() {
            tracing::info!(method = "DELETE", path = %path, "route registered");
        }
        if path_item.patch.is_some() {
            tracing::info!(method = "PATCH", path = %path, "route registered");
        }
        if path_item.options.is_some() {
            tracing::info!(method = "OPTIONS", path = %path, "route registered");
        }
        if path_item.head.is_some() {
            tracing::info!(method = "HEAD", path = %path, "route registered");
        }
        if path_item.trace.is_some() {
            tracing::info!(method = "TRACE", path = %path, "route registered");
        }
    }
    tracing::info!(
        method = "GET",
        path = "/api-docs/openapi.json",
        "route registered"
    );
    tracing::info!(method = "GET", path = "/swagger-ui", "route registered");
}
