//! <img src="https://github.com/mollie.png" width="100" height="100" align="right">
//!
//! #[Mollie API](https://docs.mollie.com/index)
//!
//! This crate provides a Rust client for the Mollie API.

#![doc(html_logo_url = "https://github.com/mollie.png")]

use std::collections::HashMap;

use api::organizations;
use auth::AuthProvider;
use log::{debug, error};
use models::error_response::ErrorResponse;
use reqwest::{header::HeaderMap, Client};
use serde::{Deserialize, Serialize};

use crate::error::Error;

pub mod api;
pub mod auth;
pub mod error;
pub mod models;

/// Result type used throughout the crate
/// Errors are of type `mollie_api::errors::Error`
pub type Result<T> = std::result::Result<T, error::Error>;

/// Mollie API base url
const API_BASE_URL: &str = "https://api.mollie.com/v2";

lazy_static::lazy_static! {
    /// Default user agent used for all requests
    /// The user agent includes name, version and repository url
    static ref USER_AGENT: String = format!(
        "{}-rust/{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );
}

#[derive(Clone)]
pub struct ApiClient<'a> {
    /// Async client
    client: Client,

    /// Api base url
    base_url: &'a str,

    /// Auth token
    auth_provider: &'a dyn AuthProvider,
}

impl<'a> ApiClient<'a> {
    /// Create a new api (async) client instance.
    pub fn new(base_url: &'static str, auth_provider: &'a impl AuthProvider) -> Self {
        let client = Client::builder()
            .default_headers(ApiClient::default_headers())
            .build()
            .expect("Failed to build http client");

        Self {
            client,
            base_url,
            auth_provider,
        }
    }

    /// Map all the default headers
    fn default_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(&USER_AGENT),
        );
        headers
    }

    fn get_auth_token(&self) -> String {
        return self.auth_provider.get_auth_token().get_token().to_owned()
    }

    fn build_url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'))
    }

    /// Perform a post request using default headers and auth token
    pub async fn post<T, R>(&self, endpoint: &str, body: &T) -> Result<R>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        let url = self.build_url(endpoint);

        let response = self
            .client
            .post(url)
            .bearer_auth(self.get_auth_token())
            .json(&body)
            .send()
            .await?;

        self.parse_response(response).await
    }

    /// Perform a get request using default headers and auth token
    pub async fn get<R>(&self, endpoint: &str, query: Option<HashMap<&str, String>>) -> Result<R>
    where
        R: for<'de> Deserialize<'de>,
    {
        let url = self.build_url(endpoint);

        let mut req = self.client.get(url).bearer_auth(self.get_auth_token());

        if let Some(q) = query {
            req = req.query(&q);
        }

        let response = req.send().await?;
        self.parse_response(response).await
    }

    async fn parse_response<R>(&self, response: reqwest::Response) -> Result<R>
    where
        R: for<'de> Deserialize<'de>,
    {
        if response.status().is_success() {
            return response.json::<R>().await.map_err(|e| {
                error!("{:?}", e);
                // Remove the url from the error to avoid leaking sensitive information
                // that might be present in the url
                Error::CouldNotPerformRequest(e.without_url())
            });
        }

        let status = response.status();
        let raw_json = response.json::<ErrorResponse>().await?;

        Err(Error::ApiError {
            status: status.as_u16(),
            title: raw_json.title.to_string(),
            detail: raw_json.detail.to_string(),
            raw_response: raw_json.to_string(),
        })
    }
}

/// Mollie API client
#[derive(Clone)]
pub struct Mollie<'c> {
    api_client: ApiClient<'c>,
}

impl<'c> Mollie<'c> {
    /// Create a new Mollie instance
    pub fn build(auth_provider: &'c impl AuthProvider) -> Self {
        debug!("Creating new Mollie instance. Base url: {}", API_BASE_URL);
        Self {
            api_client: ApiClient::new(API_BASE_URL, auth_provider),
        }
    }

    /// Organizations API
    pub fn organizations(&self) -> organizations::OrganizationsApi {
        organizations::OrganizationsApi::new(&self.api_client)
    }
}
