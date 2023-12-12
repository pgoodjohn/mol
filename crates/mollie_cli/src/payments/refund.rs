use crate::config::MollieConfig;
use colored_json::ToColoredJson;
use mollie_api::Mollie;

pub async fn command(
    config: &MollieConfig,
    payment_id: &String,
    amount: &f32,
    description: &String,
    with_response: bool,
) -> miette::Result<()> {
    let request = mollie_api::models::refund::RefundPaymentRequest {
        amount: mollie_api::models::amount::Amount {
            value: format!("{:.2}", amount),
            currency: String::from("EUR"),
        },
        description: String::from(description),
    };

    let token = config.bearer_token()?;
    let response = Mollie::build(&token.as_str())
        .refunds()
        .refund(&payment_id, &request)
        .await;

    match response {
        Ok(res) => {
            log::info!("{}", res.to_string());
            if with_response {
                let pretty_json =
                    jsonxf::pretty_print(&serde_json::to_string(&res).unwrap()).unwrap();
                log::info!("{}", pretty_json.to_colored_json_auto().unwrap());
            }
        }
        Err(e) => log::info!("{:?}", e),
    }

    return Ok(());
}
