use axum::{Json, Router, extract::State, response::IntoResponse, routing::{Route, post}};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tracing::{info, error, warn};
use crate::models::sales_data_model::Entity;
use crate::types::salespayloadtype::{TrnsSalesSaveReq,ReceiptInfo, SalesItem};
pub fn route_sales(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/", post(post_sales).get(get_sales))
        .with_state(db)
}


// before doing anything i would like to save the data in my own database then forward it to govt server
pub async fn post_sales(
    State(db): State<DatabaseConnection>,
    Json(mut payload): Json<TrnsSalesSaveReq>,
) -> impl IntoResponse {
info!(
        tin = %payload.tin,
        bhf_id = %payload.bhf_id,
        invc_no = payload.invc_no,
        "Received sales transaction"
    );
    // 1️⃣ Enforce item sequencing
    for (i, item) in payload.item_list.iter_mut().enumerate() {
        item.item_seq = (i + 1) as i32;
    }

    // 2️⃣ Validate item count
    if payload.tot_item_cnt != payload.item_list.len() as i32 {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            "Item count mismatch",
        );
    }

    // 3️⃣ Save locally (PENDING)
    let sale = ActiveModel {
        invc_no: Set(payload.invc_no),
        trd_invc_no: Set(payload.trd_invc_no.clone()),
        payload: Set(serde_json::to_value(&payload).unwrap()),
        status: Set("PENDING".to_string()),
        ..Default::default()
    };

    let saved = match sale.insert(&db).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Local save failed",
            );
        }
    };

    // 4️⃣ Send to VSCU
    let client = reqwest::Client::new();
    let response = client
        .post("http://192.168.1.71:8088/trnsSales/saveSales")
        .json(&payload)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await;

    let res = match response {
        Ok(r) => r,
        Err(e) => {
            eprintln!("VSCU connection error: {:?}", e);
            return (
                axum::http::StatusCode::BAD_GATEWAY,
                "Failed to reach VSCU",
            );
        }
    };

    let body = res.text().await.unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();

    // 5️⃣ Check result code
    if parsed["resultCd"] != "000" {
        return (
            axum::http::StatusCode::BAD_GATEWAY,
            body,
        );
    }

    // 6️⃣ Save VSCU response
    let mut update = saved.into_active_model();
    update.status = Set("CONFIRMED".to_string());
    update.vscu_response = Set(Some(parsed.clone()));

    update.update(&db).await.ok();

    (
        axum::http::StatusCode::OK,
        Json(parsed),
    )
}

pub async fn get_sales(State(db):State<DatabaseConnection>) -> impl IntoResponse {
   match Entity::find().all(&db).await {
        Ok(sales) => (axum::http::StatusCode::OK, Json(sales)),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
    }
}