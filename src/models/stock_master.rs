use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "stock_master")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub tin: String,        // Branch TIN (max 11)
    pub bhf_id: String,     // Branch ID (max 2)
    pub item_cd: String,    // Item code (max 20)
    pub rsd_qty: Decimal,   // Remaining quantity (13 digits, 2 decimals)
    pub regr_nm: String,    // Registrant name (max 60)
    pub regr_id: String,    // Registrant ID (max 20)
    pub modr_nm: String,    // Modifier name (max 60)
    pub modr_id: String,    // Modifier ID (max 20)
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
