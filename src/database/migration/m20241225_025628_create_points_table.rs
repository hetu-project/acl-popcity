use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Points::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Points::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Points::UserUid).string().not_null())
                    .col(ColumnDef::new(Points::PointType).string().not_null())
                    .col(ColumnDef::new(Points::Points).integer().not_null())
                    .col(ColumnDef::new(Points::Description).string().null())
                    .col(
                        ColumnDef::new(Points::ExpiresAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Points::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Points::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Points {
    Table,
    Id,
    UserUid,
    PointType,
    Points,
    Description,
    ExpiresAt,
    CreatedAt,
}
