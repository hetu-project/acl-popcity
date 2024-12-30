use super::auth_handler::{auth_token, callback_handler, get_csrf_token};
use crate::app::SharedState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn auth_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/callback", get(callback_handler))
        .route("/token", post(auth_token))
        .route("/csrf_token", get(get_csrf_token))
        .with_state(state.clone())
}
