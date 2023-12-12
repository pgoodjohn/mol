use crate::config::MollieConfig;
use colored::Colorize;
use log::{debug, info, warn};
use mollie_api::Mollie;
use requestty::Question;
use serde::Serialize;

pub async fn command(
    config: &MollieConfig,
    input_currency: Option<&String>,
    input_amount: Option<&String>,
    input_description: Option<&String>,
    input_redirect_url: Option<&String>,
    input_profile_id: Option<&String>,
    debug: &bool,
) -> anyhow::Result<()> {
    debug!("Running Create Payment Command");
    let currency = String::from(input_currency.unwrap());
    let description = String::from(input_description.unwrap());
    let redirect_url = String::from(input_redirect_url.unwrap());
    let profile_id = Some(String::from(input_profile_id.unwrap()));

    let create_payment_request = mollie_api::models::payment::CreatePaymentRequest {
        amount: mollie_api::models::amount::Amount {
            value: format!("{:.2}", input_amount.unwrap().parse::<f64>().unwrap()),
            currency,
        },
        description,
        redirect_url,
        profile_id,
    };

    if debug == &true {
        let json = serde_json::to_string(&create_payment_request).unwrap();
        debug!("Request Body: {:?}", json);
        ask_confirmation();
    }

    let token = config.bearer_token()?;

    let response = Mollie::build(&token.as_str())
        .payments()
        .create_payment(&create_payment_request)
        .await;

    log::debug!("{:?}", response);
    match response {
        Ok(payment) => handle_payment_created_response(payment),
        Err(e) => info!("{}", e),
    }
    return Ok(());
}

pub async fn interactive(config: &MollieConfig, debug: &bool) -> anyhow::Result<()> {
    debug!("Running interactive Create Payment Command");

    // Currency
    let currency = ask_currency().unwrap();
    // Amount
    let amount = ask_amount(currency).unwrap();
    // Description
    let description = ask_description().unwrap();
    // Redirect URL
    let redirect_url = ask_redirect_url().unwrap();
    // Webhook (Optional fields [...])
    // Profile ID - prompted only if auth is via access token
    let profile_id = ask_profile_id(config).unwrap();
    let create_payment_request = mollie_api::models::payment::CreatePaymentRequest {
        amount: mollie_api::models::amount::Amount {
            currency: amount.currency,
            value: amount.value,
        },
        description,
        redirect_url,
        profile_id,
    };

    if debug == &true {
        let json = serde_json::to_string(&create_payment_request).unwrap();
        debug!("Request Body: {:?}", json);
        ask_confirmation();
    }

    let token = config.bearer_token()?;

    let response = Mollie::build(token.as_str())
        .payments()
        .create_payment(&create_payment_request)
        .await;

    log::debug!("{:?}", response);
    match response {
        Ok(payment) => handle_payment_created_response(payment),
        Err(e) => info!("{}", e),
    }
    return Ok(());
}

fn handle_payment_created_response(response: mollie_api::models::payment::PaymentResource) {
    match response.links.get("checkout") {
        Some(checkout_url) => {
            info!("Pay this payment: {}", Colorize::blue(&*checkout_url.href));
            qr2term::print_qr(checkout_url.href.clone()).ok(/* only print qrcode if everything is fine */);
        }
        None => warn!("Couldn't find the checkout url!"),
    }
}

fn ask_confirmation() {
    let question = Question::confirm("request")
        .message("Are you sure? [y/N]")
        .default(false)
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let answer = result.as_bool().unwrap();

            match answer {
                true => debug!("Ok - continuing"),
                false => {
                    debug!("oh oh");
                    panic!("aborting")
                }
            }
        }
        Err(_) => {
            panic!("Smth went wrong :O")
        }
    }
}

#[derive(Serialize, Debug)]
struct Amount {
    currency: String,
    value: String,
}

#[derive(Debug)]
struct SorryCouldNotCreatePayment {}

fn ask_currency() -> Result<String, SorryCouldNotCreatePayment> {
    let question = Question::input("currency")
        .message("Currency (3 letter code)")
        .default("EUR")
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let answer = result.as_string().unwrap();

            debug!("Selected currency {} - not yet validated", answer);

            // TODO: add validation
            Ok(String::from(answer))
        }
        Err(_) => Err(SorryCouldNotCreatePayment {}),
    }
}

fn ask_amount(currency: String) -> Result<Amount, SorryCouldNotCreatePayment> {
    let question = Question::float("amount")
        .message("Amount (format depends on your desired currency")
        .default(1.00)
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let answer = result.as_float().unwrap();
            debug!("Input amount {} - not yet validated", answer);
            let amount = Amount {
                currency,
                value: format!("{:.2}", answer),
            };
            debug!("Amount {:?} (not validated)", amount);

            // TODO: add validation
            Ok(amount)
        }
        Err(_) => Err(SorryCouldNotCreatePayment {}),
    }
}

fn ask_description() -> Result<String, SorryCouldNotCreatePayment> {
    let question = Question::input("description")
        .message("Choose a description")
        .default("N/A")
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let answer = result.as_string().unwrap();

            debug!("Description: {} - not yet validated", answer);

            // TODO: add validation
            Ok(String::from(answer))
        }
        Err(_) => Err(SorryCouldNotCreatePayment {}),
    }
}

fn ask_redirect_url() -> Result<String, SorryCouldNotCreatePayment> {
    let question = Question::input("redirect_url")
        .message("Choose a redirect_url")
        .default("https://example.com/?source=mol-cli")
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let answer = result.as_string().unwrap();

            debug!("Redirect URL: {} - not yet validated", answer);

            // TODO: add validation
            Ok(String::from(answer))
        }
        Err(_) => Err(SorryCouldNotCreatePayment {}),
    }
}

fn ask_profile_id(config: &MollieConfig) -> Result<Option<String>, SorryCouldNotCreatePayment> {
    if !config.auth.access_code.is_some() {
        return Ok(None);
    }

    let question = Question::input("profile_id")
        .message("Input a profile id")
        .default("pfl_CRjJMqbnVr")
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let answer = result.as_string().unwrap();

            debug!("Redirect URL: {} - not yet validated", answer);

            // TODO: add validation
            Ok(Some(String::from(answer)))
        }
        Err(_) => Err(SorryCouldNotCreatePayment {}),
    }
}
