use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use serde_json::json;
use rust_decimal::Decimal;

use crate::{
    models::stock_master::{ActiveModel as StockActiveModel, Entity as StockEntity},
    types::stock_management::{StockMstSaveReq, StockMasterItem},
};

/// Create the router for stock master endpoints
pub fn router(db: Arc<DatabaseConnection>) -> axum::Router {
    axum::Router::new()
        .route("/", post(post_stock_master).get(get_stock_master))
        .with_state(db)
}

/// Helper: Error response
fn error_response(msg: &str, status: StatusCode) -> impl IntoResponse {
    let body = json!({
        "resultCd": if status == StatusCode::OK { "000" } else { "500" },
        "resultMsg": msg,
        "resultDt": chrono::Utc::now().format("%Y%m%d%H%M%S").to_string(),
        "data": serde_json::Value::Null,
    });

    (status, Json(body))
}

/// Helper: Success response
fn success_response(msg: &str, data: Option<serde_json::Value>) -> impl IntoResponse {
    let body = json!({
        "resultCd": "000",
        "resultMsg": msg,
        "resultDt": chrono::Utc::now().format("%Y%m%d%H%M%S").to_string(),
        "data": data.unwrap_or(serde_json::Value::Null),
    });

    (StatusCode::OK, Json(body))
}

/// POST /stock_master
pub async fn post_stock_master(
    State(db): State<Arc<DatabaseConnection>>,
    Json(payload): Json<StockMstSaveReq>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // Normalize payload into Vec<StockMasterItem>
    let items: Vec<StockMasterItem> = match payload {
        StockMstSaveReq::Single(item) => vec![item],
        StockMstSaveReq::Multiple(items) => items,
    };

    if items.is_empty() {
        return Err(error_response("No items provided", StatusCode::BAD_REQUEST));
    }

    // Start transaction
    let txn = db
        .begin()
        .await
        .map_err(|e| error_response(&format!("Failed to start transaction: {}", e), StatusCode::INTERNAL_SERVER_ERROR))?;

    // Insert all items
    for item in items {
        let active_model = StockActiveModel {
            id: ActiveValue::NotSet, // auto-increment
            tin: ActiveValue::Set(item.tin),
            bhf_id: ActiveValue::Set(item.bhf_id),
            item_cd: ActiveValue::Set(item.item_cd),
            rsd_qty: ActiveValue::Set(Decimal::from_f64(item.rsd_qty).unwrap_or_default()),
            regr_nm: ActiveValue::Set(item.regr_nm),
            regr_id: ActiveValue::Set(item.regr_id),
            modr_nm: ActiveValue::Set(item.modr_nm),
            modr_id: ActiveValue::Set(item.modr_id),
            ..Default::default()
        };

        if let Err(e) = active_model.insert(&txn).await {
            let _ = txn.rollback().await;
            return Err(error_response(
                &format!("Failed to insert stock item {}: {}", item.item_cd, e),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    }

    // Commit transaction
    txn.commit()
        .await
        .map_err(|e| error_response(&format!("Failed to commit transaction: {}", e), StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(success_response(
        &format!("Successfully saved {} stock item(s)", items.len()),
        None,
    ))
}

/// GET /stock_master
pub async fn get_stock_master(
    State(db): State<Arc<DatabaseConnection>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match StockEntity::find().all(&*db).await {
        Ok(records) => Ok(success_response("Stock masters retrieved", Some(json!(records)))),
        Err(e) => Err(error_response(
            &format!("Failed to fetch stock masters: {}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}
