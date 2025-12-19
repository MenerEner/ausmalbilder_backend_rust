mod healthcheck;

use axum::Router;

pub fn router() -> Router {
    Router::new().merge(healthcheck::router())
}
