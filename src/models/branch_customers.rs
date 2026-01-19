use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "bhf_customer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,                 // Auto-increment primary key

    pub tin: String,              // TIN, max 11
    pub bhf_id: String,           // Branch ID, max 2
    pub cust_no: String,          // Customer number, max 9
    pub cust_tin: String,         // Customer PIN, max 11
    pub cust_nm: String,          // Customer Name, max 60
    pub adrs: Option<String>,     // Address, nullable, max 300
    pub tel_no: Option<String>,   // Contact, nullable, max 20
    pub email: Option<String>,    // Email, nullable, max 50
    pub fax_no: Option<String>,   // Fax number, nullable, max 20
    pub use_yn: String,           // Y/N flag, 1 char
    pub remark: Option<String>,   // Remark, nullable, max 1000
    pub regr_nm: String,          // Registrant name, max 60
    pub regr_id: String,          // Registrant ID, max 20
    pub modr_nm: String,          // Modifier name, max 60
    pub modr_id: String,          // Modifier ID, max 20
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
