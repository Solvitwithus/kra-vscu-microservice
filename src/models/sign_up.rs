use sea_orm::entity::prelude::*;
use serde::Serialize;

/// Users database model
#[derive(Clone, Debug, PartialEq, Eq, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub full_name: String,
    pub email: String,
    pub phone_number: String,

    /// bcrypt hashed password
    pub password_hash: String,

    pub agreement: bool,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
