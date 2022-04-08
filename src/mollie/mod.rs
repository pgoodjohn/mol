use super::config;
use log::{debug, warn};
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::StatusCode;
use serde::ser;
use serde::{Deserialize, Serialize};

pub mod errors;
pub mod organizations;
pub mod payments;

pub struct ApiClient {
    base_url: String,
    auth_token: ApiBearerToken,
    client: Client,
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

impl payments::PaymentsApi for ApiClient {
    fn get(
        &self,
        url: String,
        parameter: Option<String>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.get(url, parameter)
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

pub fn handle_mollie_api_error(response: Response) {
    if response.status() == StatusCode::UNAUTHORIZED {
        warn!("Request to the Mollie API failed, could not authorize your request");
        warn!("Please check that your API key is configured correctly");
        warn!("Run mol auth add to add a new API key");
        return;
    }

    if response.status() == StatusCode::UNPROCESSABLE_ENTITY {
        let decoded_response = response.json::<MollieApiError>().unwrap();
        warn!("Request to the Mollie API failed, something was wrong with the request!");
        warn!(
            "The Mollie API returned the following error: {}",
            decoded_response.detail
        );
        return;
    }

    if response.status().as_u16() >= 500 {
        warn!("Request to the Mollie API Failed: the Mollie API appears to be unreachable.");
        debug!("Response: {:?}", response);
        return;
    }

    warn!("Request to the Mollie API failed in a spectacular and unexpected way.");
    warn!("Response status code was: {}", response.status());
    warn!("Run the command with the -d flag to get more details on the response");
    debug!("Response: {:?}", response);
}
