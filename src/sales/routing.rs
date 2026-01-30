
use std::sync::Arc;

use axum::{
    Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post
};
use axum_extra::extract::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect, TransactionTrait};
use serde_json::json;
use tracing::{info, error};
use reqwest;
use chrono::Utc;
use crate::{
    models::sales_uploads::{ActiveModel, Column, Entity},
    types::salespayloadtype::{InvoicePayload,AuthUser},
    utils::{bearer::bearer_resolver, crypto::decrypt_deterministic},
};







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

    // first get to fetch the entire records with api if none start from 0

    // 3️⃣ FETCH LAST INVOICE NUMBER FOR THIS API KEY
   let last_sale = match Entity::find()
    .filter(Column::ApiKey.eq(token.clone()))
    .order_by_desc(Column::GeneratedInvcNo)
    .limit(1)
    .all(db.as_ref())
    .await
{
    Ok(mut rows) => rows.pop(),
    Err(err) => {
        let _ = txn.rollback().await;
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": format!("Failed to fetch last sale: {err}")
            })),
        );
    }
};


   

    let mut current_invoice_number = if let Some(record) = last_sale {
        record.generated_invc_no
    } else {
        0
    };

    let mut inserted_ids: Vec<i32> = Vec::new();

    // 4️⃣ INSERT PAYLOAD - INCREMENT FOR EACH ITEM
    for item in payload.0.iter() {
        current_invoice_number += 1;

        let model = ActiveModel {
            api_key: Set(token.to_string()),
            status: Set("RECEIVED".to_string()),

            tin: Set(user.pin.clone()),
            bhf_id: Set(user.branch_id.clone()),
            
            generated_invc_no: Set(current_invoice_number),
            invc_no: Set(current_invoice_number),
            
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
            response: Set(None), // Will be set after KRA call

            ..Default::default()
        };

        match model.insert(&txn).await {
            Ok(inserted) => {
                inserted_ids.push(inserted.id);
                info!("Inserted invoice #{} with ID {} for api_key: {}", 
                      current_invoice_number, inserted.id, token);
            }
            Err(e) => {
                let _ = txn.rollback().await;
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "message": format!("Insert failed: {e}") })),
                );
            }
        }
    }

    // 5️⃣ COMMIT TRANSACTION in a wierd syntax
    if let Err(e) = txn.commit().await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": format!("Commit failed: {e}") })),
        );
    }

    info!("Successfully inserted {} invoices. Starting KRA transmission...", inserted_ids.len());

    // 6️⃣ TRANSMIT TO KRA ENDPOINT
for id in inserted_ids {
        // Fetch the record we just inserted
        let record = match Entity::find_by_id(id).one(db.as_ref()).await {
            Ok(Some(r)) => r,
            Ok(None) => {
                error!("Record with ID {} not found after insert", id);
                continue;
            }
            Err(e) => {
                error!("Failed to fetch record {}: {}", id, e);
                continue;
            }
        };

        // Update status to PROCESSING (lock)
        if let Err(e) = update_status(db.as_ref(), id, "PROCESSING").await {
            error!("Failed to update status to PROCESSING for ID {}: {}", id, e);
            continue;
        }

        // Decrypt TIN and BHF_ID
        let decrypted_tin = match decrypt_deterministic(&record.tin) {
            Ok(t) => t,
            Err(e) => {
                error!("Failed to decrypt TIN for record {}: {}", id, e);
                mark_as_failed_with_retry(db.as_ref(), id, 0).await.ok();
                continue;
            }
        };

        let decrypted_bhf_id = match decrypt_deterministic(&record.bhf_id) {
            Ok(b) => b,
            Err(e) => {
                error!("Failed to decrypt BHF_ID for record {}: {}", id, e);
                mark_as_failed_with_retry(db.as_ref(), id, 0).await.ok();
                continue;
            }
        };

        // Build KRA payload
        let kra_payload = json!({
            "tin": decrypted_tin,
            "bhfId": decrypted_bhf_id,
            "trdInvcNo": record.trd_invc_no,
            "invcNo": record.invc_no,
            "orgInvcNo": record.org_invc_no,
            "custTin": record.cust_tin,
            "custNm": record.cust_nm,
            "salesTyCd": record.sales_ty_cd,
            "rcptTyCd": record.rcpt_ty_cd,
            "pmtTyCd": record.pmt_ty_cd,
            "salesSttsCd": record.sales_stts_cd,
            "cfmDt": record.cfm_dt,
            "salesDt": record.sales_dt,
            "stockRlsDt": record.stock_rls_dt,
            "cnclReqDt": record.cncl_req_dt,
            "cnclDt": record.cncl_dt,
            "rfdDt": record.rfd_dt,
            "rfdRsnCd": record.rfd_rsn_cd,
            "totItemCnt": record.tot_item_cnt,
            "taxblAmtA": record.taxbl_amt_a,
            "taxblAmtB": record.taxbl_amt_b,
            "taxblAmtC": record.taxbl_amt_c,
            "taxblAmtD": record.taxbl_amt_d,
            "taxblAmtE": record.taxbl_amt_e,
            "taxRtA": record.tax_rt_a,
            "taxRtB": record.tax_rt_b,
            "taxRtC": record.tax_rt_c,
            "taxRtD": record.tax_rt_d,
            "taxRtE": record.tax_rt_e,
            "taxAmtA": record.tax_amt_a,
            "taxAmtB": record.tax_amt_b,
            "taxAmtC": record.tax_amt_c,
            "taxAmtD": record.tax_amt_d,
            "taxAmtE": record.tax_amt_e,
            "totTaxblAmt": record.tot_taxbl_amt,
            "totTaxAmt": record.tot_tax_amt,
            "totAmt": record.tot_amt,
            "prchrAcptcYn": record.prchr_acptc_yn,
            "remark": record.remark,
            "regrId": record.regr_id,
            "regrNm": record.regr_nm,
            "modrId": record.modr_id,
            "modrNm": record.modr_nm,
            "receipt": record.receipt,
            "itemList": record.item_list,
        });

        info!("Sending payload to KRA for invoice #{}", record.invc_no);

        // Send to KRA endpoint with timeout
        let client = reqwest::Client::builder()
            .timeout(tokio::time::Duration::from_secs(30))
            .build()
            .unwrap();

        match client
            .post("http://192.168.1.71:8088/trnsSales/saveSales")
            .json(&kra_payload)
            .send()
            .await
        {
            Ok(response) => {
                if !response.status().is_success() {
                    error!("KRA returned error status: {} for record {}", response.status(), id);
                    mark_as_failed_with_retry(db.as_ref(), id, 0).await.ok();
                    continue;
                }

                match response.json::<serde_json::Value>().await {
                    Ok(kra_response) => {
                        info!("Received response from KRA for invoice #{}", record.invc_no);
                        
                        // Update record with response and status
                        if let Err(e) = update_record_with_response(
                            db.as_ref(),
                            id,
                            kra_response,
                            "TRANSMITTED"
                        ).await {
                            error!("Failed to update record {} with KRA response: {}", id, e);
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse KRA response for record {}: {}", id, e);
                        mark_as_failed_with_retry(db.as_ref(), id, 0).await.ok();
                    }
                }
            }
            Err(e) => {
                error!("Failed to send to KRA endpoint for record {}: {}", id, e);
                mark_as_failed_with_retry(db.as_ref(), id, 0).await.ok();
            }
        }
    }

    (
        StatusCode::OK,
        Json(json!({
            "message": "success",
            "resultMsg": "Sales uploaded and transmitted successfully",
            "invoices_created": payload.0.len(),
            "last_invoice_number": current_invoice_number
        })),
    )
}

// Helper function to update status
async fn update_status(
    db: &DatabaseConnection,
    id: i32,
    new_status: &str,
) -> Result<(), sea_orm::DbErr> {
    use sea_orm::ActiveValue::Set;
    
    let record = Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound(format!("ID {}", id)))?;

    let mut active_model: ActiveModel = record.into();
    active_model.status = Set(new_status.to_string());
    active_model.update(db).await?;

    info!("Updated record {} status to {}", id, new_status);
    Ok(())
}

// Helper function to update record with KRA response
async fn update_record_with_response(
    db: &DatabaseConnection,
    id: i32,
    kra_response: serde_json::Value,
    new_status: &str,
) -> Result<(), sea_orm::DbErr> {
    use sea_orm::ActiveValue::Set;
    
    let record = Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound(format!("ID {}", id)))?;

    let mut active_model: ActiveModel = record.into();
    active_model.status = Set(new_status.to_string());
    active_model.response = Set(Some(kra_response));
    active_model.update(db).await?;

    info!("Updated record {} with KRA response and status {}", id, new_status);
    Ok(())
}



// Add this new helper function
async fn mark_as_failed_with_retry(
    db: &DatabaseConnection,
    id: i32,
    current_retry_count: i64,
) -> Result<(), sea_orm::DbErr> {
    use sea_orm::ActiveValue::Set;
    use chrono::Duration as ChronoDuration;

    let record = Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound(format!("ID {}", id)))?;

    // Calculate next retry time with exponential backoff
    let backoff_minutes = 2_i64.pow((current_retry_count + 1) as u32);
    let next_retry = Utc::now() + ChronoDuration::minutes(backoff_minutes);

    let mut active_model: ActiveModel = record.into();
    active_model.status = Set("FAILED".to_string());
    active_model.retry_count = Set(Some(current_retry_count));
    active_model.next_retry_at = Set(Some(next_retry.to_rfc3339()));
    active_model.update(db).await?;

    info!("Marked record {} as FAILED, will retry at {}", id, next_retry.format("%Y-%m-%d %H:%M:%S"));
    Ok(())
}