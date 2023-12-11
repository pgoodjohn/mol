use crate::balances::Balance;
use crate::config::MollieConfig;
use colored::Colorize;
use log::{debug, info};
use mollie_api::Mollie;

pub async fn command(
    config: &MollieConfig,
    limit: &Option<i32>,
    from: &Option<String>,
) -> anyhow::Result<()> {
    debug!("Listing balances");
    let token = config.bearer_token()?;
    let balances = Mollie::build(token.as_str())
        .balances()
        .list(*limit, from)
        .await?;

    info!("Listing balances");
    info!("   {}", Colorize::bright_black(&*Balance::header()));
    balances
        .embedded
        .balances
        .iter()
        .enumerate()
        .for_each(|(index, balance)| {
            info!(
                "{}. {}",
                index + 1,
                Balance::from(balance.clone()).to_string()
            );
        });
    debug!("{:?}", balances);

    Ok(())
}
