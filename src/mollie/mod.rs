use super::config;
use log::debug;
use reqwest::blocking::Client;
use serde::ser;
use serde::{Deserialize, Serialize};

pub mod errors;
pub mod organizations;
pub mod payments;
pub mod permissions;

pub struct ApiClient {
    base_url: String,
    auth_token: ApiBearerToken,
    client: Client,
}

#[derive(Deserialize, Debug)]
pub struct Link {
    #[allow(dead_code)]
    r#type: String,
    pub href: String,
}

impl ApiClient {
    pub fn new() -> ApiClient {
        ApiClient {
            base_url: config::api_url().unwrap(),
            auth_token: get_bearer_token_from_config().unwrap(),
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn post<T: ser::Serialize>(
        &self,
        request: T,
        url: String,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let response = self
            .client
            .post(format!("{}/{}", &self.base_url, url))
            .bearer_auth(&self.auth_token.value)
            .header(
                reqwest::header::USER_AGENT,
                format!(
                    "{} {} / {}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION"),
                    env!("CARGO_PKG_REPOSITORY")
                ),
            )
            .json(&request)
            .send()?;

        Ok(response)
    }

    pub fn get(
        &self,
        url: String,
        parameter: Option<String>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let full_url: String;

        match parameter {
            Some(p) => full_url = format!("{}/{}/{}", &self.base_url, url, p),
            None => full_url = format!("{}/{}", &self.base_url, url),
        }

        let response = self
            .client
            .get(full_url)
            .bearer_auth(&self.auth_token.value)
            .header(
                reqwest::header::USER_AGENT,
                format!(
                    "{} {} / {}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION"),
                    env!("CARGO_PKG_REPOSITORY")
                ),
            )
            .send()?;

        Ok(response)
    }
}

impl organizations::OrganizationsApi for ApiClient {
    fn get(
        &self,
        url: String,
        parameter: Option<String>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.get(url, parameter)
    }
    fn get_authentication_method(&self) -> ApiBearerToken {
        get_bearer_token_from_config().unwrap()
    }
}

#[derive(Debug)]
pub struct ApiBearerToken {
    value: String,
    token_type: ApiTokenTypes,
}

#[derive(Debug, PartialEq)]
enum ApiTokenTypes {
    ApiKey,
    AccessCode,
}

fn get_bearer_token_from_config() -> Result<ApiBearerToken, Box<dyn std::error::Error>> {
    match config::access_code() {
        Ok(access_code) => {
            return Ok(ApiBearerToken {
                value: access_code.to_string(),
                token_type: ApiTokenTypes::AccessCode,
            });
        }
        Err(_) => {
            debug!("No access code set, trying to see if an API key is set instead")
        }
    }

    match config::api_key() {
        Ok(live_api_key) => {
            return Ok(ApiBearerToken {
                value: live_api_key.to_string(),
                token_type: ApiTokenTypes::ApiKey,
            });
        }
        Err(_) => {
            // TODO: Handle this error better - probably check it also before doing all the prompts
            panic!("No auth set!!!")
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MollieApiError {
    pub status: i32,
    pub title: String,
    pub detail: String,
}
