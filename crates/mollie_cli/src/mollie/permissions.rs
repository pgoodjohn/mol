use log::debug;
use mollie_api::auth::ApiBearerToken;
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct PermissionsResponse {
    #[serde(rename(deserialize = "_embedded"))]
    pub embedded: PermissionsResources,
}

#[derive(Deserialize, Debug)]
pub struct PermissionsResources {
    pub permissions: Vec<PermissionResource>,
}

#[derive(Deserialize, Debug)]
pub struct PermissionResource {
    pub id: String,
    pub description: String,
    pub granted: bool,
}

pub trait Permissions {
    fn get(
        &self,
        url: String,
        parameter: Option<String>,
        query: Option<HashMap<&str, String>>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error>;

    fn get_authentication_method(&self) -> ApiBearerToken;

    fn get_permissions(&self) -> Result<PermissionsResponse, super::errors::ApiClientError> {
        let auth_token = self.get_authentication_method();
        debug!("{:?}", &auth_token);
        if !auth_token.is_access_code() {
            return Err(
                super::errors::ApiClientError::CouldNotFindValidAuthorizationMethodToPerformRequest(
                ),
            );
        }

        let response = self
            .get(String::from("v2/permissions"), None, None)
            .map_err(super::errors::ApiClientError::CouldNotPerformRequest)?;

        if response.status() == StatusCode::OK {
            let decoded_response = response
                .json::<PermissionsResponse>()
                .map_err(super::errors::ApiClientError::CouldNotUnderstandResponse)?;
            debug!("{:?}", decoded_response);

            return Ok(decoded_response);
        }

        let decoded_error_response = response
            .json::<super::MollieApiError>()
            .map_err(super::errors::ApiClientError::CouldNotUnderstandResponse)?;

        Err(super::errors::ApiClientError::MollieApiReturnedAnError(
            decoded_error_response,
        ))
    }
}

impl Permissions for super::ApiClient {
    fn get(
        &self,
        url: String,
        parameter: Option<String>,
        query: Option<HashMap<&str, String>>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.get(url, parameter, query)
    }
    fn get_authentication_method(&self) -> super::ApiBearerToken {
        self.auth_token.clone()
    }
}
