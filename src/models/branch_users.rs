use sea_orm::entity::prelude::*;
use serde::Serialize;

/// Branch Users database model
#[derive(Clone, Debug, PartialEq, Eq, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "bhf_users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub tin: String,
    pub bhf_id: String,
    pub user_id: String,
    pub user_nm: String,
    pub pwd: String,
    pub adrs: Option<String>,
    pub cntc: Option<String>,
    pub auth_cd: Option<String>,
    pub remark: Option<String>,
    pub use_yn: String,
    pub regr_nm: String,
    pub regr_id: String,
    pub modr_nm: String,
    pub modr_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
