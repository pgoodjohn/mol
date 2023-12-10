use log::debug;
use serde::{Deserialize, Serialize};
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;
extern crate dirs;
use super::mollie;

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
pub enum CouldNotRetrieveConfig {
    UnableToReadFile(io::Error),
    UnableToParseFile(toml::de::Error),
    NoAccessCodeSet(),
    NoLiveApiKeySet(),
    NoTestApiKeySet(),
}

impl fmt::Display for CouldNotRetrieveConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CouldNotRetrieveConfig::UnableToReadFile(ref err) => write!(f, "IO error: {}", err),
            CouldNotRetrieveConfig::UnableToParseFile(ref err) => write!(f, "TOML parse error: {}", err),
            CouldNotRetrieveConfig::NoAccessCodeSet() => write!(f, "No Access Code set. Run 'mol auth add --access-code {{access-code}} or 'mol auth add -i' to configure one."),
            CouldNotRetrieveConfig::NoLiveApiKeySet() => write!(f, "No Api Key set. Run 'mol auth add --api-key api-key or 'mol auth add -i' to set one up."),
            CouldNotRetrieveConfig::NoTestApiKeySet() => write!(f, "No testmode Api Key set. Run 'mol auth add --api-key api-key or 'mol auth add -i' to set one up."),
        }
    }
}

impl error::Error for CouldNotRetrieveConfig {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            CouldNotRetrieveConfig::UnableToParseFile(ref err) => Some(err),
            CouldNotRetrieveConfig::UnableToReadFile(ref err) => Some(err),
            CouldNotRetrieveConfig::NoAccessCodeSet() => None,
            CouldNotRetrieveConfig::NoLiveApiKeySet() => None,
            CouldNotRetrieveConfig::NoTestApiKeySet() => None,
        }
    }
}

pub fn from_file() -> Result<Config, CouldNotRetrieveConfig> {
    let contents =
        fs::read_to_string(config_path()).map_err(CouldNotRetrieveConfig::UnableToReadFile)?;
    debug!("Config text loaded:\n\n{}", contents);

    let config: Config =
        toml::from_str(&contents).map_err(CouldNotRetrieveConfig::UnableToParseFile)?;

    debug!("Loaded config: {:?}", config);

    Ok(config)
}

pub fn api_key() -> Result<String, CouldNotRetrieveConfig> {
    let config = from_file()?;

    match config.keys.live {
        Some(key) => Ok(key),
        None => Err(CouldNotRetrieveConfig::NoLiveApiKeySet()),
    }
}

pub fn api_key_test() -> Result<String, CouldNotRetrieveConfig> {
    let config = from_file()?;

    match config.keys.test {
        Some(key) => Ok(key),
        None => Err(CouldNotRetrieveConfig::NoTestApiKeySet()),
    }
}

pub fn access_code() -> Result<String, CouldNotRetrieveConfig> {
    let config = from_file()?;

    match config.access_code {
        Some(key) => Ok(key),
        None => Err(CouldNotRetrieveConfig::NoAccessCodeSet()),
    }
}

pub fn api_url() -> Result<String, CouldNotRetrieveConfig> {
    let config = from_file()?;

    Ok(config.api_url)
}

#[derive(Debug)]
pub enum CouldNotSaveConfig {
    UnableToWriteFile(io::Error),
    UnableToSerializeConfig(toml::ser::Error),
}

impl fmt::Display for CouldNotSaveConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CouldNotSaveConfig::UnableToWriteFile(ref err) => write!(f, "IO error: {}", err),
            CouldNotSaveConfig::UnableToSerializeConfig(ref err) => {
                write!(f, "TOML parse error: {}", err)
            }
        }
    }
}

impl error::Error for CouldNotSaveConfig {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            CouldNotSaveConfig::UnableToWriteFile(ref err) => Some(err),
            CouldNotSaveConfig::UnableToSerializeConfig(ref err) => Some(err),
        }
    }
}

pub fn save_to_file(config: Config) -> Result<(), CouldNotSaveConfig> {
    let new_config_str =
        toml::to_string(&config).map_err(CouldNotSaveConfig::UnableToSerializeConfig)?;

    fs::write(config_path(), new_config_str).map_err(CouldNotSaveConfig::UnableToWriteFile)?;

    Ok(())
}

fn config_path() -> PathBuf {
    let mut config_path = PathBuf::new();

    if cfg!(debug_assertions) {
        config_path.push("/tmp/.mol/conf.toml");
    } else {
        config_path.push(dirs::home_dir().unwrap());
        config_path.push(".mol/conf.toml");
    }

    config_path
}

pub fn get_bearer_token() -> Result<mollie::ApiBearerToken, Box<dyn std::error::Error>> {
    match access_code() {
        Ok(access_code) => {
            return Ok(mollie::ApiBearerToken {
                value: access_code.to_string(),
                token_type: mollie::ApiTokenTypes::AccessCode,
            });
        }
        Err(_) => {
            debug!("No access code set, trying to see if an API key is set instead")
        }
    }

    match api_key() {
        Ok(live_api_key) => Ok(mollie::ApiBearerToken {
            value: live_api_key.to_string(),
            token_type: mollie::ApiTokenTypes::ApiKey,
        }),
        Err(_) => {
            // TODO: Handle this error better - probably check it also before doing all the prompts
            panic!("No auth set!!!")
        }
    }
}
