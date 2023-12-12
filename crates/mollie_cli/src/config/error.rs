use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("No authentication method set")]
    NoAuthenticationMethodSet,

    #[error("Could not serialize configuration")]
    CouldNotSerializeConfig(#[from] toml::ser::Error),

    #[error("Could not save configuration")]
    CouldNotSaveConfig(#[from] std::io::Error),
}

pub type ConfigResult<T> = Result<T, ConfigError>;
