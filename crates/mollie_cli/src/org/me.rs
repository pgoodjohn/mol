use crate::{config::MollieConfig, apiauth::CliAuthProvider};
use mollie_api::Mollie;

pub async fn command(config: &MollieConfig) -> anyhow::Result<()> {
    let response = Mollie::build(&CliAuthProvider::new(config))
        .organizations()
        .me()
        .await?;
    println!("Organization: {:#?}", response.id);
    println!("{:#?}", response);
    Ok(())
}
