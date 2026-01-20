use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use chrono::Utc;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, TransactionTrait};
use serde::{Deserialize, Serialize};
use serde_json::{json};
use std::sync::Arc;

use crate::{
    models::stock_master::ActiveModel,
    types::stock_management::{StockMasterItem, StockMstSaveReq},
};

// ── Router ─────────────────────────────────────────────────────────────────────
pub fn master_router(db: Arc<DatabaseConnection>) -> Router {
    Router::new()
        .route("/", post(create_stock_items).get(list_stock_items))
        .with_state(db)
}

// ── Handlers ───────────────────────────────────────────────────────────────────
async fn create_stock_items(
    State(db): State<Arc<DatabaseConnection>>,
    Json(payload): Json<StockMstSaveReq>,
) -> impl IntoResponse {
    let items: Vec<StockMasterItem> = payload.0;

    if items.is_empty() {
        return error_response("No items provided", StatusCode::BAD_REQUEST);
    }

    // Start transaction
    let mut txn = match db.begin().await {
        Ok(t) => t,
        Err(e) => return error_response(&format!("Failed to start transaction: {e}"), StatusCode::INTERNAL_SERVER_ERROR),
    };

    for item in items.iter() {
        let qty = match Decimal::try_from(item.rsd_qty) {
            Ok(d) => d,
            Err(e) => {
                let _ = txn.rollback().await;
                return error_response(&format!("Invalid quantity for {}: {e}", item.item_cd), StatusCode::BAD_REQUEST);
            }
        };

        let active = ActiveModel {
            id: ActiveValue::NotSet,
            tin: ActiveValue::Set(item.tin.clone()),
            bhf_id: ActiveValue::Set(item.bhf_id.clone()),
            item_cd: ActiveValue::Set(item.item_cd.clone()),
            rsd_qty: ActiveValue::Set(qty),
            regr_nm: ActiveValue::Set(item.regr_nm.clone()),
            regr_id: ActiveValue::Set(item.regr_id.clone()),
            modr_nm: ActiveValue::Set(item.modr_nm.clone()),
            modr_id: ActiveValue::Set(item.modr_id.clone()),
            ..Default::default()
        };

        if let Err(e) = active.insert(&mut txn).await {
            let _ = txn.rollback().await;
            return error_response(&format!("Insert failed for {}: {e}", item.item_cd), StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // Commit transaction
    match txn.commit().await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({"resultCd": "000", "resultMsg": "All items inserted successfully"})),
        ),
        Err(e) => error_response(&format!("Transaction commit failed: {e}"), StatusCode::INTERNAL_SERVER_ERROR),
    }
}


// ── Error helper ───────────────────────────────────────────────────────────────
pub fn error_response(message: &str, code: StatusCode) -> (StatusCode, Json<serde_json::Value>) {
    (code, Json(json!({
        "resultCd": code.as_u16().to_string(),
        "resultMsg": message,
    })))
}
async fn list_stock_items(
    State(db): State<Arc<DatabaseConnection>>,
) -> impl IntoResponse {
    match crate::models::stock_master::Entity::find().all(db.as_ref()).await {
        Ok(records) => (
            StatusCode::OK,
            Json(json!({
                "resultCd": "000",
                "resultMsg": "Success",
                "data": records,
            })),
        ),
        Err(e) => error_response(&format!("Failed to fetch records: {e}"), StatusCode::INTERNAL_SERVER_ERROR),
    }
}