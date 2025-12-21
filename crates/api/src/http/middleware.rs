use axum::{
    body::Body,
    http::{HeaderName, HeaderValue, Request},
    middleware::Next,
    response::Response,
};
use tracing::Instrument;
use uuid::Uuid;

pub const CORRELATION_ID_HEADER: &str = "x-correlation-id";

tokio::task_local! {
    pub static CORRELATION_ID: String;
}

#[derive(Clone, Debug)]
pub struct CorrelationId(pub String);

pub fn get_correlation_id() -> Option<String> {
    CORRELATION_ID.try_with(|id| id.clone()).ok()
}

pub async fn correlation_id_middleware(mut request: Request<Body>, next: Next) -> Response {
    let header_name = HeaderName::from_static(CORRELATION_ID_HEADER);

    let correlation_id = request
        .headers()
        .get(&header_name)
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let val = HeaderValue::from_str(&correlation_id).unwrap_or_else(|_| {
        HeaderValue::from_static("invalid-correlation-id")
    });

    request.headers_mut().insert(header_name.clone(), val.clone());
    request.extensions_mut().insert(CorrelationId(correlation_id.clone()));

    let span = tracing::info_span!(
        "http_request",
        correlation_id = %correlation_id,
        method = %request.method(),
        uri = %request.uri(),
    );

    let mut response = CORRELATION_ID
        .scope(correlation_id.clone(), next.run(request).instrument(span))
        .await;

    response.headers_mut().insert(header_name, val);

    response
}
