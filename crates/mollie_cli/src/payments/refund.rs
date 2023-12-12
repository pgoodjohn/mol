use crate::config::MollieConfig;
use mollie_api::Mollie;

pub async fn command(
    config: &MollieConfig,
    payment_id: &String,
    amount: &f32,
    description: &String,
) -> anyhow::Result<()> {
    let request = mollie_api::models::refund::RefundPaymentRequest {
        amount: mollie_api::models::amount::Amount {
            value: format!("{:.2}", amount),
            currency: String::from("EUR"),
        },
        description: String::from(description),
    };

    let token = super::config::get_bearer_token().unwrap();
    let response = Mollie::build(&token.value).refunds().refund(&payment_id, &request).await;

    match response {
        Ok(res) => log::info!("{}", res.to_string()),
        Err(e) => log::info!("{:?}", e),
    }



    return Ok(());
}
