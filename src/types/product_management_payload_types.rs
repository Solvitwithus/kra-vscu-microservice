use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSaveReq (pub Vec<Item>);





#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    // Required
    pub tin: Option<String> ,        
    pub bhf_id: Option<String>,       
    pub status: Option<String>,
    pub item_cd: String,      // CHARY20
    pub item_cls_cd: String,  // CHARY10
    pub item_ty_cd: String,   // CHARY5
    pub item_nm: String,      // CHARY200

    // Optional
    pub item_std_nm: Option<String>, // CHARN200
    pub orgn_nat_cd: String,          // CHARY5
    pub pkg_unit_cd: String,          // CHARY5
    pub qty_unit_cd: String,          // CHARY5
    pub tax_ty_cd: String,            // CHARY5
    pub btch_no: Option<String>,      // CHARN10
    pub bcd: Option<String>,          // CHARN20

    // Prices
    pub dft_prc: Decimal,             // NUMBER(18,2)
    pub grp_prc_l1: Option<Decimal>,  // NUMBER(18,2)
    pub grp_prc_l2: Option<Decimal>,  // NUMBER(18,2)
    pub grp_prc_l3: Option<Decimal>,  // NUMBER(18,2)
    pub grp_prc_l4: Option<Decimal>,  // NUMBER(18,2)
    pub grp_prc_l5: Option<Decimal>,  // NUMBER(18,2)

    // Other
    pub add_info: Option<String>,     // CHARN7
    pub sfty_qty: Option<Decimal>,    // NUMBER(13,2)
    pub isrc_aplcb_yn: String,         // CHARY1 (Y/N)
    pub use_yn: String,                // CHARY1 (Y/N)

    // Audit fields
    pub regr_nm: String,               // CHARY60
    pub regr_id: String,               // CHARY20
    pub modr_nm: String,               // CHARY60
    pub modr_id: String,               // CHARY20

    pub response: Option<KraResponse>

}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KraResponse {
    pub result_cd: String,
    pub result_msg: String,
    pub result_dt: String,
    pub data: Option<serde_json::Value>,
}