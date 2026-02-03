use std::sync::Arc;

use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use reqwest::StatusCode;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, TransactionTrait};
use serde_json::json;
use tracing::{info, error};
use crate::{
    models::product_save_items::{ActiveModel, Entity}, 
    stock_management::route_stock_master::error_response, 
    types::{
        product_management_payload_types::ItemSaveReq, 
        salespayloadtype::AuthUser
    }, utils::{bearer::bearer_resolver, crypto::{decrypt, decrypt_deterministic}}, 
    
};

pub fn items_save_items_router(db: Arc<DatabaseConnection>) -> Router {
    Router::new()
        .route("/", post(save_item))
        .with_state(db)
}

async fn save_item(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    State(db): State<Arc<DatabaseConnection>>,
    Json(payload): Json<ItemSaveReq>,
) -> impl IntoResponse {
    let mut inserted_ids: Vec<i64> = Vec::new();
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

    // 2️⃣ START TRANSACTION & INSERT ALL ITEMS
    let items = payload.0;
    let items_count = items.len();
    
    let mut txn = match db.begin().await {
        Ok(t) => t,
        Err(e) => return error_response(
            &format!("Failed to start transaction: {e}"), 
            StatusCode::INTERNAL_SERVER_ERROR
        ),
    };

    for item in items {
        let stats = "inserted".to_string();
        let model = ActiveModel {
            tin: Set(Some(user.pin.clone())),
            bhf_id: Set(Some(user.branch_id.clone())),
            status: Set(Some(stats)),
            item_cd: Set(item.item_cd.clone()),
            item_cls_cd: Set(item.item_cls_cd),
            item_ty_cd: Set(item.item_ty_cd),
            item_nm: Set(item.item_nm),
            item_std_nm: Set(item.item_std_nm),
            orgn_nat_cd: Set(item.orgn_nat_cd),
            pkg_unit_cd: Set(item.pkg_unit_cd),
            qty_unit_cd: Set(item.qty_unit_cd),
            tax_ty_cd: Set(item.tax_ty_cd),
            btch_no: Set(item.btch_no),
            bcd: Set(item.bcd),
            dft_prc: Set(item.dft_prc),
            grp_prc_l1: Set(item.grp_prc_l1),
            grp_prc_l2: Set(item.grp_prc_l2),
            grp_prc_l3: Set(item.grp_prc_l3),
            grp_prc_l4: Set(item.grp_prc_l4),
            grp_prc_l5: Set(item.grp_prc_l5),
            add_info: Set(item.add_info),
            sfty_qty: Set(item.sfty_qty),
            isrc_aplcb_yn: Set(item.isrc_aplcb_yn),
            use_yn: Set(item.use_yn),
            regr_nm: Set(item.regr_nm),
            regr_id: Set(item.regr_id),
            modr_nm: Set(item.modr_nm),
            modr_id: Set(item.modr_id),
            ..Default::default()
        };

        match model.insert(&mut txn).await {
            Ok(inserted) => {
                inserted_ids.push(inserted.id);
                info!("Inserted item with ID: {}", inserted.id);
            },
            Err(e) => {
                let _ = txn.rollback().await;
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "message": format!("Insert failed: {e}") })),
                );
            }
        }
    }

    // 3️⃣ COMMIT TRANSACTION
    match txn.commit().await {
        Ok(_) => {
            info!("Transaction committed successfully. {} items inserted.", inserted_ids.len());
        },
        Err(e) => {
            return error_response(
                &format!("Transaction commit failed: {e}"), 
                StatusCode::INTERNAL_SERVER_ERROR
            );
        }
    }

    // 4️⃣ BACKGROUND PROCESSING - Send to KRA
    // Spawn async task so we don't block the response
    let db_clone = db.clone();
    tokio::spawn(async move {
        process_kra_submissions(db_clone, inserted_ids).await;
    });

    // 5️⃣ RETURN SUCCESS IMMEDIATELY
    (
        StatusCode::OK,
        Json(json!({
            "resultCd": "000", 
            "resultMsg": "All items inserted successfully",
            "items_created": items_count
        })),
    )
}

// FIXED: Changed return type to () instead of Result<()> to avoid Send trait issues
async fn process_kra_submissions(db: Arc<DatabaseConnection>, inserted_ids: Vec<i64>) {
    for id in inserted_ids {
        // Fetch the record we just inserted
        let record = match Entity::find_by_id(id).one(db.as_ref()).await {
            Ok(Some(r)) => {
                info!("Processing record ID: {}", id);
                r
            },
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

        // Decrypt TIN
        let decrypted_tin = match decrypt_deterministic(&record.tin.clone().unwrap_or_default()) {
            Ok(d) => {
                info!("Decrypt tin OK for record {}", id);
                d
            }
            Err(e) => {
                error!("Failed to decrypt TIN for record {}: {}", id, e);
                let _ = mark_as_failed_with_retry(db.as_ref(), id, 0).await;
                continue;
            }
        };

        // Decrypt BHF_ID
        let decrypted_bhf_id = match decrypt(&record.bhf_id.clone().unwrap_or_default()) {
            Ok(d) => d,
            Err(e) => {
                error!("Failed to decrypt BHF_ID for record {}: {}", id, e);
                let _ = mark_as_failed_with_retry(db.as_ref(), id, 0).await;
                continue;
            }
        };

        // Build KRA payload with all necessary fields
        let kra_payload = json!({
            "tin": decrypted_tin,
            "bhfId": decrypted_bhf_id,
            "itemCd": record.item_cd,
            "itemClsCd": record.item_cls_cd,
            "itemTyCd": record.item_ty_cd,
            "itemNm": record.item_nm,
            "itemStdNm": record.item_std_nm,
            "orgnNatCd": record.orgn_nat_cd,
            "pkgUnitCd": record.pkg_unit_cd,
            "qtyUnitCd": record.qty_unit_cd,
            "taxTyCd": record.tax_ty_cd,
            "btchNo": record.btch_no,
            "bcd": record.bcd,
            "dftPrc": record.dft_prc,
            "grpPrcL1": record.grp_prc_l1,
            "grpPrcL2": record.grp_prc_l2,
            "grpPrcL3": record.grp_prc_l3,
            "grpPrcL4": record.grp_prc_l4,
            "grpPrcL5": record.grp_prc_l5,
            "addInfo": record.add_info,
            "sftyQty": record.sfty_qty,
            "isrcAplcbYn": record.isrc_aplcb_yn,
            "useYn": record.use_yn,
            "regrNm": record.regr_nm,
            "regrId": record.regr_id,
            "modrNm": record.modr_nm,
            "modrId": record.modr_id,
        });

        // Send to KRA
        let client = match reqwest::Client::builder()
            .timeout(tokio::time::Duration::from_secs(30))
            .build()
        {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to build HTTP client: {}", e);
                continue;
            }
        };

        match client
            .post("http://192.168.1.71:8088/items/saveItems")
            .json(&kra_payload)
            .send()
            .await
        {
            Ok(response) => {
                if !response.status().is_success() {
                    error!("KRA returned error status: {} for record {}", response.status(), id);
                    let _ = mark_as_failed_with_retry(db.as_ref(), id, 0).await;
                    continue;
                }

                match response.json::<serde_json::Value>().await {
                    Ok(kra_response) => {
                        info!("Received response from KRA for item ID: {}", id);
                        
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
                        let _ = mark_as_failed_with_retry(db.as_ref(), id, 0).await;
                    }
                }
            }
            Err(e) => {
                error!("Failed to send to KRA endpoint for record {}: {}", id, e);
                let _ = mark_as_failed_with_retry(db.as_ref(), id, 0).await;
            }
        }
    }
    
    info!("Completed KRA submission processing for all items");
}

async fn update_status(
    db: &DatabaseConnection,
    id: i64,
    new_status: &str,
) -> Result<(), sea_orm::DbErr> {
    use sea_orm::ActiveValue::Set;
    
    let record = Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound(format!("ID {}", id)))?;

    let mut active_model: ActiveModel = record.into();
    active_model.status = Set(Some(new_status.to_string()));
    active_model.update(db).await?;

    info!("Updated record {} status to {}", id, new_status);
    Ok(())
}

async fn update_record_with_response(
    db: &DatabaseConnection,
    id: i64,
    kra_response: serde_json::Value,
    new_status: &str,
) -> Result<(), sea_orm::DbErr> {
    use sea_orm::ActiveValue::Set;
    
    let record = Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound(format!("ID {}", id)))?;

    let mut active_model: ActiveModel = record.into();
    active_model.status = Set(Some(new_status.to_string()));
    active_model.response = Set(Some(kra_response));
    active_model.update(db).await?;

    info!("Updated record {} with KRA response and status {}", id, new_status);
    Ok(())
}

async fn mark_as_failed_with_retry(
    db: &DatabaseConnection,
    id: i64,
    retry_count: i32,
) -> Result<(), sea_orm::DbErr> {
    use sea_orm::ActiveValue::Set;
    
    let record = Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound(format!("ID {}", id)))?;

    let mut active_model: ActiveModel = record.into();
    active_model.status = Set(Some("FAILED".to_string()));
    // Add retry count field if you have it in your schema
    // active_model.retry_count = Set(retry_count);
    active_model.update(db).await?;

    error!("Marked record {} as FAILED with retry count {}", id, retry_count);
    Ok(())
}