use colored::Colorize;
use colored_json::ToColoredJson;
use log::{debug, info};
use mollie_api::Mollie;
use crate::balances::Balance;
use crate::config::MollieConfig;

pub async fn command(config: &MollieConfig, balance_id: &String, with_response: bool) -> anyhow::Result<()> {
    debug!("Running Get API Balance for balance: {}", balance_id);
    let token = config.bearer_token()?;
    let balance = Mollie::build(token.as_str())
        .balances()
        .get_by_id(balance_id)
        .await?;

    info!("{}", Colorize::bright_black(&*Balance::header()));
    info!("{}", Balance::from(balance.clone()).to_string());
    debug!("{:#?}", balance);

    if with_response {
        let pretty_json = jsonxf::pretty_print(&serde_json::to_string(&balance).unwrap()).unwrap();
        info!("{}", pretty_json.to_colored_json_auto().unwrap());
    }

    Ok(())
}
