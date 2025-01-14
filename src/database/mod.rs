pub mod entities;
pub mod migration;
pub mod services;

use crate::common::{config::DatabaseConfig, error::AppResult};
use sea_orm::*;
use std::{sync::Arc, time::Duration};

#[derive(Debug, Default, Clone)]
pub struct Storage {
    pub conn: Arc<DatabaseConnection>,
}

impl Storage {
    pub async fn new(config: DatabaseConfig) -> Self {
        let mut opt = ConnectOptions::new(&config.db_url);
        opt.max_connections(config.max_connect_pool)
            .min_connections(config.min_connect_pool)
            .connect_timeout(Duration::from_secs(config.connect_timeout))
            .acquire_timeout(Duration::from_secs(config.acquire_timeout));

        let db = Database::connect(opt.clone())
            .await
            .expect("failed to connect to database");

        Self { conn: Arc::new(db) }
    }
}

#[derive(Debug)]
pub struct DbTxn(pub DatabaseTransaction);

impl DbTxn {
    pub async fn new(db: &DatabaseConnection) -> AppResult<Self> {
        Ok(Self(db.begin().await?))
    }

    pub async fn commit_transaction(self) -> AppResult<()> {
        Ok(self.0.commit().await?)
    }
}
