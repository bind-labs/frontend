use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method,
};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use types::{auth::*, feed::*};

mod error;
pub mod types;

pub use error::ApiClientError;
use error::Result;

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
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("User-Agent", HeaderValue::from_static(USER_AGENT));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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

        if response.status() == reqwest::StatusCode::CREATED {
            let created_feed = response.json::<Feed>().await?;
            Ok(created_feed)
        } else {
            // Handle API errors (4xx, 5xx)
            let status = response.status();
            // Attempt to get error message from response body
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|_| "Could not retrieve error body".to_string());
            Err(ApiClientError::ApiError {
                status_code: status.as_u16(),
                error_message,
            })
        }
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

        let status = response.status();

        if status == reqwest::StatusCode::OK {
            // 200 OK for successful registration
            let register_response = response.json::<UserRegisterResponse>().await?;
            Ok(register_response)
        } else {
            // Handle API errors (400, 403, 409, 500)
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

    /// Sends an email to the user with a verification code
    /// which must be used during registration.
    ///
    /// Corresponds to `POST /user/email/verify`.
    /// This endpoint does not require prior authentication.
    pub async fn send_email_verification(&self, email: &str) -> Result<()> {
        // Returns Ok(()) on success
        let response = self
            .make_request(reqwest::Method::POST, "/user/email/verify")
            .json(&EmailVerificationRequest {
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

        let status = response.status();

        if status == reqwest::StatusCode::OK {
            // 200 OK for successful login
            let login_response = response.json::<UserLoginResponse>().await?;
            Ok(login_response)
        } else {
            // Handle API errors (400, 401, 500 from the spec)
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
