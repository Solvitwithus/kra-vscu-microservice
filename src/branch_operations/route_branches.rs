use axum::{Router, extract::State, response::IntoResponse};
use sea_orm::DatabaseConnection;
use serde_json::Value;

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
    Json(payload): Json<YourCustomerPayloadType>,
) -> impl IntoResponse {
    // Handle customer POST request
}

pub async fn handle_customer_get(
    State(db): State<DatabaseConnection>,
) -> impl IntoResponse {
    // Handle customer GET request
}

pub async fn handle_user_post(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<YourUserPayloadType>,
) -> impl IntoResponse {
    // Handle user POST request
}

pub async fn handle_user_get(
    State(db): State<DatabaseConnection>,
) -> impl IntoResponse {
    // Handle user GET request
}

pub async fn handle_insurance_post(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<YourInsurancePayloadType>,
) -> impl IntoResponse {
    // Handle insurance POST request
}   

pub async fn handle_insurance_get(
    State(db): State<DatabaseConnection>,
) -> impl IntoResponse {
    // Handle insurance GET request
}   