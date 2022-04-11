use super::console;
use super::mollie;
use super::mollie::payments::PaymentsApi;
use log::{debug, info};
use pad::{Alignment, PadStr};

pub fn command() {
    debug!("Listing 10 Payments");

    let client = mollie::ApiClient::new();

    let response = client.list_payments();

    match response {
        Ok(success) => list_payments_from_response(success),
        Err(err) => console::handle_mollie_client_error(err),
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
