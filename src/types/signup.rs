use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpPayload {
    pub fullName: String,
    pub email: String,
    pub phoneNumber: String,
    pub password: String,
    pub confirmPassword: String,
    pub agreement: bool,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}