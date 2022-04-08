use super::mollie;
use super::mollie::payments::PaymentsApi;
use log::{debug, info, warn};
use pad::{Alignment, PadStr};

pub fn command() {
    debug!("Listing 10 Payments");

    let client = mollie::ApiClient::new();

    let response = client.list_payments();

    match response {
        Ok(success) => list_payments_from_response(success),
        Err(err) => handle_error(err),
    }
}

fn list_payments_from_response(response: super::mollie::payments::ListPaymentsResponse) {
    for payment in response.embedded.payments {
        info!(
            "{} | {} {} | {}",
            payment.id,
            payment
                .amount
                .value
                .pad_to_width_with_alignment(8, Alignment::Right),
            payment.amount.currency,
            payment.status
        );
    }
}

fn handle_error(err: super::mollie::payments::ApiError) {
    debug!("{:?}", err);
    warn!("🚨 There was an error communicating with the Mollie API 🚨");

    match err {
        super::mollie::payments::ApiError::CouldNotPerformRequest(i) => {
            warn!("Could not reach the Mollie API to perform the request.");
            debug!("{:?}", i);
        }
        super::mollie::payments::ApiError::CouldNotUnderstandResponse(i) => {
            warn!("Request failed catastrophically and could not understand the Mollie API error response.");
            debug!("{:?}", i);
        }
        super::mollie::payments::ApiError::MollieApiReturnedAnError(i) => {
            warn!("Request to the Mollie API Failed: {}", i.detail);

            if i.status == 401 {
                warn!("Run mol auth to set up your authentication with the Mollie API");
                warn!("Run mol env url {{prod|dev}} to switch environments");
            }
        }
    }
}
