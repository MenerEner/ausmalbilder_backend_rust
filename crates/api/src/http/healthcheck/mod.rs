use axum::{Json, Router};
use axum::routing::get;
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok"})
}

pub fn router() -> Router {
   Router::new().route("/health", get(health))
}