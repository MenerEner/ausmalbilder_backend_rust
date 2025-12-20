mod healthcheck;
mod users;
pub mod state;

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use self::state::AppState;
use crate::http::users::dtos::{CreateUserRequest, UserResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        users::handlers::create_user,
    ),
    components(
        schemas(CreateUserRequest, UserResponse)
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
        .with_state(state)
}
