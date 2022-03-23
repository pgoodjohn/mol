use super::config;
use log::{debug, info};
use regex::Regex;
use requestty::Question;
use strum::EnumString;

pub fn interactive() -> Result<(), &'static str> {
    let new_api_key = ask_api_key().unwrap();

    // TODO: Implement access code through interactive command
    store_api_key(new_api_key);

    Ok(())
}

pub fn api_key(api_key: &String) -> Result<(), &'static str> {
    let new_api_key = ApiKey::from_string(String::from(api_key));

    store_api_key(new_api_key);

    Ok(())
}

pub fn access_code(access_code: &String) -> Result<(), &'static str> {
    let new_access_code = AccessCode::from_string(String::from(access_code));

    store_access_token(new_access_code);

    Ok(())
}

fn store_access_token(new_access_code: AccessCode) {
    let old_config = config::from_file().unwrap();

    let mut new_config = old_config.clone();
    new_config.access_code = Some(new_access_code.value);

    debug!("Old config: {:?}", old_config);
    debug!("New config: {:?}", new_config);

    config::save_to_file(new_config).unwrap();

    info!("Configuration updated");
}

fn store_api_key(new_api_key: ApiKey) {
    let old_config = config::from_file().unwrap();

    let mut new_config = old_config.clone();
    match new_api_key.mode {
        ApiKeyMode::Live => {
            new_config.keys.live = Some(new_api_key.value);
        }
        ApiKeyMode::Test => {
            new_config.keys.test = Some(new_api_key.value);
        }
    }

    debug!("Old config: {:?}", old_config);
    debug!("New config: {:?}", new_config);

    config::save_to_file(new_config).unwrap();

    info!("Configuration updated");
}

pub struct ApiKey {
    value: String,
    mode: ApiKeyMode,
}

#[derive(EnumString, Debug)]
pub enum ApiKeyMode {
    Live,
    Test,
}

impl ApiKeyMode {
    pub fn from_string(value: String) -> Self {
        match value.as_str() {
            "live" => ApiKeyMode::Live,
            "test" => ApiKeyMode::Test,
            _ => {
                panic!("Invalid mode")
            }
        }
    }
}

impl ApiKey {
    pub fn from_string(value: String) -> Self {
        let api_key_format = Regex::new(r"^(test|live)_{1}\w{30}$").unwrap();

        if api_key_format.is_match(&value) == false {
            panic!("Invalid Api Key format");
        }

        let cap = api_key_format.captures(&value).unwrap();
        let mode = ApiKeyMode::from_string(cap[1].to_string());

        ApiKey { value, mode }
    }
}

struct AccessCode {
    value: String,
}

impl AccessCode {
    pub fn from_string(value: String) -> Self {
        let access_code_format = Regex::new(r"^(access)_{1}\w{40}$").unwrap();

        if access_code_format.is_match(&value) == false {
            panic!("Invalid Access Code format");
        }
        
        AccessCode { value }
    }
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
            Ok(ApiKey::from_string(String::from(answer)))
        }
        Err(_) => Err(SorryCouldNotRetrieveApiKey {}),
    }
}
