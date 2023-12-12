use log::debug;
use mollie_api::auth::ApiBearerToken;
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct OrganizationResource {
    pub id: String,
    pub name: String,
    pub locale: String,
    #[serde(rename(deserialize = "registrationNumber"))]
    pub registration_number: String,
    #[serde(rename(deserialize = "vatNumber"))]
    pub vat_number: Option<String>,
    #[serde(rename(deserialize = "vatRegulation"))]
    pub vat_regulation: Option<String>,
}

pub trait OrganizationsApi {
    fn get(
        &self,
        url: String,
        parameter: Option<String>,
        query: Option<HashMap<&str, String>>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error>;

    fn get_authentication_method(&self) -> super::ApiBearerToken;

    fn get_current_organization(
        &self,
    ) -> Result<OrganizationResource, super::errors::ApiClientError> {
        let auth_token = self.get_authentication_method();
        debug!("{:?}", &auth_token);
        if !matches!(auth_token, ApiBearerToken::AccessCode(..)) {
            return Err(
                super::errors::ApiClientError::CouldNotFindValidAuthorizationMethodToPerformRequest(
                ),
            );
        }

        let response = self
            .get(String::from("v2/organizations/me"), None, None)
            .map_err(super::errors::ApiClientError::CouldNotPerformRequest)?;

        if response.status() == StatusCode::OK {
            let decoded_response = response
                .json::<OrganizationResource>()
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
