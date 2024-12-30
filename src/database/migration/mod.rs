pub use sea_orm_migration::prelude::*;

mod m20241223_064204_create_users_table;
mod m20241225_025628_create_points_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241223_064204_create_users_table::Migration),
            Box::new(m20241225_025628_create_points_table::Migration),
        ]
    }
}
