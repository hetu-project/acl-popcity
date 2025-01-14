use axum::{debug_handler, http::StatusCode};

#[debug_handler]
pub async fn healthcheck() -> StatusCode {
    StatusCode::OK
}
