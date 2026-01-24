
use axum::{Json};
use axum_extra::extract::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use serde_json::Value;


pub async fn receive_payload(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<Value>,
) {
    let token = auth.token();

    println!("Bearer token: {}", token);
    println!("Payload: {:?}", payload);

}