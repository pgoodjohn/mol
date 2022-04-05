use super::config;
use super::mollie_sdk;
use log::{debug, info};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

pub fn command(payment_id: &String) -> Result<(), &'static str> {
    debug!("Running Get API Payment for payment: {}", payment_id);

    get_payment_from_api(payment_id);

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct PaymentResponse {
    resource: String,
    id: String,
    mode: String,
    description: String,
    method: Option<String>,
    status: String,
}

fn get_payment_from_api(payment_id: &String) {
    debug!("Making HTTP Request");

    let mollie_client = mollie_sdk::ApiClient::new();
    let response = mollie_client
        .get(String::from("v2/payments"), Some(String::from(payment_id)))
        .unwrap();

    // Load API key from ~/.mol/conf.tom
    // HTTP 200 Response means the payment was found
    if response.status() == StatusCode::OK {
        debug!("Successfull call to the Mollie API!");
        let decoded_response = response.json::<PaymentResponse>().unwrap();
        debug!("{:?}", decoded_response);

        // TODO: do stuff with the payment

        // If payment is still open, show the checkout url
        if decoded_response.status == "open" {
            match decoded_response.method {
                Some(_) => info!("I still don't support going to the method URL directly, but the payment ID is: {}", decoded_response.id),
                None => info!("Pay this payment: {}/checkout/select-method/{}", config::api_url().unwrap(), decoded_response.id)
            }
        }

        return;
    }

    // Any other response is an error
    mollie_sdk::handle_mollie_api_error(response);
}
