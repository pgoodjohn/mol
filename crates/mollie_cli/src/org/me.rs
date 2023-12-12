use crate::{config::{MollieConfig, ConfigurationService}, apiauth::CliAuthProvider};
use mollie_api::Mollie;

pub async fn command(config: &mut dyn ConfigurationService) -> anyhow::Result<()> {
    let response = Mollie::build(&CliAuthProvider::new(config))
        .organizations()
        .me()
        .await?;
    println!("Organization: {:#?}", response.id);
    println!("{:#?}", response);
    Ok(())
}
