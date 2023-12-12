use log::{debug, info};
use mollie_api::Mollie;
use crate::config;

pub async fn command(limit: &Option<i32>, from: &Option<String>) -> anyhow::Result<()> {
    debug!("Listing balances");
    let token = config::get_bearer_token().unwrap();
    let balance = Mollie::build(&token.value).balances().list(*limit, from).await?;

    info!("Listing balances");
    balance.embedded.balances.iter().enumerate().for_each(|(index, balance)| {
        info!("{}. {}", index, balance.to_string());
    });
    debug!("{:?}", balance);

    Ok(())
}