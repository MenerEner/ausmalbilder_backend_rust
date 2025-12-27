use crate::http::ApiResponse;
use crate::http::state::AppState;
use axum::routing::get;
use axum::{Json, Router, extract::State};
use sea_orm::ConnectionTrait;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    status: &'static str,
    database: &'static str,
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Health check", body = ApiResponse<HealthResponse>)
    )
)]
async fn health(State(state): State<AppState>) -> Json<ApiResponse<HealthResponse>> {
    let db_status = match state
        .db
        .execute(sea_orm::Statement::from_string(
            state.db.get_database_backend(),
            "SELECT 1",
        ))
        .await
    {
        Ok(_) => "up",
        Err(_) => "down",
    };

    Json(ApiResponse::success(HealthResponse {
        status: "ok",
        database: db_status,
    }))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health))
}
