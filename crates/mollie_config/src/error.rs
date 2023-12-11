use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("Invalid API key mode. API keys should either start with `live_` or `test_`")]
    InvalidApiKeyMode,

    #[error("Invalid access code")]
    InvalidAccessCode,

    #[error("No authentication method set")]
    NoAuthenticationMethodSet,

    #[error("Could not serialize configuration")]
    CouldNotSerializeConfig(#[from] toml::ser::Error),

    #[error("Could not save configuration")]
    CouldNotSaveConfig(#[from] std::io::Error),
}

pub type ConfigResult<T> = Result<T, ConfigError>;
