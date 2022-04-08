use super::mollie;
use super::mollie::payments::PaymentsApi;
use log::{debug, info};

pub fn command(payment_id: &String) {
    debug!("Running Get API Payment for payment: {}", payment_id);
    let client = mollie::ApiClient::new();

    let payment = client.get_payment_details(payment_id).unwrap();

    debug!("{:?}", payment);

    info!(
        "{} | {} | {} {} | {}",
        payment.id, payment.mode, payment.amount.value, payment.amount.currency, payment.status
    );
}
