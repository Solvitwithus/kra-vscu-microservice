use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use sea_orm::{
    ActiveModelTrait,
    ActiveValue::Set,
    DatabaseConnection,
    EntityTrait,
};

use serde_json::json;
use tracing::error;

use crate::{
    models::{
        branch_customers::{
            ActiveModel as CustomerActiveModel,
            Entity as CustomerEntity,
        },
        branch_users::{
            ActiveModel as UserActiveModel,
            Entity as UserEntity,
        },
        branch_insurances::{
            ActiveModel as InsuranceActiveModel,
            Entity as InsuranceEntity,
        },
    },
    types::braches_data_payload::{
        BhfCustSaveReq,
        BhfUserSaveReq,
        BhfInsuranceSaveReq,
    },
};

/// ──────────────────────────────────────────────
/// ROUTERS
/// ──────────────────────────────────────────────

pub fn branch_customers(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/", post(handle_customer_post).get(handle_customer_get))
        .with_state(db)
}

pub fn branch_users(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/", post(handle_user_post).get(handle_user_get))
        .with_state(db)
}

pub fn branch_insurances(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/", post(handle_insurance_post).get(handle_insurance_get))
        .with_state(db)
}

/// ──────────────────────────────────────────────
/// CUSTOMER HANDLERS
/// ──────────────────────────────────────────────

pub async fn handle_customer_post(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<BhfCustSaveReq>,
) -> impl IntoResponse {
    let model = CustomerActiveModel {
        tin: Set(payload.tin),
        bhf_id: Set(payload.bhfId),
        cust_no: Set(payload.custNo),
        cust_tin: Set(payload.custTin),
        cust_nm: Set(payload.custNm),
        adrs: Set(payload.adrs),
        tel_no: Set(payload.telNo),
        email: Set(payload.email),
        fax_no: Set(payload.faxNo),
        use_yn: Set(payload.useYn),
        remark: Set(payload.remark),
        regr_nm: Set(payload.regrNm),
        regr_id: Set(payload.regrId),
        modr_nm: Set(payload.modrNm),
        modr_id: Set(payload.modrId),
        ..Default::default()
    };

    match model.insert(&db).await {
        Ok(saved) => (
            StatusCode::CREATED,
            Json(json!({
                "status": "success",
                "message": "Customer created",
                "id": saved.id
            })),
        ),
        Err(e) => {
            error!("Failed to save customer: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to create customer",
                    "details": e.to_string()
                })),
            )
        }
    }
}

pub async fn handle_customer_get(
    State(db): State<DatabaseConnection>,
) -> impl IntoResponse {
    match CustomerEntity::find().all(&db).await {
        Ok(customers) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "data": customers
            })),
        ),
        Err(e) => {
            error!("Failed to fetch customers: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to fetch customers",
                    "details": e.to_string()
                })),
            )
        }
    }
}

/// ──────────────────────────────────────────────
/// USER HANDLERS
/// ──────────────────────────────────────────────

pub async fn handle_user_post(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<BhfUserSaveReq>,
) -> impl IntoResponse {
    let model = UserActiveModel {
        tin: Set(payload.tin),
        bhf_id: Set(payload.bhfId),
        user_id: Set(payload.userId),
        user_nm: Set(payload.userNm),
        pwd: Set(payload.pwd),
        adrs: Set(payload.adrs),
        cntc: Set(payload.cntc),
        auth_cd: Set(payload.authCd),
        remark: Set(payload.remark),
        use_yn: Set(payload.useYn),
        regr_nm: Set(payload.regrNm),
        regr_id: Set(payload.regrId),
        modr_nm: Set(payload.modrNm),
        modr_id: Set(payload.modrId),
        ..Default::default()
    };

    match model.insert(&db).await {
        Ok(saved) => (
            StatusCode::CREATED,
            Json(json!({
                "status": "success",
                "message": "User created",
                "id": saved.id   // change to saved.user_id if that's your PK
            })),
        ),
        Err(e) => {
            error!("Failed to insert user: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to create user",
                    "details": e.to_string()
                })),
            )
        }
    }
}

pub async fn handle_user_get(
    State(db): State<DatabaseConnection>,
) -> impl IntoResponse {
    match UserEntity::find().all(&db).await {
        Ok(users) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "data": users
            })),
        ),
        Err(e) => {
            error!("Failed to fetch users: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to fetch users",
                    "details": e.to_string()
                })),
            )
        }
    }
}

/// ──────────────────────────────────────────────
/// INSURANCE HANDLERS
/// ──────────────────────────────────────────────

pub async fn handle_insurance_post(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<BhfInsuranceSaveReq>,
) -> impl IntoResponse {
    let model = InsuranceActiveModel {
        tin: Set(payload.tin),
        bhf_id: Set(payload.bhfId),
        isrcc_cd: Set(payload.isrccCd),
        isrcc_nm: Set(payload.isrccNm),
        isrc_rt: Set(payload.isrcRt),
        use_yn: Set(payload.useYn),
        regr_nm: Set(payload.regrNm),
        regr_id: Set(payload.regrId),
        modr_nm: Set(payload.modrNm),
        modr_id: Set(payload.modrId),
        ..Default::default()
    };

    match model.insert(&db).await {
        Ok(saved) => (
            StatusCode::CREATED,
            Json(json!({
                "status": "success",
                "message": "Insurance created",
                "id": saved.id
            })),
        ),
        Err(e) => {
            error!("Failed to save insurance: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to create insurance",
                    "details": e.to_string()
                })),
            )
        }
    }
}

pub async fn handle_insurance_get(
    State(db): State<DatabaseConnection>,
) -> impl IntoResponse {
    match InsuranceEntity::find().all(&db).await {
        Ok(insurances) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "data": insurances
            })),
        ),
        Err(e) => {
            error!("Failed to fetch insurances: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to fetch insurances",
                    "details": e.to_string()
                })),
            )
        }
    }
}