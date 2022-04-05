use super::mollie_sdk;
use serde::Deserialize;
use log::{debug, info};
use reqwest::StatusCode;

pub fn command() {
    debug!("Listing 10 Payments");

    let client = mollie_sdk::ApiClient::new();

    let response = client.get(String::from("v2/payments"), None).unwrap();

       // HTTP 200 Response means the request was successful
    if response.status() == StatusCode::OK {
        debug!("Successfull call to the Mollie API!");
        let decoded_response = response
            .json::<ListPaymentsResponse>()
            .unwrap();
        debug!("{:?}", decoded_response);

        info!("Found {} payments", decoded_response.count);

        for payment in decoded_response.embedded.payments {
            info!(
                "{} - {}",
                payment.id, payment.status
            )
        }

        return;
    }

    // Any other response is an error
    mollie_sdk::handle_mollie_api_error(response);

}

#[derive(Debug, Deserialize)]
struct ListPaymentsResponse {
    count: i32,
    #[serde(rename(deserialize = "_embedded"))]
    embedded: PaymentResources
}

#[derive(Debug, Deserialize)]
struct PaymentResources {
    payments: Vec<PaymentResource>
}

#[derive(Debug, Deserialize)]
struct PaymentResource {
    id: String,
    status: String
}