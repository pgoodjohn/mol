use log::debug;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::auth::{AccessCode, ApiBearerToken, ApiKey, ApiTokenType};
use crate::error::{ConfigError, ConfigResult};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct MollieConfig {
    #[serde(default = "default_api_config")]
    pub api: ApiConfig,

    pub auth: Option<AuthConfig>,
}

impl MollieConfig {
    pub fn get_bearer_token(&self) -> ConfigResult<ApiBearerToken> {
        let Some(auth) = &self.auth else {
            return Err(ConfigError::NoAuthenticationMethodSet);
        };

        if let Some(access_code) = &auth.access_code {
            return Ok(ApiBearerToken::access_code(&access_code.token.0));
        };

        debug!("No access code found in config, trying API keys");

        if let Some(ApiKeysConfig {
            live: Some(live_api_key),
            ..
        }) = &auth.api_keys
        {
            return Ok(ApiBearerToken::api_key(live_api_key.key.as_ref()));
        };

        Err(ConfigError::NoAuthenticationMethodSet)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ApiConfig {
    #[serde(default = "default_api_url")]
    pub url: Url,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AuthConfig {
    pub api_keys: Option<ApiKeysConfig>,
    pub access_code: Option<AccessCodeConfig>,
    pub connect: Option<ConnectConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AccessCodeConfig {
    pub token: AccessCode,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ApiKeysConfig {
    pub live: Option<ApiKey>,
    pub test: Option<ApiKey>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ConnectConfig {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: Option<String>,
    pub access_token: Option<String>,
}

fn default_api_config() -> ApiConfig {
    ApiConfig {
        url: default_api_url(),
    }
}

fn default_api_url() -> Url {
    Url::parse("https://api.mollie.com/v2").unwrap()
}
