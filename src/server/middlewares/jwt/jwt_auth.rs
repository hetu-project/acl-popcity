///
///         let auth = jwt_auth::Authorization {
///             jwt_handler: share_state.0.read().await.jwt_handler.clone(),
///         };
///
/// .layer(ValidateRequestHeaderLayer::custom(auth));
///
use super::jwt_handler::JwtHandler;
use axum::body::Body;
use axum::extract::Request;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use jsonwebtoken::errors::ErrorKind;
use tower_http::validate_request::ValidateRequest;

#[derive(Clone)]
pub struct Authorization {
    pub jwt_handler: JwtHandler,
}

impl<B> ValidateRequest<B> for Authorization {
    type ResponseBody = Body;
    fn validate(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        request
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .map(|token| {
                self.jwt_handler.clone().decode_token(
                    token
                        .to_string()
                        .strip_prefix("Bearer ")
                        .unwrap_or(&token)
                        .to_string(),
                )
            })
            .map(|result| match result {
                Ok(res) => {
                    println!("res:{:?}", res);
                    Ok(())
                }
                Err(e) => {
                    println!("decode token error: {:?}", e);
                    if e.kind().eq(&ErrorKind::ExpiredSignature) {
                        Err((
                            StatusCode::UNAUTHORIZED,
                            "Token expired, please login again",
                        )
                            .into_response())
                    } else {
                        Err(StatusCode::UNAUTHORIZED.into_response())
                    }
                }
            })
            .unwrap_or_else(|| {
                println!("Missing authorization header");
                Err(StatusCode::UNAUTHORIZED.into_response())
            })
    }

    //fn validate(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
    //    request
    //        .headers()
    //        .get(header::COOKIE)
    //        .and_then(|cookie| cookie.to_str().ok())
    //        .and_then(|cookie| cookie.split(';').find(|cookie| cookie.contains("token")))
    //        .and_then(|cookie| cookie.split('=').nth(1))
    //        .map(|token| self.jwt_handler.clone().decode_token(token.to_string()))
    //        .map(|result| match result {
    //            Ok(_) => Ok(()),
    //            Err(e) => {
    //                if e.kind().eq(&ErrorKind::ExpiredSignature) {
    //                    Err((
    //                        StatusCode::UNAUTHORIZED,
    //                        "Token expired, please login again",
    //                    )
    //                        .into_response())
    //                } else {
    //                    Err(StatusCode::UNAUTHORIZED.into_response())
    //                }
    //            }
    //        })
    //        .unwrap_or_else(|| {
    //            println!("Missing authorization header");
    //            Err(StatusCode::UNAUTHORIZED.into_response())
    //        })
    //}
}
