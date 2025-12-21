pub mod dtos;
pub mod handlers;

use crate::http::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users", post(handlers::create_user).get(handlers::list_users))
        .route(
            "/users/{id}",
            get(handlers::get_user).delete(handlers::delete_user),
        )
}
