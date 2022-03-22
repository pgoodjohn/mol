use log::{debug, warn, info};
use requestty::Question;
use serde::{Serialize, Deserialize};
use toml;
use std::fs;

pub fn command() -> Result<(), &'static str> {
    debug!("Running Store API key");

    let new_api_key = ask_api_key().unwrap();

    debug!("New API key will be {}", &new_api_key.value);

    replace_api_key(new_api_key);

    Ok(())
}

fn replace_api_key(new_api_key: ApiKey) {

    let old_config = load_config_from_file().unwrap();

    match old_config.keys.live {
        Some(_) => {
            info!("Replacing previously stored key");
        },
        None => {
            info!("Storing new key");
        }
    }

    let mut new_config = old_config.clone();
    new_config.keys.live = Some(new_api_key.value);

    debug!("Old config: {:?}", old_config);
    debug!("New config: {:?}", new_config);
   
    store_config_to_file(new_config).unwrap();
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Config {
    keys: Keys,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Keys {
    live: Option<String>,
    test: Option<String>,
}

#[derive(Debug)]
pub struct CouldNotRetrieveConfig {}

fn load_config_from_file() -> Result<Config, CouldNotRetrieveConfig> {

    // TODO: This probably shouldn't be hardcoded to my user
    let config_path = "/Users/pietro/.mol/conf.toml";

    let contents = fs::read_to_string(config_path)
        .expect("Something went wrong reading the file");
    debug!("Config text loaded:\n\n{}", contents);

    let config: Config = toml::from_str(&contents).unwrap();

    debug!("Loaded config: {:?}", config);

    Ok(config)
}

#[derive(Debug)]
struct CouldNotSaveConfig {}

fn store_config_to_file(config: Config) -> Result<(), CouldNotSaveConfig> {
    // TODO: This probably shouldn't be hardcoded to my user
    let config_path = "/Users/pietro/.mol/conf.toml";

    fs::write(config_path, toml::to_string(&config).unwrap());
    
    Ok(())
}

pub struct ApiKey {
    value: String
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
        Err(_) => Err(SorryCouldNotRetrieveApiKey{}),
    }
}
