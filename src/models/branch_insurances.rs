use sea_orm::entity::prelude::*;
use serde::Serialize;

/// Branch Insurance Entity
#[derive(Clone, Debug, PartialEq, Eq, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "bhf_insurance")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub tin: String,
    pub bhf_id: String,
    pub isrcc_cd: String,
    pub isrcc_nm: String,
    pub isrc_rt: i32,
    pub use_yn: String,
    pub regr_nm: String,
    pub regr_id: String,
    pub modr_nm: String,
    pub modr_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
