use super::config;
use super::molliesdk;
use log::{debug, warn};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

pub fn command() -> Result<(), &'static str> {
    match execute_get_me_organization_request() {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            warn!("Could not retrieve you organization details from the API, got: {}", e);
            Err("ops")
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct OrganizationsResponse {
    id: String,
    name: String,
    email: String,
}

fn execute_get_me_organization_request() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = config::access_code()?;
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.mollie.dev/v2/organizations/me")
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
        .send()?;

    // HTTP 200 Response means the request was successful
    if response.status() == StatusCode::OK {
        debug!("Successfull call to the Mollie API!");
        let decoded_response = response.json::<OrganizationsResponse>().unwrap();
        debug!("{:?}", decoded_response);

        return Ok(());
    }

    // Any other response is an error
    molliesdk::handle_mollie_api_error(response);

    // TODO: Return CLI error
    Ok(())
}
