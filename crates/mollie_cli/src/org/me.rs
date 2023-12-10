use mollie_api::Mollie;

pub async fn command() -> anyhow::Result<()> {
    let token = super::config::get_bearer_token().unwrap();
    let response = Mollie::build(&token.value).organizations().me().await?;
    println!("Organization: {:#?}", response.id);
    println!("{:#?}", response);
    Ok(())
}
