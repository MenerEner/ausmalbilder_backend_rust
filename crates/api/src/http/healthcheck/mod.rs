use axum::{Json, Router};
use axum::routing::get;
use serde::Serialize;
use crate::http::state::AppState;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok"})
}

pub fn router() -> Router<AppState> {
   Router::new().route("/health", get(health))
}