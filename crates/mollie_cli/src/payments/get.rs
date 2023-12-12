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
) -> anyhow::Result<()> {
    debug!("Running Get API Payment for payment: {}", payment_id);

    let token = config.bearer_token()?;

    let payment = Mollie::build(&token.as_str())
        .payments()
        .get_by_id(payment_id)
        .await;

    debug!("{:?}", payment);
    match payment {
        Ok(p) => {
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
