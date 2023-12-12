use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum ConfigError {
    #[error("No authentication method set")]
    #[diagnostic(
        code("config::auth"),
        help("Please set an authentication method in your configuration file. You can use API keys, organization access tokens or Mollie Connect"),
        url("https://docs.mollie.com/overview/authentication")
    )]
    NoAuthenticationMethodSet,

    #[error("Could not serialize configuration")]
    #[diagnostic(
        code("config::serialize"),
        help(
            "Failed to serialize configuration. This should not happen, please file a bug report."
        )
    )]
    CouldNotSerializeConfig(#[from] toml::ser::Error),

    #[error("Could not save configuration")]
    #[diagnostic(
        code("config::save"),
        help(
            "Failed to save configuration to file, you might not have the correct permissions to access the file."
        )
    )]
    CouldNotSaveConfig(#[from] std::io::Error),
}

pub type ConfigResult<T> = Result<T, ConfigError>;
