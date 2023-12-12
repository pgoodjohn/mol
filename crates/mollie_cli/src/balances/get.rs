use colored::Colorize;
use log::{debug, info};
use mollie_api::Mollie;
use crate::balances::Balance;
use crate::config::MollieConfig;

pub async fn command(config: &MollieConfig, balance_id: &String) -> anyhow::Result<()> {
    debug!("Running Get API Balance for balance: {}", balance_id);
    let token = config.bearer_token()?;
    let balance = Mollie::build(token.as_str())
        .balances()
        .get_by_id(balance_id)
        .await?;

    info!("{}", Colorize::bright_black(&*Balance::header()));
    info!("{}", Balance::from(balance.clone()).to_string());
    debug!("{:#?}", balance);

    Ok(())
}
