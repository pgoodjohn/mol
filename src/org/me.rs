use super::config;
use super::mollie_sdk;
use log::{debug, info, warn};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::error;
use std::fmt;

pub fn command() {
    match execute_get_me_organization_request() {
        Ok(response) => {
            info!(
                "You are currently authenticated as Organization {} - {}",
                response.id, response.name
            )
        }
        Err(e) => {
            warn!(
                "Could not retrieve you organization details from the API, got: {}",
                e
            );
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct OrganizationsResponse {
    id: String,
    name: String,
    email: String,
}

#[derive(Debug)]
enum CouldNotRetrieveOrganizationInformation {
    UnableToLoadConfig(config::CouldNotRetrieveConfig),
    AccessCodeIsNotSet(config::CouldNotRetrieveConfig),
    SomethingWentWrongWithTheRequest(reqwest::Error),
    SomethingIsWrongWithTheResponse(reqwest::Error),
    SomethingWentWrongFetchingOrganizationDetails(),
}

impl fmt::Display for CouldNotRetrieveOrganizationInformation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CouldNotRetrieveOrganizationInformation::UnableToLoadConfig(ref err) => write!(f, "Error loading configuration: {}", err),
            CouldNotRetrieveOrganizationInformation::AccessCodeIsNotSet(ref err) => write!(f, "{}", err),
            CouldNotRetrieveOrganizationInformation::SomethingWentWrongWithTheRequest(ref err) => write!(f, "Mollie API request failed 😢 - {}", err),
            CouldNotRetrieveOrganizationInformation::SomethingIsWrongWithTheResponse(ref err) => write!(f, "Could not parse the API response: {}", err),
            CouldNotRetrieveOrganizationInformation::SomethingWentWrongFetchingOrganizationDetails() => write!(f, "Getting your details failed 😢"),
        }
    }
}

impl error::Error for CouldNotRetrieveOrganizationInformation {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            CouldNotRetrieveOrganizationInformation::UnableToLoadConfig(ref err) => Some(err),
            CouldNotRetrieveOrganizationInformation::AccessCodeIsNotSet(ref err) => Some(err),
            CouldNotRetrieveOrganizationInformation::SomethingWentWrongWithTheRequest(ref err) => Some(err),
            CouldNotRetrieveOrganizationInformation::SomethingIsWrongWithTheResponse(ref err) => Some(err),
            CouldNotRetrieveOrganizationInformation::SomethingWentWrongFetchingOrganizationDetails() => None,
        }
    }
}

fn execute_get_me_organization_request() -> Result<OrganizationsResponse, Box<dyn std::error::Error>>
{
    let api_key = config::access_code()
        .map_err(CouldNotRetrieveOrganizationInformation::AccessCodeIsNotSet)?;
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!(
            "{}/v2/organizations/me",
            config::api_url()
                .map_err(CouldNotRetrieveOrganizationInformation::UnableToLoadConfig)?
        ))
        .bearer_auth(api_key)
        .header(
            reqwest::header::USER_AGENT,
            format!(
                "{} {} / {}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_REPOSITORY")
            ),
        )
        .send()
        .map_err(CouldNotRetrieveOrganizationInformation::SomethingWentWrongWithTheRequest)?;

    // HTTP 200 Response means the request was successful
    if response.status() == StatusCode::OK {
        debug!("Successfull call to the Mollie API!");
        let decoded_response = response
            .json::<OrganizationsResponse>()
            .map_err(CouldNotRetrieveOrganizationInformation::SomethingIsWrongWithTheResponse)?;
        debug!("{:?}", decoded_response);

        return Ok(decoded_response);
    }

    // Any other response is an error
    mollie_sdk::handle_mollie_api_error(response);

    return Err(
        CouldNotRetrieveOrganizationInformation::SomethingWentWrongFetchingOrganizationDetails()
            .into(),
    );
}
