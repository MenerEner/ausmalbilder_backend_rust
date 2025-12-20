mod healthcheck;
mod users;
pub mod state;

use axum::Router;
use self::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(healthcheck::router())
        .merge(users::router())
        .with_state(state)
}
