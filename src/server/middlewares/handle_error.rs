use axum::{http::StatusCode, response::IntoResponse, BoxError};
use std::borrow::Cow;
use tower::{load_shed::error::Overloaded, timeout::error::Elapsed};

pub async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {error}")),
    )
}
