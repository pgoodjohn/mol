use super::console;
use super::mollie::refunds;
use super::mollie::refunds::RefundsApi;
use log::info;

pub fn command(payment_id: &String, amount: &f32, description: &String) {
    let request = refunds::RefundPaymentRequest {
        amount: refunds::Amount {
            value: format!("{:.2}", amount),
            currency: String::from("EUR"),
        },
        description: String::from(description),
    };

    let client = super::mollie::ApiClient::new();

    match client.refund_payment(String::from(payment_id), request) {
        Ok(response) => {
            info!("Refund {} for payment {} was created. It will be processed in 2h if there is enough balance on your organization.", response.id, payment_id);
        }
        Err(e) => {
            console::handle_mollie_client_error(e);
        }
    }
}
