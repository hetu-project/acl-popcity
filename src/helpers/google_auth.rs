use crate::common::config;
use crate::common::consts;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

pub fn oauth_client(config: config::AuthConfig) -> BasicClient {
    BasicClient::new(
        ClientId::new(config.client_id),
        Some(ClientSecret::new(config.client_secret)),
        AuthUrl::new(consts::AUTH_ENDPOINT.to_string()).expect("Invalid auth URL"),
        Some(TokenUrl::new(consts::TOKEN_ENDPOINT.to_string()).expect("Invalid token URL")),
    )
    .set_redirect_uri(RedirectUrl::new(config.redirect_url).expect("Invalid redirect URL"))
}
