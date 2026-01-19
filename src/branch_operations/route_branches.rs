use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use reqwest::StatusCode;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use serde_json::json;
use crate::{models::{branch_customers::{ActiveModel, Entity}, branch_users::ActiveModel}, types::braches_data_payload::{BhfCustSaveReq, BhfInsuranceSaveReq, BhfUserSaveReq}};
use tracing::{info, error, warn};
use sea_orm::EntityTrait;

pub fn branch_customers(db:DatabaseConnection)-> Router {
    Router::new().route("/",post(handle_customer_post).get(handle_customer_get)).with_state(db)
}

pub fn branch_users(db:DatabaseConnection) -> Router {
    Router::new().route("/",post(handle_user_post).get(handle_user_get)).with_state(db)
}

pub fn branch_insurances(db:DatabaseConnection) -> Router {
    Router::new().route("/",post(handle_insurance_post).get(handle_insurance_get)).with_state(db)
}


pub async fn handle_customer_post(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<BhfCustSaveReq>, // ✅ API payload
) -> impl IntoResponse {
    let processed = ActiveModel {
        tin: Set(payload.tin),
        bhf_id: Set(payload.bhfId),        // map camelCase -> snake_case
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

    match processed.insert(&db).await {
        Ok(v) => (axum::http::StatusCode::OK, format!("Customer saved with id {}", v.id)),
        Err(e) => {
            tracing::error!("Failed to save customer: {:?}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to save customer".to_string())
        }
    }
}


pub async fn handle_customer_get(
    State(db): State<DatabaseConnection>,
) -> impl IntoResponse {
    // Handle customer GET request
    match Entity::find().all(&db).await {
        Ok(customers) => (axum::http::StatusCode::OK, Json(customers)),
        Err(e) => {
            tracing::error!("Failed to fetch customers: {:?}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

pub async fn handle_user_post(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<BhfUserSaveReq>,
) -> impl IntoResponse {
    // Handle user POST request

    let users = ActiveModel{
        tin:Set(payload.tin),
        bhf_id:Set(payload.bhfId),
        user_id:Set(payload.userId),
        user_nm:Set(payload.userNm),
        pwd:Set(payload.pwd),
        adrs:Set(payload.adrs),
        cntc:Set(payload.cntc),
        auth_cd:Set(payload.authCd),
        remark:Set(payload.remark),
        use_yn:Set(payload.useYn),
        regr_nm:Set(payload.regrNm),
        regr_id:Set(payload.regrId),
        modr_nm:Set(payload.modrNm),
        modr_id:Set(payload.modrId),
        ..Default::default()
    };

    match users.insert(&db).await {
        Ok(u)=> (StatusCode::CREATED, Json(json!(u))),
         Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to insert user",
                "details": err.to_string()
            })),
        ),
    }
}

pub async fn handle_user_get(
    State(db): State<DatabaseConnection>,
) -> impl IntoResponse {
    // Handle user GET request
    match Entity::find().all(&db).await {
        Ok(users) => (axum::http::StatusCode::OK, Json(users)),
        Err(e) => {
            tracing::error!("Failed to fetch users: {:?}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

pub async fn handle_insurance_post(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<BhfInsuranceSaveReq>,
) -> impl IntoResponse {

    let processed = ActiveModel {
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

match processed.insert(&db).await {
    Ok(v) => (axum::http::StatusCode::OK, format!("Insurance saved with id {}", v.id)),
    Err(e) => {
        tracing::error!("Failed to save insurance: {:?}", e);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to save insurance".to_string())
    }

}

}   

pub async fn handle_insurance_get(
    State(db): State<DatabaseConnection>,
) -> impl IntoResponse {
    // Handle insurance GET request
    
}   