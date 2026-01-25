use std::sync::Arc;
use axum::{Json, extract::State, response::IntoResponse};
use axum_extra::extract::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use serde_json::{Value, json};
use serde::{Serialize, Deserialize};
use tracing::info;

use crate::models::initialization::{Column, Entity};

// First handler - receives Bearer token from header
pub async fn receive_payload(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<Value>,
    State(db): State<Arc<DatabaseConnection>>,
) -> impl IntoResponse {
    let token = auth.token();
    info!("Bearer token: {}", token);
    info!("Payload: {:?}", payload);
    
    // Call the resolver with the token
    bearer_resolver(token.to_string(), State(db)).await
}

// Second handler - validates token against database
pub async fn bearer_resolver(
    token: String,
    State(db): State<Arc<DatabaseConnection>>,
) -> impl IntoResponse {
    match Entity::find()
        .filter(Column::ApiKey.eq(&token))
        .one(db.as_ref())
        .await
    {
        Ok(Some(c)) => Json(json!({
            "message": "success",
            "data": c
        })),
        Ok(None) => Json(json!({
            "message": "user not found check your key"
        })),
        Err(e) => Json(json!({
            "message": "an error occurred",
            "detail": e.to_string()
        })),
    }
}

// Alternative: Standalone bearer resolver that can be called from routes
pub async fn bearer_resolver_endpoint(
    Json(payload): Json<Token>,
    State(db): State<Arc<DatabaseConnection>>,
) -> impl IntoResponse {
    match Entity::find()
        .filter(Column::ApiKey.eq(&payload.token))
        .one(db.as_ref())
        .await
    {
        Ok(Some(c)) => Json(json!({
            "message": "success",
            "data": c
        })),
        Ok(None) => Json(json!({
            "message": "user not found check your key"
        })),
        Err(e) => Json(json!({
            "message": "an error occurred",
            "detail": e.to_string()
        })),
    }
}

#[derive(Deserialize)]
pub struct Token {
    token: String,
}