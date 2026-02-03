use sea_orm::entity::prelude::*;
use serde::Serialize;
use rust_decimal::Decimal;

/// Item Master Entity (VSCU / eTIMS)
/// CORRECTED: Fields now match actual database NOT NULL constraints
#[derive(Clone, Debug, PartialEq, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "item_master")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    // Required fields - NOT NULL in database
    pub tin: String,          // VARCHAR NOT NULL - was Option<String>
    pub bhf_id: String,       // VARCHAR NOT NULL - was Option<String>
    pub status: String,       // VARCHAR NOT NULL - was Option<String>
    pub item_cd: String,      // VARCHAR NOT NULL
    pub item_cls_cd: String,  // VARCHAR NOT NULL
    pub item_ty_cd: String,   // VARCHAR NOT NULL
    pub item_nm: String,      // VARCHAR NOT NULL

    // Optional
    pub item_std_nm: Option<String>, // VARCHAR (nullable)

    // Classification - all NOT NULL
    pub orgn_nat_cd: String,  // VARCHAR NOT NULL
    pub pkg_unit_cd: String,  // VARCHAR NOT NULL
    pub qty_unit_cd: String,  // VARCHAR NOT NULL
    pub tax_ty_cd: String,    // VARCHAR NOT NULL

    // CRITICAL FIX: These should be NOT NULL based on your schema
    pub btch_no: String,     // VARCHAR NOT NULL (was Option<String> - THIS IS THE BUG!)
    pub bcd: String,         // VARCHAR NOT NULL (was Option<String>)

    // Prices
    pub dft_prc: Decimal,              // NUMERIC(18,2) NOT NULL
    pub grp_prc_l1: Option<Decimal>,   // NUMERIC(18,2) nullable
    pub grp_prc_l2: Option<Decimal>,   // NUMERIC(18,2) nullable
    pub grp_prc_l3: Option<Decimal>,   // NUMERIC(18,2) nullable
    pub grp_prc_l4: Option<Decimal>,   // NUMERIC(18,2) nullable
    pub grp_prc_l5: Option<Decimal>,   // NUMERIC(18,2) nullable

    // Other (optional)
    pub add_info: Option<String>,   // VARCHAR nullable
    pub sfty_qty: Option<Decimal>,  // NUMERIC(13,2) nullable
    
    // Y/N flags - NOT NULL
    pub isrc_aplcb_yn: String,       // VARCHAR NOT NULL
    pub use_yn: String,              // VARCHAR NOT NULL

    // Audit fields - all NOT NULL
    pub regr_nm: String,             // VARCHAR NOT NULL
    pub regr_id: String,             // VARCHAR NOT NULL
    pub modr_nm: String,             // VARCHAR NOT NULL
    pub modr_id: String,             // VARCHAR NOT NULL
    
    // Response (optional)
    pub response: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}