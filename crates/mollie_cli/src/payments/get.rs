use crate::config::MollieConfig;

use super::mollie;
use super::mollie::payments::PaymentsApi;
use log::{debug, info};

pub fn command(config: &MollieConfig, payment_id: &String) {
    debug!("Running Get API Payment for payment: {}", payment_id);

    let client = mollie::ApiClientBuilder::new()
        .blocking()
        .url(config.api.url.to_string())
        .auth(config.bearer_token().unwrap())
        .spawn();

    let payment = client.get_payment_details(payment_id).unwrap();

    debug!("{:?}", payment);

    info!(
        "{} | {} | {} {} | {}",
        payment.id, payment.mode, payment.amount.value, payment.amount.currency, payment.status
    );
}
