use sea_orm::entity::prelude::*;
use serde::Serialize;

/// Branch Insurance Entity
#[derive(Clone, Debug, PartialEq, Eq, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "credentials")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32, 

    pub company_id: String,
    pub environment_name: String,
    pub environment_url: String,
    pub pin: String,
    pub branch_id: String,
    pub device_serial: String,
   pub api_key: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
