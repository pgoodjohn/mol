use super::mollie;
use log::{debug, info, warn};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::error;
use std::fmt;

pub fn command() {
    match get_current_organization_from_api() {
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
    SomethingWentWrongWithTheRequest(reqwest::Error),
    SomethingIsWrongWithTheResponse(reqwest::Error),
    SomethingWentWrongFetchingOrganizationDetails(),
}

impl fmt::Display for CouldNotRetrieveOrganizationInformation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CouldNotRetrieveOrganizationInformation::SomethingWentWrongWithTheRequest(ref err) => write!(f, "Mollie API request failed 😢 - {}", err),
            CouldNotRetrieveOrganizationInformation::SomethingIsWrongWithTheResponse(ref err) => write!(f, "Could not parse the API response: {}", err),
            CouldNotRetrieveOrganizationInformation::SomethingWentWrongFetchingOrganizationDetails() => write!(f, "Getting your details failed 😢"),
        }
    }
}

impl error::Error for CouldNotRetrieveOrganizationInformation {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            CouldNotRetrieveOrganizationInformation::SomethingWentWrongWithTheRequest(ref err) => Some(err),
            CouldNotRetrieveOrganizationInformation::SomethingIsWrongWithTheResponse(ref err) => Some(err),
            CouldNotRetrieveOrganizationInformation::SomethingWentWrongFetchingOrganizationDetails() => None,
        }
    }
}

fn get_current_organization_from_api() -> Result<OrganizationsResponse, Box<dyn std::error::Error>>
{
    let client = mollie::ApiClient::new();
    let response = client
        .get(String::from("v2/organizations"), Some(String::from("me")))
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
    mollie::handle_mollie_api_error(response);

    return Err(
        CouldNotRetrieveOrganizationInformation::SomethingWentWrongFetchingOrganizationDetails()
            .into(),
    );
}
