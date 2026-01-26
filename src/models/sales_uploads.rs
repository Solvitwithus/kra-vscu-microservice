use sea_orm::entity::prelude::*;
use serde::Serialize;

/// Sales / Invoice database model
#[derive(Clone, Debug, PartialEq, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "sales")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    // ===== META / AUTH =====
    pub api_key: String,
    pub status: String,
pub generated_invc_no:i64,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,

    // ===== INVOICE CORE =====
    pub tin: String,
    pub bhf_id: String,
    pub trd_invc_no: i32,
pub retry_count:i64,
pub next_retry_at:String,
    pub invc_no: i64,
    pub org_invc_no: i64,

    pub cust_tin: String,
    pub cust_nm: String,

    pub sales_ty_cd: String,
    pub rcpt_ty_cd: String,
    pub pmt_ty_cd: String,
    pub sales_stts_cd: String,

    pub cfm_dt: String,
    pub sales_dt: String,
    pub stock_rls_dt: String,

    pub cncl_req_dt: Option<String>,
    pub cncl_dt: Option<String>,
    pub rfd_dt: Option<String>,
    pub rfd_rsn_cd: Option<String>,

    // ===== TOTALS =====
    pub tot_item_cnt: i32,

    pub taxbl_amt_a: f64,
    pub taxbl_amt_b: f64,
    pub taxbl_amt_c: f64,
    pub taxbl_amt_d: f64,
    pub taxbl_amt_e: f64,

    pub tax_rt_a: f64,
    pub tax_rt_b: f64,
    pub tax_rt_c: f64,
    pub tax_rt_d: f64,
    pub tax_rt_e: f64,

    pub tax_amt_a: f64,
    pub tax_amt_b: f64,
    pub tax_amt_c: f64,
    pub tax_amt_d: f64,
    pub tax_amt_e: f64,

    pub tot_taxbl_amt: f64,
    pub tot_tax_amt: f64,
    pub tot_amt: f64,

    // ===== FLAGS / AUDIT =====
    pub prchr_acptc_yn: String,
    pub remark: Option<String>,

    pub regr_id: String,
    pub regr_nm: String,
    pub modr_id: String,
    pub modr_nm: String,

    // ===== NESTED PAYLOADS =====
    pub receipt: Json,
    pub item_list: Json,
    pub response:Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
