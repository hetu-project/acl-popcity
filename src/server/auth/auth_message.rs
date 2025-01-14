use crate::common::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::convert::From;
use validator::Validate;

#[derive(Deserialize, Debug)]
pub struct OAuthCallbackParams {
    pub code: String,
    #[allow(unused)]
    pub scope: String,
    #[allow(unused)]
    pub authuser: String,
    #[allow(unused)]
    pub prompt: String,
}

#[derive(Deserialize, Debug, Validate)]
pub struct OAuthParams {
    #[validate(length(min = 1, message = "Code cannot be empty"))]
    pub code: Option<String>,
    #[allow(unused)]
    pub scope: Option<String>,
    #[allow(unused)]
    pub authuser: Option<String>,
    #[allow(unused)]
    pub prompt: Option<String>,
    #[validate(length(min = 1, message = "State cannot be empty"))]
    pub state: Option<String>,
    #[validate(url)]
    pub redirect_uri: Option<String>, //TODO delete in release
    #[allow(unused)]
    pub invited_by: Option<String>,
}

impl OAuthParams {
    pub fn validate_items(&self) -> AppResult<()> {
        if self.code.is_none() {
            return Err(AppError::InputValidateError("code not found".to_string()));
        }

        if self.state.is_none() {
            return Err(AppError::InputValidateError("state not found".to_string()));
        }

        if self.redirect_uri.is_none() {
            return Err(AppError::InputValidateError(
                "redirect_uri not found".to_string(),
            ));
        }

        Ok(self.validate()?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OauthUserInfo {
    pub sub: String,
    pub name: String,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub email_verified: bool,
}
