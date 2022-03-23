use log::debug;
use serde::{Deserialize, Serialize};
use std::fs;
use toml;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub api_url: String,
    pub access_code: Option<String>,
    pub keys: Keys,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Keys {
    pub live: Option<String>,
    pub test: Option<String>,
}

#[derive(Debug)]
pub struct CouldNotRetrieveConfig {}

pub fn from_file() -> Result<Config, CouldNotRetrieveConfig> {
    // TODO: This probably shouldn't be hardcoded to my user
    let config_path = "/Users/pietro/.mol/conf.toml";

    let contents = fs::read_to_string(config_path).expect("Something went wrong reading the file");
    debug!("Config text loaded:\n\n{}", contents);

    let config: Config = toml::from_str(&contents).unwrap();

    debug!("Loaded config: {:?}", config);

    Ok(config)
}

pub fn api_key() -> Result<String, CouldNotRetrieveConfig> {
    let config = from_file().unwrap();

    match config.keys.live {
        Some(key) => Ok(key),
        None => panic!("No API key set"), // TODO: Do proper error handling
    }
}

pub fn api_key_test() -> Result<String, CouldNotRetrieveConfig> {
    let config = from_file().unwrap();

    match config.keys.test {
        Some(key) => Ok(key),
        None => panic!("No test API key set"), // TODO: Do proper error handling
    }
}

pub fn access_code() -> Result<String, CouldNotRetrieveConfig> {
    let config = from_file().unwrap();

    match config.access_code {
        Some(key) => Ok(key),
        None => panic!("No API key set"), // TODO: Do proper error handling
    }
}

pub fn api_url() -> Result<String, CouldNotRetrieveConfig> {
    let config = from_file().unwrap();

    Ok(config.api_url)
}

#[derive(Debug)]
pub struct CouldNotSaveConfig {}

pub fn save_to_file(config: Config) -> Result<(), CouldNotSaveConfig> {
    // TODO: This probably shouldn't be hardcoded to my user
    let config_path = "/Users/pietro/.mol/conf.toml";

    fs::write(config_path, toml::to_string(&config).unwrap());

    Ok(())
}
