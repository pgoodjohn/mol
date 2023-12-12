use mollie_api::Mollie;
use log::{debug, info};
use colored::Colorize;
use crate::payments::Payment;
use crate::config::MollieConfig;

pub async fn command(config: &MollieConfig, payment_id: &String) -> anyhow::Result<()>{
    debug!("Running Cancel API Payment for paymner: {}", payment_id);

    let token = config.bearer_token()?;

    let cancel = Mollie::build(&token.as_str()).payments().cancel(payment_id).await;

    debug!("{:?}", cancel);
    match cancel {
        Ok(p) => {
            info!("{}", Colorize::red("Payment Cancelled"));
            info!("{}", Colorize::bright_black(&*Payment::header()));
            info!("{}", Payment::from(p.clone()).to_string());
        },
        Err(e) => info!("{}", e), 
    }

    return Ok(());
}
