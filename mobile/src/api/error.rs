use serde::Deserialize;
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
    #[error("API error: (Status: {status}): {message}")]
    ApiError { status: u16, message: String },
}

impl ApiClientError {
    pub fn message(&self) -> String {
        match self {
            ApiClientError::ReqwestError(err) => err.to_string(),
            ApiClientError::SerdeError(err) => err.to_string(),
            ApiClientError::AuthError(err) => err.to_string(),
            ApiClientError::ApiError { message, .. } => message.to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub message: String,
}
