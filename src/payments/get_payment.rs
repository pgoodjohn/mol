use super::config;
use super::mollie_sdk;
use log::{debug, info};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fs;

pub fn command(payment_id: &String) -> Result<(), &'static str> {
    debug!("Running Get API Payment for payment: {}", payment_id);

    execute_get_payment_request(payment_id);

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

fn execute_get_payment_request(payment_id: &String) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Making HTTP Request");

    // Load API key from ~/.mol/conf.toml
    let api_key = config::api_key().unwrap();

    // TODO: Enable usage with production
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!("{}/v2/payments/{}", config::api_url().unwrap(), payment_id))
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

        return Ok(());
    }

    // Any other response is an error
    mollie_sdk::handle_mollie_api_error(response);

    // TODO: Return CLI error
    Ok(())
}
