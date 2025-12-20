pub mod dtos;
pub mod handlers;

use axum::{routing::{get, post}, Router};
use crate::http::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users", post(handlers::create_user))
        .route("/users/{id}", get(handlers::get_user))
}
