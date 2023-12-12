use mollie_api::Mollie;
use crate::config::MollieConfig;
use log::{debug, info};

pub async fn command(config: &MollieConfig, payment_id: &String) -> anyhow::Result<()>{
    debug!("Running Get API Payment for payment: {}", payment_id);

    let token = config.bearer_token()?;

    let payment = Mollie::build(token.as_str()).payments().get_by_id(payment_id).await?;

    debug!("{:?}", payment);

    info!(
        "{} | {} | {} {} | {}",
        payment.id, payment.mode, payment.amount.value, payment.amount.currency, payment.status
    );

   

    return Ok(());
}
