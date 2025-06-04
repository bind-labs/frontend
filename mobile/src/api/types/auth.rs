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

#[derive(Serialize, Debug)]
pub struct SendPasswordCodeRequest {
    pub email: String,
}

#[derive(Serialize, Debug)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub code: String,
    pub new_password: String
}


#[derive(Serialize, Debug)]
pub struct UserLoginRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct UserLoginResponse {
    pub token: String,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AuthUser {
    /// Expiration time as UTC timestamp (required for JWT validation)
    pub exp: i64,
    /// Issued at time as UTC timestamp
    pub iat: i64,
    /// User's unique ID
    pub id: i32,
    /// User's email address
    pub email: String,
    /// User's username
    pub username: String,
}
