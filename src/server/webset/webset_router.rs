use super::webset_handler::serve_index;
use crate::app::SharedState;
use axum::{routing::get, Router};

pub fn index_router() -> Router<SharedState> {
    if cfg!(debug_assertions) {
        Router::new().route("/", get(serve_index))
    } else {
        Router::new().route("/fortest/index", get(serve_index))
    }
}
