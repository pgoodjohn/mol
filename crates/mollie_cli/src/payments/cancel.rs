use mollie_api::Mollie;
use log::{debug, info};
use colored::Colorize;
use crate::payments::Payment;

pub async fn command(payment_id: &String) -> anyhow::Result<()>{
    debug!("Running Cancel API Payment for paymner: {}", payment_id);

    let token = super::config::get_bearer_token().unwrap();

    let cancel = Mollie::build(&token.value).payments().cancel(payment_id).await;

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
