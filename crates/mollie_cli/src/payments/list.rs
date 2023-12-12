use crate::config::MollieConfig;

use super::console;
use super::mollie;
use super::mollie::payments::PaymentsApi;
use log::{debug, info};
use pad::{Alignment, PadStr};

pub fn command(config: &MollieConfig, limit: &Option<i32>, from: &Option<String>) {
    debug!("Listing 10 Payments");

    let client = mollie::ApiClientBuilder::new()
        .blocking()
        .url(config.api.url.to_string())
        .auth(config.bearer_token().unwrap())
        .spawn();

    let response = client.list_payments(*limit, from);

    match response {
        Ok(success) => list_payments_from_response(success),
        Err(err) => console::handle_mollie_client_error(err),
    }
}

fn list_payments_from_response(response: super::mollie::payments::ListPaymentsResponse) {
    let mut i = 0;
    for payment in response.embedded.payments {
        i += 1;
        info!(
            "{:2}. | {} | {} {} | {} | {}",
            i,
            payment.id,
            payment
                .amount
                .value
                .pad_to_width_with_alignment(8, Alignment::Right),
            payment.amount.currency,
            payment.status,
            payment.created_at
        );
    }
}
