use super::console;
use super::config;

use mollie_api::Mollie;
use log::info;

pub async fn command(payment_id: &String, amount: &f32, description: &String) -> anyhow::Result<()>{
    let request = mollie_api::models::refund::RefundPaymentRequest {
        amount: mollie_api::models::amount::Amount {
            value: format!("{:.2}", amount),
            currency: String::from("EUR"),
        },
        description: String::from(description),
    };

    let token = super::config::get_bearer_token().unwrap();
    let response = Mollie::build(&token.value).refunds().refund(&payment_id, &request).await?;
    match response {
        Ok(response) => {
            info!("Refund {} for payment {} was created. It will be processed in 2h if there is enough balance on your organization.", response.id, payment_id);
        }
        Err(e) => {
            console::handle_mollie_client_error(e);
        }
    };

    return Ok(());
}
