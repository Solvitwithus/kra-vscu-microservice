use std::sync::Arc;

use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use reqwest::StatusCode;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, TransactionTrait};
use serde_json::json;
use tracing::info;

use crate::{models::product_save_items::{ActiveModel, Entity}, stock_management::route_stock_master::error_response, types::{product_management_payload_types::ItemSaveReq, salespayloadtype::AuthUser}, utils::bearer::bearer_resolver};

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
    // Handler logic here
    let items = payload.0;
    let mut txn = match db.begin().await {
        Ok(t) => t,
        Err(e) => return error_response(&format!("Failed to start transaction: {e}"), StatusCode::INTERNAL_SERVER_ERROR),
    };

    for  item in items {
        let model = ActiveModel{
            tin: Set(Some(user.pin.clone())),
            bhf_id: Set(Some(user.branch_id.clone())),
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

 if let Err(e) = model.insert(&mut txn).await {
            let _ = txn.rollback().await;
            return error_response(&format!("Insert failed for {}: {e}", item.item_cd.clone()), StatusCode::INTERNAL_SERVER_ERROR);
        }

    }

match txn.commit().await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({"resultCd": "000", "resultMsg": "All items inserted successfully"})),
        ),
        Err(e) => error_response(&format!("Transaction commit failed: {e}"), StatusCode::INTERNAL_SERVER_ERROR),
    }
}

