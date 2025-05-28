use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct UserRegisterRequest {
    pub email: String,
    pub email_code: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct UserRegisterResponse {
    pub token: String,
}

#[derive(Serialize, Debug)]
pub struct EmailVerificationRequest {
    pub email: String,
}
