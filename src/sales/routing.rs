use std::sync::Arc;

use axum::{
    Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post
};
use axum_extra::extract::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, TransactionTrait};
use serde_json::json;
use tracing::{info, warn};

use crate::{
    models::sales_uploads::{ActiveModel, Column, Entity},
    types::salespayloadtype::{InvoicePayload, TrnsSalesSaveWrReq},
    utils::bearer::bearer_resolver,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AuthUser {
    api_key: String,
    branch_id: String,
    company_id: String,
    device_serial: String,
    environment_name: String,
    environment_url: String,
    id: i32,
    pin: String,
}

pub fn sales_route(db: Arc<DatabaseConnection>) -> Router {
    Router::new()
        .route("/", post(handle_payload_post))
        .with_state(db)
}

pub async fn handle_payload_post(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    State(db): State<Arc<DatabaseConnection>>,
    Json(payload): Json<InvoicePayload>,
) -> impl IntoResponse {

    let token = auth.token();
    info!("Bearer token received");

    // 1️⃣ AUTH FIRST
    let user: AuthUser = match bearer_resolver(token, db.as_ref()).await {
        Ok(val) => match serde_json::from_value(val) {
            Ok(u) => u,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "message": format!("Failed to parse user: {}", e) })),
                )
            }
        },
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": e })),
            )
        }
    };

    // 2️⃣ START TRANSACTION
    let txn = match db.begin().await {
        Ok(t) => t,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": format!("Transaction start failed: {e}") })),
            )
        }
    };

    // 3️⃣ FETCH LAST INVOICE NUMBER FOR THIS API KEY (OUTSIDE THE LOOP)
    let last_sale = match Entity::find()
        .filter(Column::ApiKey.eq(token))  // ✅ Filter by API key, not bhf_id
        .order_by_desc(Column::GeneratedInvcNo)
        .one(db.as_ref())
        .await 
    {
        Ok(val) => val,
        Err(err) => {
            let _ = txn.rollback().await;
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": format!("Failed to fetch last sale: {err}") })),
            );
        }
    };

    // Determine starting invoice number for this API key
    let mut current_invoice_number = if let Some(record) = last_sale {
        record.generated_invc_no
    } else {
        0  // Start at 0, will increment to 1 for first invoice
    };

    // 4️⃣ INSERT PAYLOAD - INCREMENT FOR EACH ITEM
    for item in payload.0.iter() {
        // Increment invoice number for each item
        current_invoice_number += 1;

        let model = ActiveModel {
            api_key: Set(token.to_string()),
            status: Set("RECEIVED".to_string()),

            // ✅ Generated from authenticated user
            tin: Set(user.pin.clone()),
            bhf_id: Set(user.branch_id.clone()),
            
            // ✅ Use incremented invoice number
            generated_invc_no: Set(current_invoice_number),
            invc_no: Set(current_invoice_number),
            
            // Payload fields
            trd_invc_no: Set(item.trdInvcNo),
            org_invc_no: Set(item.orgInvcNo),

            cust_tin: Set(item.custTin.clone()),
            cust_nm: Set(item.custNm.clone()),

            taxbl_amt_a: Set(item.taxblAmtA),
            taxbl_amt_b: Set(item.taxblAmtB),
            taxbl_amt_c: Set(item.taxblAmtC),
            taxbl_amt_d: Set(item.taxblAmtD),
            taxbl_amt_e: Set(item.taxblAmtE),

            tax_rt_a: Set(item.taxRtA),
            tax_rt_b: Set(item.taxRtB),
            tax_rt_c: Set(item.taxRtC),
            tax_rt_d: Set(item.taxRtD),
            tax_rt_e: Set(item.taxRtE),

            tax_amt_a: Set(item.taxAmtA),
            tax_amt_b: Set(item.taxAmtB),
            tax_amt_c: Set(item.taxAmtC),
            tax_amt_d: Set(item.taxAmtD),
            tax_amt_e: Set(item.taxAmtE),

            tot_taxbl_amt: Set(item.totTaxblAmt),
            tot_tax_amt: Set(item.totTaxAmt),
            tot_amt: Set(item.totAmt),

            regr_id: Set(item.regrId.clone()),
            regr_nm: Set(item.regrNm.clone()),
            modr_id: Set(item.modrId.clone()),
            modr_nm: Set(item.modrNm.clone()),

            prchr_acptc_yn: Set(item.prchrAcptcYn.clone()),
            
            sales_ty_cd: Set(item.salesTyCd.clone()),
            rcpt_ty_cd: Set(item.rcptTyCd.clone()),
            pmt_ty_cd: Set(item.pmtTyCd.clone()),
            sales_stts_cd: Set(item.salesSttsCd.clone()),

            cfm_dt: Set(item.cfmDt.clone()),
            sales_dt: Set(item.salesDt.clone()),
            stock_rls_dt: Set(item.stockRlsDt.clone()),

            tot_item_cnt: Set(item.totItemCnt as i32),

            receipt: Set(serde_json::to_value(&item.receipt).unwrap()),
            item_list: Set(serde_json::to_value(&item.itemList).unwrap()),
            response: Set(Some(serde_json::to_value(&item.itemList).unwrap())),

            ..Default::default()
        };

        if let Err(e) = model.insert(&txn).await {
            let _ = txn.rollback().await;
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": format!("Insert failed: {e}") })),
            );
        }

        info!("Inserted invoice #{} for api_key: {}", current_invoice_number, token);
    }

    // 5️⃣ COMMIT
    match txn.commit().await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "message": "success",
                "resultMsg": "Sales uploaded successfully",
                "invoices_created": payload.0.len(),
                "last_invoice_number": current_invoice_number
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": format!("Commit failed: {e}") })),
        ),
    }
}