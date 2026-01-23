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
    }, types::initializeTypes::InitializeData, utils::crypto::{encrypt, encrypt_deterministic}
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

    // 🔹 Deterministic encryption for fields used in uniqueness checks
    let encrypted_serial = encrypt_deterministic(&payload.deviceSerial);
    let encrypted_pin = encrypt_deterministic(&payload.pin);

    // 🔹 1. Check device serial uniqueness
    if let Ok(Some(_)) = Credentials::find()
        .filter(Column::DeviceSerial.eq(&encrypted_serial))
        .one(db.as_ref())
        .await
    {
        return Json(json!({
            "status": "error",
            "message": "Device serial number already exists"
        }));
    }

    // 🔹 2. Check PIN uniqueness in this environment
    if let Ok(Some(_)) = Credentials::find()
        .filter(Column::Pin.eq(&encrypted_pin))
        .filter(Column::EnvironmentName.eq(&payload.environmentName))
        .one(db.as_ref())
        .await
    {
        return Json(json!({
            "status": "error",
            "message": "PIN already exists in this environment"
        }));
    }

    // 🔹 3. Generate API key and encrypt other fields
    let api_key: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let encrypted_api_key = encrypt(&api_key);
    let encrypted_branch = encrypt(&payload.branchId);
    let encrypted_company = encrypt(&payload.companyId);

    // 🔹 4. Insert
    let device = ActiveModel {
        company_id: sea_orm::ActiveValue::Set(encrypted_company),
        environment_name: sea_orm::ActiveValue::Set(payload.environmentName),
        environment_url: sea_orm::ActiveValue::Set(payload.environmentUrl),
        pin: sea_orm::ActiveValue::Set(encrypted_pin),
        branch_id: sea_orm::ActiveValue::Set(encrypted_branch),
        device_serial: sea_orm::ActiveValue::Set(encrypted_serial),
        api_key: sea_orm::ActiveValue::Set(encrypted_api_key),
        ..Default::default()
    };

    match device.insert(db.as_ref()).await {
        Ok(res) => Json(json!({
            "status": "success",
            "data": {
                "id": res.id,
                "api_key": api_key
            }
        })),
        Err(e) => {
            // 🔹 Catch DB UNIQUE violation (race condition)
            if e.to_string().contains("uniq_device_serial") {
                return Json(json!({
                    "status": "error",
                    "message": "Device serial number already exists (DB constraint)"
                }));
            }
            Json(json!({
                "status": "error",
                "message": e.to_string()
            }))
        }
    }
}
