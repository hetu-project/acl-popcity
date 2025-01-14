use super::user_handler::{get_user_count, get_user_info, get_user_invites};
use crate::app::SharedState;
use crate::server::middlewares;
use axum::{middleware, routing::get, Router};

pub fn user_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/info", get(get_user_info))
        .route("/count", get(get_user_count))
        .route("/invites", get(get_user_invites))
        .layer(middleware::from_fn_with_state(
            state,
            middlewares::auth_middleware,
        ))
}
