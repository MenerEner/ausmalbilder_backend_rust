pub mod dtos;
pub mod handlers;

use axum::{routing::post, Router};
use crate::http::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/users", post(handlers::create_user))
}
