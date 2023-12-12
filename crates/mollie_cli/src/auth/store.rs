use super::config;
use log::{debug, info};
use mollie_api::auth::{AccessCode, ApiKey, ApiKeyMode, OAuth};
use requestty::Question;

pub fn interactive() {
    let new_api_key = ask_api_key().unwrap();

    // TODO: Implement access code through interactive command
    store_api_key(new_api_key);
}

pub fn api_key(api_key: &String) {
    // TODO: use result instead of expect
    let new_api_key = ApiKey::from_string(String::from(api_key)).expect("Invalid API key");

    store_api_key(new_api_key);
}

pub fn store_refresh_token(refresh_token: &String) {
    // TODO: use result instead of expect
    let new_oauth = OAuth { refresh_token: refresh_token.into(), access_token: None };

    store_oauth(new_oauth);
}

pub fn access_code(access_code: &String) {
    // TODO: use result instead of expect
    let new_access_code =
        AccessCode::from_string(String::from(access_code)).expect("Invalid access code");

    store_access_token(new_access_code);
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

fn store_oauth(oauth: OAuth) {
    let old_config = config::from_file().unwrap();

    let mut new_config = old_config.clone();
    new_config.refresh_token = Some(oauth.refresh_token);

    debug!("Old config: {:?}", old_config);
    debug!("New config: {:?}", new_config);

    config::save_to_file(new_config).unwrap();

    info!("Configuration updated");
}

#[derive(Debug)]
pub struct SorryCouldNotRetrieveApiKey {
    pub error_message: String,
}

fn ask_api_key() -> Result<ApiKey, SorryCouldNotRetrieveApiKey> {
    let question = Question::input("api_key")
        .message("Input your new API key")
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let answer = result.as_string().unwrap();
            // TODO: use result
            ApiKey::from_string(String::from(answer)).map_err(|e| SorryCouldNotRetrieveApiKey {
                error_message: format!("{}", e),
            })
        }
        Err(_) => Err(SorryCouldNotRetrieveApiKey {
            error_message: String::from("Could not retrieve API key"),
        }),
    }
}
