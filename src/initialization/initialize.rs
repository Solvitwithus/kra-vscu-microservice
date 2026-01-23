use std::sync::Arc;

use aes_gcm::aead::Payload;
use axum::{
    Json, Router,
    extract::State,
    response::IntoResponse,
    routing::post,
};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection,
    EntityTrait, ColumnTrait, QueryFilter,
};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::json;
use tracing::info;

use crate::{
    models::initialization::{
        ActiveModel, Column, Entity as Credentials
    }, types::initializeTypes::InitializeData, utils::crypto::encrypt
};

pub fn initialization_route(db: Arc<DatabaseConnection>) -> Router {
    Router::new()
        .route("/", post(initialize_system))
        .with_state(db)
}

pub async fn initialize_system(
    State(db): State<Arc<DatabaseConnection>>,
    Json(payload): Json<InitializeData>,
) -> impl IntoResponse {

    info!("Starting initialization");

    // 1️⃣ Check if device serial already exists
    match Credentials::find()
        .filter(Column::DeviceSerial.eq(&payload.deviceSerial))
        .one(db.as_ref())
        .await
    {
        Ok(Some(_)) => {
            return Json(json!({"status": "error", "message": "Device serial number already exists"}))
        },
        Err(e) => {
            return Json(json!({"status": "error", "message": e.to_string()}))
        },
        _ => {}
    }

    // 2️⃣ Check if PIN exists in this environment
    match Credentials::find()
        .filter(Column::Pin.eq(&payload.pin))
        .filter(Column::EnvironmentName.eq(&payload.environmentName))
        .one(db.as_ref())
        .await
    {
        Ok(Some(_)) => {
            return Json(json!({"status": "error", "message": "PIN already exists in this environment"}))
        },
        Err(e) => {
            return Json(json!({"status": "error", "message": e.to_string()}))
        },
        _ => {}
    }

    // 3️⃣ Generate API key
    let api_key: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    // 4️⃣ Encrypt PIN (raw or hashed)
    let encrypted_pin = encrypt(&payload.pin);
    let encrypted_serial = encrypt(&payload.deviceSerial);
    let encrypted_id = encrypt(&payload.branchId);
    let encrypted_comp_id = encrypt(&payload.companyId);
    let encrypted_key= encrypt(&api_key);

    // 5️⃣ Insert device
    let device = ActiveModel {
        company_id: sea_orm::ActiveValue::Set(encrypted_comp_id),
        environment_name: sea_orm::ActiveValue::Set(payload.environmentName),
        environment_url: sea_orm::ActiveValue::Set(payload.environmentUrl),
        pin: sea_orm::ActiveValue::Set(encrypted_pin),
        branch_id: sea_orm::ActiveValue::Set(encrypted_id),
        device_serial: sea_orm::ActiveValue::Set(encrypted_serial),
        api_key: sea_orm::ActiveValue::Set(encrypted_key),
        ..Default::default()
    };

    match device.insert(db.as_ref()).await {
        Ok(res) => Json(json!({
            "status": "success",
            "data": {
                "id": res.id,
                "api_key": api_key // return generated API key
            }
        })),
        Err(e) => Json(json!({
            "status": "error",
            "message": e.to_string()
        })),
    }
}

