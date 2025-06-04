use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method, StatusCode,
};
use serde::de::DeserializeOwned;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use types::{auth::*, feed::*};

mod error;
pub mod types;

pub use error::ApiClientError;
use error::Result;

use crate::api::error::ApiErrorResponse;

const USER_AGENT: &str = concat!("bind-app/", env!("CARGO_PKG_VERSION"));

#[derive(Clone)]
pub struct ApiClient {
    client: reqwest::Client,
    pub base_url: String,
    token: Arc<Mutex<Option<String>>>,
}

impl ApiClient {
    pub fn new(base_url: &'static str) -> Self {
        let base_url = base_url.to_string();

        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("User-Agent", HeaderValue::from_static(USER_AGENT));

        let client_builder = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .timeout(Duration::from_secs(30));

        Self {
            client: client_builder.build().unwrap(),
            base_url,
            token: Arc::new(Mutex::new(None)),
        }
    }

    pub fn get_token(&self) -> Option<String> {
        match self.token.get_cloned() {
            Ok(token) => token,
            Err(err) => {
                self.token.clear_poison();
                self.get_token()
            }
        }
    }

    pub fn set_token(&self, token: Option<String>) {
        match self.token.set(token.clone()) {
            Ok(_) => {}
            Err(err) => {
                self.token.clear_poison();
                self.set_token(token);
            }
        }
    }

    fn make_request(&self, method: Method, path: &str) -> reqwest::RequestBuilder {
        let mut request = self.client.request(
            method,
            format!("{}/{}", self.base_url, path.trim_start_matches("/")),
        );
        if let Some(token) = &self.get_token() {
            request = request.bearer_auth(token);
        }
        request
    }

    /// Handles a response from the API, returning the deserialized response if the status code matches.
    /// Otherwise, returns an error with the status code and message.
    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
        status_code: StatusCode,
    ) -> Result<T> {
        if response.status() == status_code {
            return Ok(response.json::<T>().await?);
        }

        let status = response.status().as_u16();
        let message = response
            .json::<ApiErrorResponse>()
            .await
            .map(|error| error.message)
            .unwrap_or_else(|_| format!("Unknown error from server ({})", status));
        Err(ApiClientError::ApiError { status, message })
    }

    /// Create a new feed subscription.
    ///
    /// Corresponds to `PUT /feed`.
    /// Requires authentication.
    ///
    /// # Arguments
    /// * `feed_data` - The data for the new feed, primarily its URL.
    pub async fn create_feed(&self, feed_data: &CreateFeedRequest) -> Result<Feed> {
        let response = self
            .make_request(Method::PUT, "/feed")
            .json(feed_data) // Serialize feed_data to JSON and set as body
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::CREATED)
            .await
    }

    /// Register a new user with email and password.
    ///
    /// Corresponds to `POST /user/email/register`.
    /// This endpoint does not require prior authentication.
    ///
    /// # Arguments
    /// * `registration_data` - The user's registration details.
    pub async fn register_user(
        &self,
        registration_data: &UserRegisterRequest,
    ) -> Result<UserRegisterResponse> {
        let response = self
            .make_request(Method::POST, "/user/email/register")
            .json(registration_data)
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    /// Sends an email to the user with a verification code
    /// which must be used during registration.
    ///
    /// Corresponds to `POST /user/email/verify`.
    /// This endpoint does not require prior authentication.
    pub async fn send_email_verification(&self, email: &str) -> Result<()> {
        let response = self
            .make_request(reqwest::Method::POST, "/user/email/verify")
            .json(&EmailVerificationRequest {
                email: email.to_string(),
            })
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    /// Login with email/username and password.
    ///
    /// Corresponds to `POST /user/email/login`.
    /// This endpoint does not require prior authentication.
    pub async fn login_user(
        &self,
        email_or_username: &str,
        password: &str,
    ) -> Result<UserLoginResponse> {
        let is_email = email_or_username.contains('@');

        let response = self
            .make_request(reqwest::Method::POST, "/user/email/login")
            .json(&UserLoginRequest {
                email: is_email.then_some(email_or_username.to_string()),
                username: (!is_email).then_some(email_or_username.to_string()),
                password: password.to_string(),
            })
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    /// Send reset password email code
    /// Corresponds to `POST /user/email/send-password-reset-code`.
    pub async fn send_password_reset_code(&self, email: &str) -> Result<()> {
        let response = self
            .make_request(Method::POST, "/user/email/send-password-reset-code")
            .json(&SendPasswordCodeRequest {
                email: email.to_string(),
            })
            .send()
            .await?;

        let status = response.status();

        if status == reqwest::StatusCode::OK {
            Ok(())
        } else {
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|e| format!("Could not retrieve error body: {}", e));
            Err(ApiClientError::ApiError {
                status_code: status.as_u16(),
                error_message,
            })
        }
    }

    /// Reset user's password
    /// Corresponds to `POST /user/email/reset-password`.
    pub async fn reset_password(&self, email: &str, code: &str, new_password: &str) -> Result<()> {
        let request = ResetPasswordRequest {
            email: email.to_string(),
            code: code.to_string(),
            new_password: new_password.to_string(),
        };
        let response = self
            .make_request(Method::POST, "/user/email/reset-password")
            .json(&request)
            .send()
            .await?;

        let status = response.status();

        if status == reqwest::StatusCode::OK {
            Ok(())
        } else {
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|e| format!("Could not retrieve error body: {}", e));
            Err(ApiClientError::ApiError {
                status_code: status.as_u16(),
                error_message,
            })
        }
    }
}
