use super::config;
use super::mollie_sdk;
use log::{debug, info, warn};
use requestty::Question;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::env;

pub fn command() -> Result<(), &'static str> {
    debug!("Running Create Payment Command");

    // TODO: Ask for inputs
    // Currency
    let currency = ask_currency().unwrap();
    // Amount
    let amount = ask_amount(currency).unwrap();
    // Description
    let description = ask_description().unwrap();
    // Redirect URL
    let redirect_url = ask_redirect_url().unwrap();
    // Webhook (Optional fields [...])

    // TODO: Create HTTP request
    let create_payment_request = CreatePaymentRequest {
        amount,
        description,
        redirect_url,
    };
    // TODO: If debug mode enabled show request and validate before sending

    // TODO: Send request to Mollie Dev - will need to look into tokio for async stuff probs
    execute_create_payment_request(create_payment_request);

    // TODO: Show some details of response

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct MollieApiError {
    status: i32,
    title: String,
    detail: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PaymentCreatedResponse {
    resource: String,
    id: String,
    mode: String,
    description: String,
    method: Option<String>,
    status: String,
}

fn execute_create_payment_request(
    create_payment_request: CreatePaymentRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Making HTTP Request");

    let request_json = &serde_json::json!({
        "description": create_payment_request.description.value,
        "redirectUrl": create_payment_request.redirect_url.value,
        "amount": {
            "currency": create_payment_request.amount.currency.code,
            "value": format!("{:.2}", create_payment_request.amount.value),
        },
    });

    debug!("Request Body: {:?}", request_json);

    // Load API key from ~/.mol/conf.toml
    let api_key = config::api_key().unwrap();

    // TODO: Enable usage with production
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://api.mollie.dev/v2/payments")
        .bearer_auth(api_key)
        .header(
            reqwest::header::USER_AGENT,
            format!(
                "{} {} / {}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_REPOSITORY")
            ),
        )
        .json(request_json)
        .send()?;

    // HTTP 201 Response means the payment was created successfully
    if response.status() == StatusCode::CREATED {
        debug!("Successfull call to the Mollie API!");
        let decoded_response = response.json::<PaymentCreatedResponse>().unwrap();
        debug!("{:?}", decoded_response);

        match decoded_response.method {
            Some(_) => info!(
                "I still don't support going to the method URL directly, but the payment ID is: {}",
                decoded_response.id
            ),
            None => info!(
                "Pay this payment: https://mollie.dev/checkout/select-method/{}",
                decoded_response.id
            ),
        }

        return Ok(());
    }

    // Any other response is an error
    mollie_sdk::handle_mollie_api_error(response);

    // TODO: Return CLI error
    Ok(())
}

#[derive(Debug)]
struct CreatePaymentRequest {
    amount: Amount,
    description: Description,
    redirect_url: RedirectUrl,
}

#[derive(Debug)]
struct Currency {
    code: String,
}

#[derive(Debug)]
struct Amount {
    currency: Currency,
    value: f64,
}

#[derive(Debug)]
struct Description {
    value: String,
}

#[derive(Debug)]
struct RedirectUrl {
    value: String,
}

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
            Ok(Currency {
                code: String::from(answer),
            })
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
                value: answer,
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
            Ok(Description {
                value: String::from(answer),
            })
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
            Ok(RedirectUrl {
                value: String::from(answer),
            })
        }
        Err(_) => Err(SorryCouldNotCreatePayment {}),
    }
}
