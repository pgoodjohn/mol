use super::config;
use log::{debug, info};
use requestty::Question;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use super::mollie_sdk;

pub fn command(
    input_currency: Option<&String>,
    input_amount: Option<&String>,
    input_description: Option<&String>,
    input_redirect_url: Option<&String>,
    input_profile_id: Option<&String>,
    debug: &bool,
) {
    debug!("Running Create Payment Command");
    let currency = Currency(String::from(input_currency.unwrap()));
    let amount = Amount {
        currency: currency,
        value: format!("{:.2}", input_amount.unwrap().parse::<f64>().unwrap()),
    };
    debug!("{:?}", amount);
    let description = Description(String::from(input_description.unwrap()));
    let redirect_url = RedirectUrl(String::from(input_redirect_url.unwrap()));
    let profile_id = Some(String::from(input_profile_id.unwrap()));

    let create_payment_request = CreatePaymentRequest {
        amount,
        description,
        redirect_url,
        profile_id,
    };

    if debug == &true {
        let json = serde_json::to_string(&create_payment_request).unwrap();
        debug!("Request Body: {:?}", json);
        ask_confirmation();
    }

    execute_request(create_payment_request);
}

pub fn interactive(debug: &bool) {
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
    // Profile ID - prompted only if auth is via access code
    let profile_id = ask_profile_id().unwrap();
    let create_payment_request = CreatePaymentRequest {
        amount,
        description,
        redirect_url,
        profile_id,
    };

    if debug == &true {
        let json = serde_json::to_string(&create_payment_request).unwrap();
        debug!("Request Body: {:?}", json);
        ask_confirmation();
    }

    execute_request(create_payment_request);
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CreatePaymentRequest {
    amount: Amount,
    description: Description,
    redirect_url: RedirectUrl,
    profile_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PaymentCreatedResponse {
    resource: String,
    id: String,
    description: String,
    method: Option<String>,
    status: String,
    mode: String,
}

fn execute_request(request: CreatePaymentRequest) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Connecting with the Mollie API");

    let client = mollie_sdk::ApiClient::new();
    let response = client.post(request, String::from("v2/payments")).unwrap();

    // HTTP 201 Response means the payment was created successfully
    if response.status() == StatusCode::CREATED {
        debug!("Successfull call to the Mollie API!");
        let decoded_response = response.json::<PaymentCreatedResponse>().unwrap();
        debug!("{:?}", decoded_response);

        // This should just check if there's a checkout URL on the response and then just print it
        match decoded_response.method {
            Some(_) => info!(
                "I still don't support going to the method URL directly, but the payment ID is: {}",
                decoded_response.id
            ),
            None => info!(
                // This shouldn't be api.mollie.xxx but just mollie.xxx
                "Pay this payment: {}/checkout/select-method/{}",
                config::api_url().unwrap(),
                decoded_response.id
            ),
        }

        return Ok(());
    }

    // Any other response is an error
    mollie_sdk::handle_mollie_api_error(response);
    Ok(())
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
                true => {
                    debug!("Ok - continuing")
                }
                false => {
                    debug!("oh oh");
                    panic!("aborting")
                }
            }

            return;
        }
        Err(_) => {
            panic!("Smth went wrong :O")
        }
    }
}

#[derive(Serialize, Debug)]
struct Currency(String);

#[derive(Serialize, Debug)]
struct Amount {
    currency: Currency,
    value: String,
}

#[derive(Serialize, Debug)]
struct Description(String);

#[derive(Serialize, Debug)]
struct RedirectUrl(String);

#[derive(Debug)]
struct SorryCouldNotCreatePayment {}

fn ask_currency() -> Result<Currency, SorryCouldNotCreatePayment> {
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
            Ok(Currency(String::from(answer)))
        }
        Err(_) => Err(SorryCouldNotCreatePayment {}),
    }
}

fn ask_amount(currency: Currency) -> Result<Amount, SorryCouldNotCreatePayment> {
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
                currency: currency,
                value: format!("{:.2}", answer),
            };
            debug!("Amount {:?} (not validated)", amount);

            // TODO: add validation
            Ok(amount)
        }
        Err(_) => Err(SorryCouldNotCreatePayment {}),
    }
}

fn ask_description() -> Result<Description, SorryCouldNotCreatePayment> {
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
            Ok(Description(String::from(answer)))
        }
        Err(_) => Err(SorryCouldNotCreatePayment {}),
    }
}

fn ask_redirect_url() -> Result<RedirectUrl, SorryCouldNotCreatePayment> {
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
            Ok(RedirectUrl(String::from(answer)))
        }
        Err(_) => Err(SorryCouldNotCreatePayment {}),
    }
}

fn ask_profile_id() -> Result<Option<String>, SorryCouldNotCreatePayment> {
    match config::access_code() {
        Ok(_) => {
            // found access code, continue
        }
        Err(_) => {
            return Ok(None);
        }
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
