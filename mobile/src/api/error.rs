use thiserror::Error;

pub type Result<T> = std::result::Result<T, ApiClientError>;

#[derive(Error, Debug)]
pub enum ApiClientError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Auth error: {0}")]
    AuthError(String),
    #[error("API error: (Status: {status_code}): {error_message}")]
    ApiError {
        status_code: u16,
        error_message: String,
    },
}

impl ApiClientError {
    pub fn message(&self) -> String {
        match self {
            ApiClientError::ReqwestError(err) => err.to_string(),
            ApiClientError::SerdeError(err) => err.to_string(),
            ApiClientError::AuthError(err) => err.to_string(),
            ApiClientError::ApiError { error_message, .. } => error_message.to_string(),
        }
    }
}
