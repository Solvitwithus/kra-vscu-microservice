use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TrnsSalesSaveWrReq {
    pub tin: Option<String>,
    pub bhfId: Option<String>,
    pub trdInvcNo: i64,
pub generated_invc_no:Option<i64>,
    pub invcNo: Option<i64>,
    pub orgInvcNo: i64,

    pub custTin: String,
    pub custNm: String,

    pub salesTyCd: String,
    pub rcptTyCd: String,
    pub pmtTyCd: String,
    pub salesSttsCd: String,

    pub cfmDt: String,        // yyyyMMddhhmmss
    pub salesDt: String,      // yyyyMMdd
    pub stockRlsDt: String,   // yyyyMMddhhmmss

    pub cnclReqDt: Option<String>,
    pub cnclDt: Option<String>,
    pub rfdDt: Option<String>,
    pub rfdRsnCd: Option<String>,

    pub totItemCnt: i64,

    pub taxblAmtA: f64,
    pub taxblAmtB: f64,
    pub taxblAmtC: f64,
    pub taxblAmtD: f64,
    pub taxblAmtE: f64,

    pub taxRtA: f64,
    pub taxRtB: f64,
    pub taxRtC: f64,
    pub taxRtD: f64,
    pub taxRtE: f64,

    pub taxAmtA: f64,
    pub taxAmtB: f64,
    pub taxAmtC: f64,
    pub taxAmtD: f64,
    pub taxAmtE: f64,

    pub totTaxblAmt: f64,
    pub totTaxAmt: f64,
    pub totAmt: f64,

    pub prchrAcptcYn: String,
    pub remark: Option<String>,

    pub regrId: String,
    pub regrNm: String,
    pub modrId: String,
    pub modrNm: String,

    pub receipt: ReceiptInfo,
    pub itemList: Vec<TrnsSalesSaveWrItem>,
    pub response: Option<Vec<TrnsSalesSaveWrRes>>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiptInfo {
    pub custTin: String,
    pub custMblNo: Option<String>,
    pub rptNo: i64,
    pub trdeNm: String,
    pub adrs: String,
    pub topMsg: String,
    pub btmMsg: String,
    pub prchrAcptcYn: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TrnsSalesSaveWrItem {
    pub itemSeq: i64,
    pub itemCd: String,
    pub itemClsCd: String,
    pub itemNm: String,
    pub bcd: Option<String>,

    pub pkgUnitCd: String,
    pub pkg: f64,

    pub qtyUnitCd: String,
    pub qty: f64,

    pub prc: f64,
    pub splyAmt: f64,

    pub dcRt: f64,
    pub dcAmt: f64,

    pub isrccCd: Option<String>,
    pub isrccNm: Option<String>,
    pub isrcRt: Option<f64>,
    pub isrcAmt: Option<f64>,

    pub taxTyCd: String,
    pub taxblAmt: f64,
    pub taxAmt: f64,
    pub totAmt: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TrnsSalesSaveWrRes {
    pub resultCd: String,
    pub resultMsg: String,
    pub resultDt: String,
    pub data: Option<TrnsSalesSaveResData>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TrnsSalesSaveResData {
    pub rcptNo: i64,
    pub intrlData: String,
    pub rcptSign: String,
    pub totRcptNo: i64,
    pub VSCURcptPbctDate: String,
    pub sdcId: String,
    pub mrcNo: String,
}

#[derive(Debug, Serialize, Deserialize)]



pub struct InvoicePayload(pub Vec<TrnsSalesSaveWrReq>);
#[derive(Debug, Deserialize, Serialize)]
pub struct AuthUser {
    pub api_key: String,
    pub branch_id: String,
    pub company_id: String,
    pub device_serial: String,
   pub environment_name: String,
   pub environment_url: String,
   pub id: i32,
   pub pin: String,
}

// Response structure from KRA endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct KraResponse {
    // Add fields based on actual KRA response
    // For now using generic Value
    #[serde(flatten)]
    pub data: serde_json::Value,
}