use std::sync::Arc;

use sea_orm::{DatabaseConnection, ColumnTrait, EntityTrait, QueryFilter, ActiveModelTrait};
use tokio::time::{interval, Duration};
use tracing::{info, error};
use chrono::Utc;
use serde_json::json;

use crate::{
    models::sales_uploads::{Entity, ActiveModel, Column},
    utils::crypto::decrypt_deterministic,
};

pub fn start_retry_worker(db: Arc<DatabaseConnection>) {
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(30)); // Check every 30 seconds

        loop {
            ticker.tick().await;
            info!("🔄 Retry worker tick - checking for failed transactions");

            if let Err(e) = retry_failed_transactions(db.as_ref()).await {
                error!("❌ Retry worker error: {}", e);
            }
        }
    });
}

async fn retry_failed_transactions(
    db: &DatabaseConnection,
) -> Result<(), sea_orm::DbErr> {
    use sea_orm::ActiveValue::Set;

    // Find all FAILED records that haven't exceeded retry limit
    // and are ready for retry (next_retry_at is in the past or empty)
    let now = Utc::now().to_rfc3339();
    
    let failed_records = Entity::find()
        .filter(Column::Status.is_in(["FAILED", "PROCESSING"]))
        .filter(Column::RetryCount.lt(5)) // Max 5 retries
        .all(db)
        .await?;

    info!("📋 Found {} failed records to retry", failed_records.len());

    for record in failed_records {
        let id = record.id;
        
        // Check if it's time to retry (exponential backoff)
      if let Some(next_retry_at) = &record.next_retry_at {
    if next_retry_at > &now {
        info!(
            "⏰ Record {} not ready for retry yet (next: {})",
            id, next_retry_at
        );
        continue;
    }
}


        let retry_count = record.retry_count.unwrap_or(0);

info!(
    "🔄 Retrying record {} (attempt {}/5)",
    id,
    retry_count + 1
);


        // 1️⃣ Lock the record with PROCESSING status
        if let Err(e) = update_status(db, id, "PROCESSING").await {
            error!("Failed to lock record {} for processing: {}", id, e);
            continue;
        }

        // 2️⃣ Attempt to resend to KRA
        match resend_to_kra(db, &record).await {
            Ok(true) => {
                // Success - mark as TRANSMITTED
                info!("✅ Successfully transmitted record {}", id);
                if let Err(e) = update_status(db, id, "TRANSMITTED").await {
                    error!("Failed to update status to TRANSMITTED for {}: {}", id, e);
                }
            }
            Ok(false) => {
                // Failed - increment retry count with exponential backoff
                info!("❌ Failed to transmit record {}, will retry later", id);
                if let Err(e) = increment_retry(db, id, record.retry_count.unwrap_or(0)).await
 {
                    error!("Failed to increment retry for {}: {}", id, e);
                }
            }
            Err(e) => {
                error!("Error during retry of record {}: {}", id, e);
                if let Err(e) = increment_retry(db, id, record.retry_count.unwrap_or(0)).await
{
                    error!("Failed to increment retry for {}: {}", id, e);
                }
            }
        }
    }

    Ok(())
}

// Change this function signature
async fn resend_to_kra(
    db: &DatabaseConnection,
    record: &crate::models::sales_uploads::Model,
) -> Result<bool, String> {  // ✅ Changed from Box<dyn std::error::Error>
    // Decrypt TIN and BHF_ID
    let decrypted_tin = decrypt_deterministic(&record.tin)
        .map_err(|e| format!("Decrypt TIN error: {}", e))?;
    
    let decrypted_bhf_id = decrypt_deterministic(&record.bhf_id)
        .map_err(|e| format!("Decrypt BHF_ID error: {}", e))?;

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

    info!("📤 Sending retry payload to KRA for invoice #{}", record.invc_no);

    // Send to KRA endpoint
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Client build error: {}", e))?;

    let response = client
        .post("http://192.168.1.71:8088/trnsSales/saveSales")
        .json(&kra_payload)
        .send()
        .await
        .map_err(|e| format!("Request send error: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        error!("KRA returned error status: {}", status);
        return Ok(false);
    }

    let kra_response = response.json::<serde_json::Value>()
        .await
        .map_err(|e| format!("JSON parse error: {}", e))?;
    
    info!("📥 Received response from KRA for invoice #{}", record.invc_no);

    // Save response to database
    update_record_with_response(db, record.id, kra_response, "TRANSMITTED")
        .await
        .map_err(|e| format!("DB update error: {}", e))?;

    Ok(true)
}

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

    info!("✏️ Updated record {} status to {}", id, new_status);
    Ok(())
}

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
    active_model.next_retry_at = Set(None);
 // Clear retry timestamp
    active_model.update(db).await?;

    info!("✅ Updated record {} with KRA response and status {}", id, new_status);
    Ok(())
}

async fn increment_retry(
    db: &DatabaseConnection,
    id: i32,
    current_retry_count: i64,
) -> Result<(), sea_orm::DbErr> {
    use sea_orm::ActiveValue::Set;
    use chrono::Duration as ChronoDuration;

    let record = Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound("not found".into()))?;

    let new_retry_count = current_retry_count + 1;
    
    // Exponential backoff: 2^retry_count minutes
    // Retry 1: 2 min, Retry 2: 4 min, Retry 3: 8 min, Retry 4: 16 min, Retry 5: 32 min
    let backoff_minutes = 2_i64.pow(new_retry_count as u32);
    let next_retry = Utc::now() + ChronoDuration::minutes(backoff_minutes);



    // Entity::update_many()
    // .col_expr(Column::RetryCount, Expr::value(new_retry_count))
    // .col_expr(Column::Status, Expr::value("FAILED"))
    // .col_expr(Column::NextRetryAt, Expr::value(next_retry))
    // .filter(Column::Id.eq(record.id))
    // .exec(db)
    // .await?;


    let mut model: ActiveModel = record.into();
    model.retry_count = Set(Some(new_retry_count));
    model.status = Set("FAILED".to_string());
    model.next_retry_at = Set(Some(next_retry.to_rfc3339()));
    model.update(db).await?;

    info!("📊 Record {} retry count: {}/5, next retry at: {}", 
          id, new_retry_count, next_retry.format("%Y-%m-%d %H:%M:%S"));

    Ok(())
}