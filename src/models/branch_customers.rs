use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "bhf_customer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub tin: String,
    pub bhf_id: String,
    pub cust_no: String,
    pub cust_tin: String,
    pub cust_nm: String,
    pub adrs: Option<String>,
    pub tel_no: Option<String>,
    pub email: Option<String>,
    pub fax_no: Option<String>,
    pub use_yn: String,
    pub remark: Option<String>,
    pub regr_nm: String,
    pub regr_id: String,
    pub modr_nm: String,
    pub modr_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
