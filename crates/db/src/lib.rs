use sea_orm::*;
use sea_orm_migration::prelude::*;
use std::{sync::Arc, time::Duration};

pub async fn setup_db<M: MigratorTrait>(
    req_url: &str,
    db_name: &str,
) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(req_url).await?;
    let db = match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", db_name),
            ))
            .await?;

            let url = format!("{}/{}", req_url, db_name);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", db_name),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", db_name),
            ))
            .await?;

            let url = format!("{}/{}", req_url, db_name);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };

    M::up(&db.clone(), None).await?;

    Ok(db)
}

#[derive(Debug, Default, Clone)]
pub struct Storage {
    pub conn: Arc<DatabaseConnection>,
}

impl Storage {
    pub async fn new(
        db_url: String,
        max_connect_pool: u32,
        min_connect_pool: u32,
        connect_timeout: u64,
        acquire_timeout: u64,
    ) -> Self {
        let mut opt = ConnectOptions::new(&db_url);
        opt.max_connections(max_connect_pool)
            .min_connections(min_connect_pool)
            .connect_timeout(Duration::from_secs(connect_timeout))
            .acquire_timeout(Duration::from_secs(acquire_timeout));

        let db = Database::connect(opt.clone())
            .await
            .expect("failed to connect to database");

        Self { conn: Arc::new(db) }
    }
}
