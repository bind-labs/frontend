use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method, StatusCode,
};
use serde::de::DeserializeOwned;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::api::error::ApiErrorResponse;
use types::{feed::*, index::*, list::*, search::*, tag::*, user::*};

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

    // ---------------
    // Feed
    // ---------------

    /// `GET /feed/{id}`: Get a feed by ID
    pub async fn get_feed(&self, id: i32) -> Result<Feed> {
        let response = self
            .make_request(Method::GET, &format!("/feed/{}", id))
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    /// `PUT /feed`: Create a new feed subscription.
    pub async fn create_feed(&self, link: &str) -> Result<Feed> {
        let response = self
            .make_request(Method::PUT, "/feed")
            .json(&CreateFeedRequest {
                link: link.to_string(),
            })
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::CREATED)
            .await
    }

    /// `POST /feed/discover`: Discover feeds from a website URL
    pub async fn discover_feeds(&self, link: &str) -> Result<Vec<FeedInformation>> {
        let response = self
            .make_request(Method::POST, "/feed/discover")
            .json(&DiscoverFeedsRequest {
                link: link.to_string(),
            })
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    // ---------------
    // Index
    // ---------------

    /// `GET /index/{id}`: Get an index
    pub async fn get_index(&self, id: i32) -> Result<UserIndex> {
        let response = self
            .make_request(Method::GET, &format!("/index/{}", id))
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    /// `GET /index`: List all indexes
    pub async fn list_indexes(&self, page: usize) -> Result<Vec<UserIndex>> {
        let response = self
            .make_request(Method::GET, &format!("/index?page={}&limit=20", page))
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    /// `PUT /index`: Create a new index
    pub async fn create_index(&self, index_data: &CreateIndexRequest) -> Result<UserIndex> {
        let response = self
            .make_request(Method::PUT, "/index")
            .json(index_data)
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::CREATED)
            .await
    }

    /// `PATCH /index/{id}`: Update an index
    pub async fn update_index(
        &self,
        id: i32,
        index_data: &UpdateIndexRequest,
    ) -> Result<UserIndex> {
        let response = self
            .make_request(Method::PATCH, &format!("/index/{}", id))
            .json(index_data)
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    /// `DELETE /index/{id}`: Delete an index
    pub async fn delete_index(&self, id: i32) -> Result<()> {
        let response = self
            .make_request(Method::DELETE, &format!("/index/{}", id))
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::NO_CONTENT)
            .await
    }

    // ---------------
    // Item
    // ---------------

    /// `GET /item/{id}`: Get a feed item by ID
    pub async fn get_item(&self, id: i32) -> Result<FeedItem> {
        let response = self
            .make_request(Method::GET, &format!("/item/{}", id))
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    // ---------------
    // List
    // ---------------

    /// `GET /list/{id}`: Get a list by ID
    pub async fn get_list(&self, id: i32) -> Result<UserList> {
        let response = self
            .make_request(Method::GET, &format!("/list/{}", id))
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    /// `GET /list`: List all lists
    pub async fn list_lists(&self, page: usize) -> Result<Vec<UserList>> {
        let response = self
            .make_request(Method::GET, &format!("/list?page={}&limit=20", page))
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    /// `PUT /list`: Create a new list
    pub async fn create_list(&self, list_data: &CreateListRequest) -> Result<UserList> {
        let response = self
            .make_request(Method::PUT, "/list")
            .json(list_data)
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::CREATED)
            .await
    }

    /// `PATCH /list/{id}`: Update a list
    pub async fn update_list(&self, id: u32, list_data: &UpdateListRequest) -> Result<UserList> {
        let response = self
            .make_request(Method::PATCH, &format!("/list/{}", id))
            .json(list_data)
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    /// `DELETE /list/{id}`: Delete a list
    pub async fn delete_list(&self, id: i32) -> Result<()> {
        let response = self
            .make_request(Method::DELETE, &format!("/list/{}", id))
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::NO_CONTENT)
            .await
    }

    // ---------------
    // List/Item
    // ---------------

    /// `GET /list/{list_id}/item/{item_id}`: Get a list item by ID
    pub async fn get_list_item(&self, list_id: i32, item_id: i64) -> Result<UserListItem> {
        let response = self
            .make_request(Method::GET, &format!("/list/{}/item/{}", list_id, item_id))
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    /// `GET /list/{list_id}/item`: List all list items
    pub async fn list_list_items(&self, list_id: i32, page: usize) -> Result<Vec<UserListItem>> {
        let response = self
            .make_request(Method::GET, &format!("/list/item?page={}&limit=20", page))
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    /// `PUT /list/{list_id}/item`: Create a new list item
    pub async fn create_list_item(
        &self,
        list_id: i32,
        item_data: &CreateListItemRequest,
    ) -> Result<UserListItem> {
        let response = self
            .make_request(Method::PUT, &format!("/list/{}/item", list_id))
            .json(item_data)
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::CREATED)
            .await
    }

    /// `DELETE /list/{list_id}/item/{item_id}`: Delete a list item
    pub async fn delete_list_item(&self, list_id: i32, item_id: i64) -> Result<()> {
        let response = self
            .make_request(
                Method::DELETE,
                &format!("/list/{}/item/{}", list_id, item_id),
            )
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::NO_CONTENT)
            .await
    }

    // ---------------
    // Search
    // ---------------

    /// `POST /search`: Search for feeds
    pub async fn search(&self, search_data: &SearchRequest, page: usize) -> Result<Vec<Feed>> {
        let response = self
            .make_request(Method::POST, &format!("/search?page={}&limit=20", page))
            .json(search_data)
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    // ---------------
    // Tag
    // ---------------

    /// `GET /tag`: List all tags
    pub async fn list_tags(&self, page: usize) -> Result<Vec<UserTag>> {
        let response = self
            .make_request(Method::GET, &format!("/tag?page={}&limit=20", page))
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    /// `PUT /tag`: Create a new tag
    pub async fn create_tag(&self, tag_data: &CreateTagRequest) -> Result<UserTag> {
        let response = self
            .make_request(Method::PUT, "/tag")
            .json(tag_data)
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::CREATED)
            .await
    }

    /// `PATCH /tag/{id}`: Update a tag
    pub async fn update_tag(&self, id: i32, tag_data: &UpdateTagRequest) -> Result<UserTag> {
        let response = self
            .make_request(Method::PATCH, &format!("/tag/{}", id))
            .json(tag_data)
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    /// `DELETE /tag/{id}`: Delete a tag
    pub async fn delete_tag(&self, id: i32) -> Result<()> {
        let response = self
            .make_request(Method::DELETE, &format!("/tag/{}", id))
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::NO_CONTENT)
            .await
    }

    // ---------------
    // User/History
    // ---------------

    /// `GET /user/history/{id}`: Get a history item by ID
    pub async fn get_history(&self, id: i64) -> Result<HistoryItem> {
        let response = self
            .make_request(Method::GET, &format!("/user/history/{}", id))
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    /// `GET /user/history`: List all history items for the current user
    pub async fn list_history(&self, page: usize) -> Result<Vec<HistoryItem>> {
        let response = self
            .make_request(
                Method::GET,
                &format!("/user/history?page={}&limit=20", page),
            )
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    /// `PUT /user/history`: Create a new history item for the current user
    pub async fn create_history(&self, item_data: &CreateHistoryRequest) -> Result<HistoryItem> {
        let response = self
            .make_request(Method::PUT, "/user/history")
            .json(item_data)
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::CREATED)
            .await
    }

    /// `PATCH /user/history/{id}`: Update a history item
    pub async fn update_history(
        &self,
        id: i64,
        item_data: &UpdateHistoryRequest,
    ) -> Result<HistoryItem> {
        let response = self
            .make_request(Method::PATCH, &format!("/user/history/{}", id))
            .json(item_data)
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    /// `DELETE /user/history/{id}`: Delete a history item
    pub async fn delete_history(&self, id: i64) -> Result<()> {
        let response = self
            .make_request(Method::DELETE, &format!("/user/history/{}", id))
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::NO_CONTENT)
            .await
    }

    // ---------------
    // User/Email
    // ---------------

    /// `POST /user/email/register`: Register a new user with email and password.
    /// This endpoint does not require prior authentication.
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

    /// `POST /user/email/verify`: Sends an email to the user with a verification code
    /// which must be used during registration.
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

    /// `POST /user/email/login`: Login with email/username and password.
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

    /// `POST /user/email/send-password-reset-code`: Send reset password email code
    pub async fn send_password_reset_code(&self, email: &str) -> Result<()> {
        let response = self
            .make_request(Method::POST, "/user/email/send-password-reset-code")
            .json(&SendPasswordCodeRequest {
                email: email.to_string(),
            })
            .send()
            .await?;

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    /// `POST /user/email/reset-password`: Reset user's password
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

        self.handle_response(response, reqwest::StatusCode::OK)
            .await
    }

    // ---------------
    // User/OAuth
    // ---------------

    /// `POST /user/oauth/authorize`: Get redirect URL to OAuth provider for authorization
    pub async fn authorize_user(&self, provider: &str) -> Result<String> {
        let response = self
            .make_request(Method::POST, "/user/oauth/authorize")
            .json(&AuthorizeRequest {
                // TODO: use enum
                provider: provider.to_string(),
                client: OAuthRedirectClient::Android,
            })
            .send()
            .await?;

        if response.status() != reqwest::StatusCode::TEMPORARY_REDIRECT {
            return self
                .handle_response(response, reqwest::StatusCode::TEMPORARY_REDIRECT)
                .await;
        }

        let location = response
            .headers()
            .get("Location")
            .expect("No location header");
        let location = location.to_str().expect("Invalid location header");
        Ok(location.to_string())
    }
}
