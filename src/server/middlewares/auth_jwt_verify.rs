//! This module provides middleware for handling JWT authentication in Axum applications.
//!
//! # Overview
//! - The middleware validates JWT tokens included in the `Authorization` header of incoming requests.
//! - Tokens are decoded and verified using the application's shared JWT handler.
//! - On successful validation, the request is passed to the next handler in the processing pipeline.
//! - If the token is invalid or expired, an appropriate HTTP status code is returned.
//!
//! # Key Components
//! - **`auth_middleware`**: The primary middleware function that performs the authentication.
//! - **`AuthToken`**: A custom extractor for parsing JWT tokens from requests.
//! - **`SharedState`**: Application-wide shared state, including the JWT handler.
//!
//! # Usage
//! 1. Add this middleware to your Axum router to protect specific routes or groups of routes.
//! 2. Ensure the shared state (`SharedState`) contains a properly configured JWT handler.
//!
//! # Example
//! ```rust
//! use axum::{Router, routing::get, middleware};
//! use crate::auth_middleware::auth_middleware;
//!
//! let app = Router::new()
//!     .route("/protected", get(protected_handler))
//!     .route_layer(middleware::from_fn(auth_middleware));
//!
//! async fn protected_handler() -> &'static str {
//!     "You are authenticated!"
//! }
//! ```

use super::AuthToken;
use crate::app::SharedState;
use axum::{
    body::Body, extract::State, http::Request, http::StatusCode, middleware::Next,
    response::Response,
};
use jsonwebtoken::errors::ErrorKind;
use tracing;

/// Middleware for authenticating requests using JWT tokens.
///
/// # Arguments
/// - `State(state)`: Shared application state, which contains the JWT handler.
/// - `AuthToken(user)`: Extracted JWT token from the `Authorization` header of the request.
/// - `req`: The incoming HTTP request.
/// - `next`: The next middleware or handler in the processing pipeline.
///
/// # Returns
/// - On successful authentication, forwards the request to the next handler and returns its response.
/// - On authentication failure, returns an appropriate `StatusCode` error response.
pub async fn auth_middleware(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let client = state.jwt_handler.clone();

    // Decode and verify the provided JWT token.
    match client.decode_token(user) {
        Ok(claim) => tracing::info!("Authenticate pass, extract claim: {:?}", claim),
        Err(e) => {
            if e.kind().eq(&ErrorKind::ExpiredSignature) {
                return Err(StatusCode::UNAUTHORIZED);
            } else {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
    }

    // If authentication succeeds, pass the request to the next handler in the pipeline.
    Ok(next.run(req).await)
}
