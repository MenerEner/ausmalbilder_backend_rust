mod healthcheck;
pub mod state;

use axum::Router;
use self::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(healthcheck::router())
        .with_state(state)
}
