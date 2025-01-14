use super::auth_message::*;
use super::auth_service::*;
use crate::{
    app::SharedState,
    common::{
        consts,
        error::{AppError, AppResult},
    },
    server::user::*,
};
use axum::{
    debug_handler,
    extract::{Query, State},
    Json,
};
use oauth2::{reqwest::async_http_client, AuthorizationCode, RedirectUrl, TokenResponse};
use reqwest::Client;

#[debug_handler]
pub async fn auth_token(
    State(state): State<SharedState>,
    Json(params): Json<OAuthParams>,
) -> AppResult<Json<serde_json::Value>> {
    tracing::info!("[auth_token] get params: {:?}", params);

    params.validate_items()?;

    let client = state
        .oauth
        .clone()
        .set_redirect_uri(RedirectUrl::new(state.config.auth.redirect_url.clone())?);
    //.set_redirect_uri(RedirectUrl::new(params.redirect_uri.unwrap().clone())?);

    let csrf_state = params
        .state
        .ok_or(AppError::InputValidateError("Invild state".into()))?;

    let redis_client = RedisClient::from(state.redis.clone());
    if let Ok(token) = redis_client.get_csrf_token(csrf_state.as_str()).await {
        tracing::info!("got csrf token: {:?} by key: {:?}", token, csrf_state);
    } else {
        tracing::error!("got csrf token err: wrong state:{:?} ", csrf_state);
        return Err(AppError::InputValidateError(
            "csrf token verification error".into(),
        ));
    }

    let token = client
        .exchange_code(AuthorizationCode::new(params.code.unwrap()))
        .request_async(async_http_client)
        .await
        .map_err(|e| {
            tracing::error!("{:?}", e);
            AppError::RequestError(e.to_string() + ".failed to exchange code")
        })?;

    tracing::info!("[auth_token] exchange code get: {:?}", token);

    let access_token = token.access_token().secret();

    tracing::info!("[auth_token] Access Token: {:?}", access_token);

    let client = Client::new();

    let user_info_response = client
        .get(consts::USERINFO_ENDPOINT)
        .bearer_auth(&access_token)
        .send()
        .await
        .map_err(|_e| AppError::RequestError("failed to get user info".to_string()))?;

    if !user_info_response.status().is_success() {
        return Err(AppError::RequestError(
            "non user info in response".to_string(),
        ));
    }

    let user_info: OauthUserInfo = user_info_response
        .json()
        .await
        .map_err(|e| AppError::CustomError(e.to_string() + "Failed to parse user info"))?;

    tracing::info!("[auth_token] get user info: {:?}", user_info);

    let created_user = if state
        .store
        .is_user_exists_by_email(user_info.email.as_ref())
        .await?
        == true
    {
        state
            .store
            .get_user_by_email(user_info.email.as_ref())
            .await?
    } else {
        let user: User = User::from(user_info.clone());

        //points
        let user = match params.invited_by {
            Some(invited) => user.add_invited_by(invited.as_str()),
            None => user,
        };

        let created_user = match state.store.create_user(user.into()).await {
            Ok(u) => u,
            Err(AppError::UserExisted(_)) => {
                tracing::info!("user has already existed, log in");
                state
                    .store
                    .get_user_by_email(user_info.email.as_ref())
                    .await?
            }
            Err(e) => return Err(e),
        };

        match created_user.invited_by.as_deref() {
            Some(invited_by) => {
                let inviter = state.store.get_inviter_by_code(invited_by).await?;
                state
                    .store
                    .award_points(inviter.uid, "invite", 100, "invite reward")
                    .await?;
            }
            None => (),
        }

        tracing::info!("[auth_token] database  user info: {:?}", created_user);
        created_user
    };

    let secret = state.jwt_handler.clone();
    let token: String =
        secret.create_token(&created_user.uid, &created_user.name, &created_user.email);

    tracing::info!("[auth_token] jwt token: {:?}", token);

    redis_client
        .del_csrf_token(csrf_state.as_str())
        .await
        .unwrap();

    return Ok(Json(serde_json::json!({
        "result": {
            "access_token": token,
            "user_info": UserResponse::from(created_user)
        }
    })));
}

#[debug_handler]
pub async fn callback_handler(
    State(state): State<SharedState>,
    Query(params): Query<OAuthCallbackParams>,
) -> Json<serde_json::Value> {
    tracing::info!("auth params: {:?}", params);

    return Json(serde_json::json!({
        "result": {
            "code": params.code,
            "scope":params.scope,
            "authuser": params.authuser,
            "prompt": params.prompt ,
            "state": "authorization_code",
            "redirect_uri": state.config.auth.redirect_url.clone()
        }
    }));
}

#[debug_handler]
pub async fn get_csrf_token(
    State(state): State<SharedState>,
) -> AppResult<Json<serde_json::Value>> {
    let redis_client = RedisClient::from(state.redis.clone());
    let token = redis_client.cache_csrf_token().await.unwrap();
    tracing::info!("gen csrf token: {:?}", token);

    Ok(Json(serde_json::json!({
        "result": {
            "csrf_token": token
        }
    })))
}
