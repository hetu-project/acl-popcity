use super::health_handler::healthcheck;
use crate::app::SharedState;
use axum::{routing::get, Router};

pub fn health_router() -> Router<SharedState> {
    Router::new()
        .route("/ping", get(|| async { "pong" }))
        .route("/healthcheck", get(healthcheck))
}
