use std::sync::Arc;

use axum::{
    Json,
    Router,
    extract::State,
    response::IntoResponse,
    routing::post,
};
use axum_extra::extract::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

use crate::utils::bearer::bearer_resolver;

pub fn sales_route(db: Arc<DatabaseConnection>) -> Router {
    Router::new()
        .route("/", post(handle_payload_post))
        .with_state(db)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MyData {
    pub name: String,
    pub age: String,
    pub item_name: String,
}


pub async fn handle_payload_post(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    State(db): State<Arc<DatabaseConnection>>,
    Json(payload): Json<MyData>,
) -> impl IntoResponse {
    let token = auth.token();
    info!("Bearer token received");
    info!("Payload: {:?}", payload);

    match bearer_resolver(token, db.as_ref()).await {
        Ok(user) => Json(json!({
            "status": "success",
            "data": {
                "payload": payload,
                "user": user
            }
        })),
        Err(err) => Json(json!({
            "status": "error",
            "message": err
        })),
    }
}
