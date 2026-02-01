use sea_orm::entity::prelude::*;
use serde::Serialize;
use rust_decimal::Decimal;

/// Item Master Entity (VSCU / eTIMS)
#[derive(Clone, Debug, PartialEq, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "item_master")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    // Required
  pub tin: Option<String> ,          // CHARY11
    pub bhf_id: Option<String>,       // CHARY2
    pub status: Option<String>,
    pub item_cd: String,      // CHARY20
    pub item_cls_cd: String,  // CHARY10
    pub item_ty_cd: String,   // CHARY5
    pub item_nm: String,      // CHARY200

    // Optional
    pub item_std_nm: Option<String>, // CHARN200

    // Classification
    pub orgn_nat_cd: String,  // CHARY5
    pub pkg_unit_cd: String,  // CHARY5
    pub qty_unit_cd: String,  // CHARY5
    pub tax_ty_cd: String,    // CHARY5

    // Optional identifiers
    pub btch_no: Option<String>, // CHARN10
    pub bcd: Option<String>,     // CHARN20

    // Prices
    pub dft_prc: Decimal,              // NUMBER(18,2)
    pub grp_prc_l1: Option<Decimal>,   // NUMBER(18,2)
    pub grp_prc_l2: Option<Decimal>,   // NUMBER(18,2)
    pub grp_prc_l3: Option<Decimal>,   // NUMBER(18,2)
    pub grp_prc_l4: Option<Decimal>,   // NUMBER(18,2)
    pub grp_prc_l5: Option<Decimal>,   // NUMBER(18,2)

    // Other
    pub add_info: Option<String>,   // CHARN7
    pub sfty_qty: Option<Decimal>,  // NUMBER(13,2)
    pub isrc_aplcb_yn: String,       // CHARY1 (Y/N)
    pub use_yn: String,              // CHARY1 (Y/N)

    // Audit fields
    pub regr_nm: String,             // CHARY60
    pub regr_id: String,             // CHARY20
    pub modr_nm: String,             // CHARY60
    pub modr_id: String,             // CHARY20
  pub response: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
