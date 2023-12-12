use mollie_api::auth::ApiBearerToken;
use reqwest::blocking::Client;
use serde::ser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod errors;
pub mod organizations;
pub mod payments;
pub mod refunds;

#[derive(Deserialize, Debug)]
pub struct Link {
    #[allow(dead_code)]
    r#type: String,
    pub href: String,
}

pub struct ApiClientBuilder {
    base_url: Option<String>,
    auth_token: Option<ApiBearerToken>,
    client: Option<Client>,
}

impl ApiClientBuilder {
    pub fn new() -> ApiClientBuilder {
        ApiClientBuilder {
            base_url: None,
            auth_token: None,
            client: None,
        }
    }

    pub fn auth(mut self, token: ApiBearerToken) -> ApiClientBuilder {
        self.auth_token = Some(token);
        self
    }

    pub fn url(mut self, url: String) -> ApiClientBuilder {
        self.base_url = Some(url);
        self
    }

    pub fn blocking(mut self) -> ApiClientBuilder {
        self.client = Some(Client::new());
        self
    }

    pub fn spawn(self) -> ApiClient {
        ApiClient {
            base_url: self.base_url.expect("Must have a base URL set."),
            auth_token: self.auth_token.expect("Must have an Auth Token set."),
            client: self.client.expect("Must have a Client set."),
        }
    }
}

pub struct ApiClient {
    base_url: String,
    auth_token: ApiBearerToken,
    client: Client,
}

impl ApiClient {
    pub fn post<T: ser::Serialize>(
        &self,
        request: T,
        url: String,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let response = self
            .client
            .post(format!("{}/{}", &self.base_url, url))
            .bearer_auth(&self.auth_token.as_str())
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
        query: Option<HashMap<&str, String>>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let full_url: String = match parameter {
            Some(p) => format!("{}/{}/{}", &self.base_url, url, p),
            None => format!("{}/{}", &self.base_url, url),
        };

        let mut request = self
            .client
            .get(full_url)
            .bearer_auth(&self.auth_token.as_str())
            .header(
                reqwest::header::USER_AGENT,
                format!(
                    "{} {} / {}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION"),
                    env!("CARGO_PKG_REPOSITORY")
                ),
            );

        if let Some(query) = query {
            request = request.query(&query);
        }

        let response = request.send()?;

        Ok(response)
    }
}

impl organizations::OrganizationsApi for ApiClient {
    fn get(
        &self,
        url: String,
        parameter: Option<String>,
        query: Option<HashMap<&str, String>>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.get(url, parameter, query)
    }
    fn get_authentication_method(&self) -> ApiBearerToken {
        self.auth_token.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MollieApiError {
    pub status: i32,
    pub title: String,
    pub detail: String,
}

#[cfg(test)]
mod client_builder_tests {
    use mollie_api::auth::{AccessCode, ApiBearerToken};

    #[test]
    fn it_spawns_a_new_client() {
        let client = super::ApiClientBuilder::new()
            .auth(ApiBearerToken::AccessCode(AccessCode {
                value: String::from("Test"),
            }))
            .url(String::from("https://api.mollie.dev/"))
            .blocking()
            .spawn();

        assert_eq!("https://api.mollie.dev/", client.base_url);
        assert_eq!("Test", client.auth_token.as_str());
    }

    #[test]
    #[should_panic(expected = "Must have a base URL set.")]
    fn it_does_not_spawn_a_client_without_base_url() {
        super::ApiClientBuilder::new()
            .auth(ApiBearerToken::AccessCode(AccessCode {
                value: String::from("Test"),
            }))
            .blocking()
            .spawn();
    }

    #[test]
    #[should_panic(expected = "Must have an Auth Token set.")]
    fn it_does_not_spawn_a_client_without_authorization() {
        super::ApiClientBuilder::new()
            .url(String::from("https://api.mollie.dev/"))
            .blocking()
            .spawn();
    }

    #[test]
    #[should_panic(expected = "Must have a Client set.")]
    fn it_does_not_spawn_a_client_without_http_client() {
        super::ApiClientBuilder::new()
            .auth(ApiBearerToken::AccessCode(AccessCode {
                value: String::from("Test"),
            }))
            .url(String::from("https://api.mollie.dev/"))
            .spawn();
    }
}
