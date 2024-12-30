use super::webset_handler::serve_index;
use crate::app::SharedState;
use axum::{routing::get, Router};

pub fn index_router() -> Router<SharedState> {
    Router::new().route("/", get(serve_index))
}
