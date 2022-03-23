use super::config;
use log::{debug, info, warn};
use requestty::Question;
use serde::{Deserialize, Serialize};
use std::fs;
use toml;

pub fn command() -> Result<(), &'static str> {
    debug!("Running Store API key");

    let new_api_key = ask_api_key().unwrap();

    debug!("New API key will be {}", &new_api_key.value);

    replace_api_key(new_api_key);

    Ok(())
}

fn replace_api_key(new_api_key: ApiKey) {
    let old_config = config::from_file().unwrap();

    match old_config.keys.live {
        Some(_) => {
            info!("Replacing previously stored key");
        }
        None => {
            info!("Storing new key");
        }
    }

    let mut new_config = old_config.clone();
    new_config.keys.live = Some(new_api_key.value);

    debug!("Old config: {:?}", old_config);
    debug!("New config: {:?}", new_config);

    config::save_to_file(new_config).unwrap();
}

pub struct ApiKey {
    value: String,
}

#[derive(Debug)]
pub struct SorryCouldNotRetrieveApiKey {}

fn ask_api_key() -> Result<ApiKey, SorryCouldNotRetrieveApiKey> {
    let question = Question::input("api_key")
        .message("Input your new API key")
        .default("live_Rrm6VWAGFDPwA6fuv759BeKr2J882s")
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let answer = result.as_string().unwrap();

            debug!("New api key {} - not yet validated", answer);

            // TODO: add validation
            Ok(ApiKey {
                value: String::from(answer),
            })
        }
        Err(_) => Err(SorryCouldNotRetrieveApiKey {}),
    }
}
