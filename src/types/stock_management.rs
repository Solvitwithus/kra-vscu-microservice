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

