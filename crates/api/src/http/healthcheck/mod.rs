use axum::{extract::State, Json, Router};
use axum::routing::get;
use serde::Serialize;
use crate::http::state::AppState;
use sea_orm::ConnectionTrait;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    database: &'static str,
}

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    let db_status = match state.db.execute(sea_orm::Statement::from_string(state.db.get_database_backend(), "SELECT 1")).await {
        Ok(_) => "up",
        Err(_) => "down",
    };

    Json(HealthResponse {
        status: "ok",
        database: db_status,
    })
}

pub fn router() -> Router<AppState> {
   Router::new().route("/health", get(health))
}