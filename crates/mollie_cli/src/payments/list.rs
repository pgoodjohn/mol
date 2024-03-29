use crate::config::MollieConfig;
use crate::payments::Payment;
use crate::payments::Status;
use colored::Colorize;
use colored_json::ToColoredJson;
use log::{debug, info};
use mollie_api::Mollie;

pub async fn command(
    config: &MollieConfig,
    limit: &Option<i32>,
    from: &Option<String>,
    profile_id: &Option<String>,
    test_mode: &Option<bool>,
    with_response: bool,
    status: &Option<Status>,
) -> miette::Result<()> {
    debug!("Listing 10 Payments");
    let token = config.bearer_token()?;
    let response = Mollie::build(&token.as_str())
        .payments()
        .list(limit, from, profile_id, test_mode)
        .await;
    match response {
        Ok(res) => {
            list_payments_from_response(res, with_response, status);
        }
        Err(e) => info!("{}", e),
    }
    return Ok(());
}

fn list_payments_from_response(
    response: mollie_api::models::payment::PaymentsListResource,
    with_response: bool,
    status: &Option<Status>,
) {
    info!(" {}", Colorize::bright_black(&*Payment::header()));
    /*response.embedded.payments.iter().enumerate().for_each(|(index, payment)| {
        info!("{}. {}", index + 1, Payment::from(payment.clone()).to_string());
    });
    */
    match status {
        Some(s) => {
            response
                .embedded
                .payments
                .iter()
                .enumerate()
                .for_each(|(index, payment)| {
                    if payment.status == s.to_string().to_lowercase() {
                        info!(
                            "{}. {}",
                            index + 1,
                            Payment::from(payment.clone()).to_string()
                        );
                    }
                });
        }
        None => {
            response
                .embedded
                .payments
                .iter()
                .enumerate()
                .for_each(|(index, payment)| {
                    info!(
                        "{}. {}",
                        index + 1,
                        Payment::from(payment.clone()).to_string()
                    );
                });
        }
    }

    if with_response {
        let pretty_json = jsonxf::pretty_print(&serde_json::to_string(&response).unwrap()).unwrap();
        info!("{}", pretty_json.to_colored_json_auto().unwrap());
    }
}
