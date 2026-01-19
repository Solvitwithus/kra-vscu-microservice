use sea_orm::entity::prelude::*;

/// Branch Users database model
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "bhf_users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,               // Auto-increment primary key

    pub tin: String,            // Branch TIN, max 11
    pub bhf_id: String,         // Branch ID, max 2
    pub user_id: String,        // User ID, max 20
    pub user_nm: String,        // User Name, max 60
    pub pwd: String,            // Password, max 255
    pub adrs: Option<String>,   // Address, nullable, max 200
    pub cntc: Option<String>,   // Contact, nullable, max 20
    pub auth_cd: Option<String>,// Authority Code, nullable, max 100
    pub remark: Option<String>, // Remark, nullable, max 2000
    pub use_yn: String,         // Y/N, max 1
    pub regr_nm: String,        // Registrant Name, max 60
    pub regr_id: String,        // Registrant ID, max 20
    pub modr_nm: String,        // Modifier Name, max 60
    pub modr_id: String,        // Modifier ID, max 20
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
