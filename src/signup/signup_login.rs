use std::sync::Arc;
use bcrypt::{DEFAULT_COST, hash, verify};
use axum::{
    Json, Router, extract::State, response::IntoResponse,
    routing::post
};
use serde::Deserialize;
use rand::{distributions::Alphanumeric, Rng};
use serde_json::json;
use tracing::{error, info, warn};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use crate::models::sign_up::{Entity as UserEntity, Model};
use crate::models::sign_up::Entity;
use crate::{
    models::sign_up::{ActiveModel, Column},
    types::signup::{LoginPayload, SignUpPayload}
};

/// Register signup route
pub fn sign_up(db: Arc<DatabaseConnection>) -> Router {
    Router::new()
        .route("/", post(signup_handler))
        .with_state(db)
}
pub fn log_in(db: Arc<DatabaseConnection>) -> Router {
    Router::new()
        .route("/", post(login_handler))
        .with_state(db)
}

pub fn log_in_users(db: Arc<DatabaseConnection>) -> Router {
    Router::new()
        .route("/", post(login_handler_fetch))
        .with_state(db)
}

/// Signup handler
pub async fn signup_handler(
    State(db): State<Arc<DatabaseConnection>>,
    Json(payload): Json<SignUpPayload>,
) -> impl IntoResponse {
    info!("Received signup payload: {:?}", payload);

    // Hash the password
    let hashed_password = match hash_password(&payload.password) {
        Ok(h) => h,
        Err(e) => {
            info!("Error hashing password: {}", e);
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Error processing request").into_response();
        }
    };

    let token: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(64)
    .map(char::from)
    .collect();

    // Prepare new user ActiveModel
    let new_user = ActiveModel {
        full_name: sea_orm::ActiveValue::Set(payload.fullName),
        email: sea_orm::ActiveValue::Set(payload.email),
        phone_number: sea_orm::ActiveValue::Set(payload.phoneNumber),
        password_hash: sea_orm::ActiveValue::Set(hashed_password),
        tracking_id: sea_orm::ActiveValue::Set(Some(token)),
        role: sea_orm::ActiveValue::Set("user".to_string()),
        status: sea_orm::ActiveValue::Set("Pending Approval".to_string()),
        agreement: sea_orm::ActiveValue::Set(payload.agreement),
        ..Default::default()
    };

    // Insert into DB
    match new_user.insert(db.as_ref()).await {
        Ok(user) => {
            info!("User created with ID: {:?}", user.id);
            (axum::http::StatusCode::CREATED, "User created successfully").into_response()
        }
        Err(e) => {
            info!("Error creating user: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Error creating user").into_response()
        }
    }
}

/// Hash a password
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

/// Verify password
pub fn verify_password(
    password: &str,
    hashed_password: &str,
) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hashed_password)
}


/// Login handler
pub async fn login_handler(
    State(db): State<Arc<DatabaseConnection>>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    info!("Login attempt for email: {}", payload.email);

    // 1️⃣ Find user by email
    let user = match UserEntity::find()
        .filter(Column::Email.eq(payload.email))
        .one(db.as_ref())
        .await
    {
        Ok(Some(u)) => u,
        Ok(None) => {
            return (
                axum::http::StatusCode::NOT_FOUND,
                Json(json!({ "message": "User not found" })),
            )
                .into_response();
        }
        Err(e) => {
            info!("Error querying user: {}", e);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Error processing request" })),
            )
                .into_response();
        }
    };

    // 2️⃣ Verify password
    match verify_password(&payload.password, &user.password_hash) {
        Ok(true) => {
             info!("Loaded tracking_id: {:?}", user.tracking_id);
            (
                axum::http::StatusCode::OK,
                Json(json!({
                    "message": "Login successful",
                    "tracking_id": user.tracking_id
                })),
            )
                .into_response()

        }
        Ok(false) => {
            (
                axum::http::StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "Invalid credentials" })),
            )
                .into_response()
        }
        Err(e) => {
            info!("Error verifying password: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Error processing request" })),
            )
                .into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct UserFetch {
    pub tracing_id: String,
}

pub async fn login_handler_fetch(
    State(db): State<Arc<DatabaseConnection>>,
    Json(data): Json<UserFetch>,
) -> impl IntoResponse {
    info!("Login fetch via POST for tracing_id: {}", data.tracing_id);

  match Entity::find()
    .filter(Column::TrackingId.eq(data.tracing_id))
    .all(db.as_ref())
    .await
{
    Ok(users) => {
        info!("Fetched users: {:?}", users);
        (
            axum::http::StatusCode::OK,
            Json(json!({ "users": users })),
        )
            .into_response()
    }
    Err(err) => {
        error!("DB error fetching user by tracking_id {err}");
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,   // ← change to 500
            Json(json!({
                "message": "Failed to fetch user data",
                "error": err.to_string(),                     // optional: expose in dev only
            })),
        )
            .into_response()
    }
}
}


