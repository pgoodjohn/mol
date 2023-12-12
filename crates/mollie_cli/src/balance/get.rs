use log::{debug, info};
use mollie_api::Mollie;
use crate::config;

pub async fn command(balance_id: &String) -> anyhow::Result<()> {
    debug!("Running Get API Balance for balance: {}", balance_id);
    let token = config::get_bearer_token().unwrap();
    let balance = Mollie::build(&token.value).balances().get_by_id(balance_id).await?;

    info!("{}", balance.to_string());
    debug!("{:#?}", balance);

    Ok(())
}
