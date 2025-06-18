use serde::{Deserialize, Serialize};

// History

/// Represents a single item in a user's history
#[derive(Clone, Debug, Deserialize)]
pub struct HistoryItem {
    pub id: i64,
    /// ID of the user this history item belongs to
    pub owner: i32,
    /// ID of the feed item this item is referencing
    pub item: i64,
    /// Progress in the item, 0 - 1
    pub progress: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct CreateHistoryRequest {
    /// ID of the feed item this item is referencing
    pub item: i64,
    /// Progress in the item, 0 - 1
    pub progress: f64,
}

#[derive(Debug, Serialize)]
pub struct UpdateHistoryRequest {
    /// Progress in the item, 0 - 1
    pub progress: f64,
}

// Email

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
    pub new_password: String,
}

// OAuth

#[derive(Serialize)]
pub struct AuthorizeRequest {
    pub provider: String,
    pub client: OAuthRedirectClient,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OAuthRedirectClient {
    Web,
    Android,
    IOS,
}

// ----------

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
