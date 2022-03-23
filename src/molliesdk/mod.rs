use log::{debug, warn};
use reqwest::blocking::Response;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MollieApiError {
    status: i32,
    title: String,
    detail: String,
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
}
