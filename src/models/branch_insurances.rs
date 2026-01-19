use sea_orm::entity::prelude::*;

/// Branch Insurance Entity
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "bhf_insurance")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,                 // Auto-increment primary key

    pub tin: String,              // Branch TIN, max 11
    pub bhf_id: String,           // Branch ID, max 2
    pub isrcc_cd: String,         // Insurance Code, max 10
    pub isrcc_nm: String,         // Insurance Name, max 100
    pub isrc_rt: u8,              // Premium Rate, 3-digit max
    pub use_yn: String,           // Y/N flag, max 1
    pub regr_nm: String,          // Registrant Name, max 60
    pub regr_id: String,          // Registrant ID, max 20
    pub modr_nm: String,          // Modifier Name, max 60
    pub modr_id: String,          // Modifier ID, max 20
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
