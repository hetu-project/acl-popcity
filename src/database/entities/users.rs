//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub uid: String,
    pub name: String,
    #[sea_orm(unique)]
    pub email: String,
    #[sea_orm(unique)]
    pub address: Option<String>,
    pub password: Option<String>,
    #[sea_orm(unique)]
    pub invite_code: String,
    pub invited_by: Option<String>,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    pub provider: String,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
