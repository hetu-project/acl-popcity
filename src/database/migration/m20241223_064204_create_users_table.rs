use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Uid).string().not_null().unique_key())
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::Address).string().unique_key().null())
                    .col(ColumnDef::new(Users::Password).string().null())
                    .col(
                        ColumnDef::new(Users::InviteCode)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::InvitedBy).string().null())
                    .col(ColumnDef::new(Users::Role).string().not_null())
                    .col(ColumnDef::new(Users::Photo).string().not_null())
                    .col(ColumnDef::new(Users::Verified).boolean().not_null())
                    .col(ColumnDef::new(Users::Provider).string().not_null())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Uid,
    Name,
    Email,
    Address,
    Password,
    InvitedBy,
    InviteCode,
    Role,
    Photo,
    Verified,
    Provider,
    CreatedAt,
    UpdatedAt,
}
