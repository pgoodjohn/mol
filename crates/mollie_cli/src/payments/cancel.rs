use crate::config::MollieConfig;
use crate::payments::Payment;
use colored::Colorize;
use colored_json::ToColoredJson;
use log::{debug, info};
use mollie_api::Mollie;

pub async fn command(
    config: &MollieConfig,
    payment_id: &String,
    with_response: bool,
) -> miette::Result<()> {
    debug!("Running Cancel API Payment for paymner: {}", payment_id);

    let token = config.bearer_token()?;

    let cancel = Mollie::build(&token.as_str())
        .payments()
        .cancel(payment_id)
        .await;

    debug!("{:?}", cancel);
    match cancel {
        Ok(p) => {
            info!("{}", Colorize::red("Payment Cancelled"));
            info!("{}", Colorize::bright_black(&*Payment::header()));
            info!("{}", Payment::from(p.clone()).to_string());
            if with_response {
                let pretty_json =
                    jsonxf::pretty_print(&serde_json::to_string(&p).unwrap()).unwrap();
                info!("{}", pretty_json.to_colored_json_auto().unwrap());
            }
        }
        Err(e) => info!("{}", e),
    }

    return Ok(());
}
