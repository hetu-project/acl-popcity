use crate::{
    common::{config::Config, consts, error::AppResult},
    database,
    helpers::google_auth,
    server::{http_server_start, middlewares::jwt::jwt_handler},
};
use std::ops::Deref;
use std::{path::PathBuf, sync::Arc};
//use tokio::sync::RwLock;
use oauth2::basic::BasicClient;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub store: database::Storage,
    pub jwt_handler: jwt_handler::JwtHandler,
    pub oauth: BasicClient,
    pub redis: redis::Client,
}

impl AppState {
    pub async fn new(path: PathBuf) -> Self {
        let config = Config::load_config(path).unwrap();
        let store = database::Storage::new(config.database.clone()).await;

        let secret = consts::JWT_SECRET_KEY.to_string();
        let jwt_handler = jwt_handler::JwtHandler { secret };

        Self {
            config: config.clone(),
            store,
            jwt_handler,
            oauth: google_auth::oauth_client(config.auth),
            redis: redis::Client::open(config.redis.redis_url).unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SharedState(pub Arc<AppState>);

impl Deref for SharedState {
    type Target = AppState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SharedState {
    pub async fn new(path: PathBuf) -> Self {
        let state = AppState::new(path).await;
        SharedState(Arc::new(state))
    }

    pub async fn run(&self) -> AppResult<()> {
        http_server_start(self.clone()).await?;

        Ok(())
    }
}
