use std::sync::Arc;
use bcrypt::{DEFAULT_COST, hash, verify};
use axum::{
    Json, Router, extract::State, response::{IntoResponse, Response},
    routing::post, http::StatusCode,
};
use crate::utils::crypto::{decrypt_deterministic, encrypt_deterministic};
use serde::Deserialize;
use rand::{distributions::Alphanumeric, Rng};
use serde_json::json;
use tracing::{error, info};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::models::sign_up::{
    ActiveModel, Column, Entity as UserEntity,
};
use crate::types::signup::{LoginPayload, SignUpPayload};

// ========== Custom Error Type ==========

#[derive(Debug)]
pub enum AuthError {
    PasswordHashError(bcrypt::BcryptError),
    DatabaseError(sea_orm::DbErr),
    DecryptionError(String),
    UserNotFound,
    InvalidCredentials,
    PasswordVerificationError(bcrypt::BcryptError),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::PasswordHashError(e) => {
                error!("Password hashing failed: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Error processing request")
            }
            AuthError::DatabaseError(e) => {
                error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Error processing request")
            }
            AuthError::DecryptionError(e) => {
                error!("Decryption failed: {}", e);
                (StatusCode::BAD_REQUEST, "Invalid tracking ID")
            }
            AuthError::UserNotFound => {
                (StatusCode::NOT_FOUND, "User not found")
            }
            AuthError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials")
            }
            AuthError::PasswordVerificationError(e) => {
                error!("Password verification failed: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Error processing request")
            }
        };

        (status, Json(json!({ "message": message }))).into_response()
    }
}

// ========== Route Registration ==========

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

// ========== Handlers ==========

/// Signup handler - creates a new user account
pub async fn signup_handler(
    State(db): State<Arc<DatabaseConnection>>,
    Json(payload): Json<SignUpPayload>,
) -> Result<Response, AuthError> {
    info!("Received signup request for email: {}", payload.email);

    // Hash the password
    let hashed_password = hash_password(&payload.password)
        .map_err(AuthError::PasswordHashError)?;

    // Generate tracking token
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    // Create new user model
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

    // Insert user into database
    let user = new_user.insert(db.as_ref())
        .await
        .map_err(AuthError::DatabaseError)?;

    info!("User created successfully with ID: {:?}", user.id);

    Ok((
        StatusCode::CREATED,
        Json(json!({ "message": "User created successfully" }))
    ).into_response())
}

/// Login handler - authenticates user and returns encrypted tracking ID
pub async fn login_handler(
    State(db): State<Arc<DatabaseConnection>>,
    Json(payload): Json<LoginPayload>,
) -> Result<Response, AuthError> {
    info!("Login attempt for email: {}", payload.email);

    // Find user by email
    let user = UserEntity::find()
        .filter(Column::Email.eq(&payload.email))
        .one(db.as_ref())
        .await
        .map_err(AuthError::DatabaseError)?
        .ok_or(AuthError::UserNotFound)?;

    // Verify password
    let is_valid = verify_password(&payload.password, &user.password_hash)
        .map_err(AuthError::PasswordVerificationError)?;

    if !is_valid {
        return Err(AuthError::InvalidCredentials);
    }

    // Encrypt tracking ID for response
    let tracking_str = user.tracking_id
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or_default();

    let encrypted_tracking_id = encrypt_deterministic(&tracking_str);

    info!("User logged in successfully: {}", payload.email);

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Login successful",
            "tracking_id": encrypted_tracking_id,
        }))
    ).into_response())
}

/// Fetch user handler - retrieves user data by encrypted tracking ID
#[derive(Deserialize)]
pub struct UserFetch {
    pub tracking_id: String,
}

pub async fn login_handler_fetch(
    State(db): State<Arc<DatabaseConnection>>,
    Json(data): Json<UserFetch>,
) -> Result<Response, AuthError> {
    info!("Fetching user with encrypted tracking_id");

    // Decrypt tracking ID
    let decrypted_tracking_id = decrypt_deterministic(&data.tracking_id)
        .map_err(AuthError::DecryptionError)?;

    // Find user by decrypted tracking ID
    let user = UserEntity::find()
        .filter(Column::TrackingId.eq(&decrypted_tracking_id))
        .one(db.as_ref())
        .await
        .map_err(AuthError::DatabaseError)?
        .ok_or(AuthError::UserNotFound)?;

    info!("User found: {}", user.email);

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "success",
            "email": user.email,
            "username": user.full_name,
            "role": user.role,
            "id": user.tracking_id
        }))
    ).into_response())
}

// ========== Helper Functions ==========

/// Hash a password using bcrypt
fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

/// Verify a password against a bcrypt hash
fn verify_password(password: &str, hashed_password: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hashed_password)
}