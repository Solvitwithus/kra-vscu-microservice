use std::sync::Arc;

use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use sea_orm::{DatabaseConnection, TransactionTrait};
use tracing::info;

use crate::{stock_management::route_stock_master::error_response, types::info::VerificationInfo};

pub fn items_save_items_router(db: Arc<DatabaseConnection>) -> Router {
    Router::new()
        .route("/", post(save_item).get(get_items))
        .with_state(db)
}

async fn save_item(
    State(db): State<Arc<DatabaseConnection>>,
    Json(payload): Json<VerificationInfo>,
) -> impl IntoResponse {
    // // Handler logic here
    // info!("Received payload: {:?}", payload);
    // info!("Database connection established. processing starting...");

    // let mut initalize =match db.begin().await {
    //     Ok(t) => t,
    //     Err(e) => return error_response(&format!("Failed to start transaction: {e}"), StatusCode::INTERNAL_SERVER_ERROR),
    // }

    // let data = Ac


    //use reqwest to fetch this data instead of creating a whole complex thing since i do not intend to store the data in this particulat handler
}

async fn get_items(
    State(db): State<Arc<DatabaseConnection>>,
) -> impl IntoResponse {
    // Handler logic here
}