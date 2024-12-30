//! This module provides functionality for extracting and validating JWT tokens
//! from incoming HTTP requests in an Axum application.
//!
//! # Overview
//! - Defines the `AuthToken` struct, which encapsulates a JWT token extracted from the
//!   `Authorization` header.
//! - Implements the `FromRequestParts` trait for `AuthToken`, enabling seamless integration
//!   with Axum's request extraction system.
//!
//! # Key Features
//! - Automatically extracts and validates `Bearer` tokens from the `Authorization` header.
//! - Returns `401 Unauthorized` for requests with missing or improperly formatted tokens.
//!
//! # Usage
//! 1. Import the `AuthToken` extractor in your Axum application.
//! 2. Use `AuthToken` in your handler functions to access and validate JWT tokens.
//!
//! # Example
//! ```rust
//! use axum::{Router, routing::get, extract::Extension};
//! use crate::auth::AuthToken;
//! use http::StatusCode;
//!
//! async fn protected_route(AuthToken(token): AuthToken) -> String {
//!     format!("Your token is: {}", token)
//! }
//!
//! let app = Router::new()
//!     .route("/protected", get(protected_route));
//! ```
//!
//! # Notes
//! - This module assumes the `Authorization` header follows the `Bearer <token>` format.
//! - Ensure proper error handling for cases where tokens are invalid or expired.

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

/// A struct representing an authentication token extracted from the `Authorization` header.
pub struct AuthToken(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for AuthToken
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    /// Extracts the `Bearer` token from the `Authorization` header of the request.
    ///
    /// # Arguments
    /// - `parts`: The mutable reference to the HTTP request parts, which includes headers.
    /// - `_state`: A reference to the shared application state, unused in this implementation.
    ///
    /// # Returns
    /// - On success, returns `AuthToken` containing the extracted token.
    /// - On failure, returns `StatusCode::UNAUTHORIZED` if the token is missing or improperly formatted.
    ///
    /// # Logic
    /// 1. Access the `Authorization` header from the request.
    /// 2. Check if the header value starts with "Bearer " and extract the token part.
    /// 3. Return `AuthToken` with the token or reject with `StatusCode::UNAUTHORIZED` if the token is missing or invalid.
    ///
    /// # Example
    /// Given the following HTTP header:
    /// ```
    /// Authorization: Bearer <token>
    /// ```
    /// This method will extract `<token>` and encapsulate it in an `AuthToken`.
    ///
    /// If the `Authorization` header is missing or improperly formatted, the method rejects the request with `401 Unauthorized`.
    async fn from_request_parts<'life0, 'life1>(
        parts: &'life0 mut Parts,
        _state: &'life1 S,
    ) -> std::result::Result<Self, Self::Rejection>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        let token = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Return the extracted token wrapped in an `AuthToken`.
        Ok(AuthToken(token.to_string()))
    }
}
