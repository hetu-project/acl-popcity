use config_macros::LoadConfig;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub db_url: String,
    pub max_connect_pool: u32,
    pub min_connect_pool: u32,
    pub connect_timeout: u64,
    pub acquire_timeout: u64,
}
#[derive(Clone, Debug, Deserialize)]
pub struct RedisConfig {
    pub redis_url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
}

#[derive(Clone, Debug, Deserialize, LoadConfig)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub redis: RedisConfig,
}
