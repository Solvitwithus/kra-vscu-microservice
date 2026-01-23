use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockMasterItem {
    pub tin: String,
    #[serde(rename = "bhfId")]
    pub bhf_id: String,
    #[serde(rename = "itemCd")]
    pub item_cd: String,
    #[serde(rename = "rsdQty")]
    pub rsd_qty: f64, // Can convert to Decimal before DB insert
    #[serde(rename = "regrNm")]
    pub regr_nm: String,
    #[serde(rename = "regrId")]
    pub regr_id: String,
    #[serde(rename = "modrNm")]
    pub modr_nm: String,
    #[serde(rename = "modrId")]
    pub modr_id: String,
}

/// Accepts both a single item or multiple items
#[derive(Debug, Clone, Deserialize)]


pub struct StockMstSaveReq(pub Vec<StockMasterItem>);



#[derive(Debug, Clone, Deserialize)]
pub struct SaveStockItemsReq(pub Vec<StockItem>);

#[derive(Debug, Clone, Deserialize)]
pub struct StockItem {
    // Header-level fields
    pub tin: String,               // Taxpayer Identification Number
    pub bhf_id: String,            // Branch ID
    pub sar_no: u32,               // Stock In/Out Number
    pub org_sar_no: u32,           // Original Stock In/Out Number
    pub reg_ty_cd: String,         // Registration Type Code
    pub cust_tin: Option<String>,  // Customer TIN
    pub cust_nm: Option<String>,   // Customer Name
    pub cust_bhf_id: Option<String>, // Customer Branch ID
    pub sar_ty_cd: String,         // Stock In/Out Type Code
    pub ocrn_dt: String,           // Occurred Date (yyyyMMdd)
    pub tot_item_cnt: u32,         // Total Item Count
    pub tot_taxbl_amt: f64,        // Total Taxable Amount
    pub tot_tax_amt: f64,          // Total Tax Amount
    pub tot_amt: f64,              // Total Amount
    pub remark: Option<String>,    // Remark
    pub regr_nm: String,           // Registrant Name
    pub regr_id: String,           // Registrant ID
    pub modr_nm: String,           // Modifier Name
    pub modr_id: String,           // Modifier ID

    // Item list
    pub item_list: Vec<ItemDetail>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemDetail {
    pub item_seq: u32,             // Item Sequence
    pub item_cd: String,           // Item Code
    pub item_cls_cd: String,       // Item Class Code
    pub item_nm: String,           // Item Name
    pub bcd: Option<String>,       // Barcode
    pub pkg_unit_cd: String,       // Package Unit Code
    pub pkg: f64,                  // Package Quantity
    pub qty_unit_cd: String,       // Unit Quantity Code
    pub qty: f64,                  // Quantity
    pub item_expr_dt: Option<String>, // Expiry Date (yyyyMMdd)
    pub prc: f64,                  // Unit Price
    pub sply_amt: f64,             // Supply Amount
    pub tot_dc_amt: f64,           // Discount Amount
    pub taxbl_amt: f64,            // Taxable Amount
    pub tax_ty_cd: String,         // Tax Type Code
    pub tax_amt: f64,              // Tax Amount
    pub tot_amt: f64,              // Total Amount
}
