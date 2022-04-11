use super::mollie::refunds;
use super::mollie::refunds::RefundsApi;
use log::debug;

pub fn command(payment_id: &String, amount: &f32, description: &String) {
    debug!("Refunding payment");
    let request = refunds::RefundPaymentRequest {
        amount: refunds::Amount {
            value: format!("{:.2}", amount),
            currency: String::from("EUR"),
        },
        description: String::from(description),
    };

    let client = super::mollie::ApiClient::new();

    let response = client.refund_payment(String::from(payment_id), request);

    debug!("{:?}", response);
}
