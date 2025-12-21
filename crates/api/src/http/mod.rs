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
    CreateUserRequest, PaginationParams, UserResponse, VerifyEmailRequest,
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
    ),
    components(
        schemas(
            HealthResponse,
            CreateUserRequest,
            UserResponse,
            VerifyEmailRequest,
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
    Router::new()
        .merge(healthcheck::router())
        .merge(users::router())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(axum::middleware::from_fn(
            middleware::correlation_id_middleware,
        ))
        .with_state(state)
}
