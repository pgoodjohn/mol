use log::{debug, info};
use pad::{Alignment, PadStr};
use mollie_api::Mollie;

pub async fn command(limit: &Option<i32>, from: &Option<String>, profile_id: &Option<String>, test_mode: &Option<bool>)-> anyhow::Result<()> {
    debug!("Listing 10 Payments");
    let token = super::config::get_bearer_token().unwrap();
    let response = Mollie::build(&token.value).payments().list(limit, from, profile_id, test_mode).await?;
    //let response = client.list_payments(*limit, from);
    list_payments_from_response(response);    
    return Ok(());
}

fn list_payments_from_response(response: mollie_api::models::payment::PaymentsListResponse) {
    let mut i = 0;
    for payment in response.embedded.payments {
        i += 1;
        info!(
            "{:2}. | {} | {} {} | {} | {}",
            i,
            payment.id,
            payment
                .amount
                .value
                .pad_to_width_with_alignment(8, Alignment::Right),
            payment.amount.currency,
            payment.status,
            payment.created_at
        );
    }
}


//pub async fn command() -> anyhow::Result<()> {
//    let token = super::config::get_bearer_token().unwrap();
//   let response = Mollie::build(&token.value).organizations().me().await?;
//    println!("Organization: {:#?}", response.id);
//    println!("{:#?}", response);
//    Ok(())
//}
