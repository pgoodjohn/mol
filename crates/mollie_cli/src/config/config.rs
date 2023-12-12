use chrono::DateTime;
use log::debug;
use mollie_api::auth::{AccessCode, ApiBearerToken, ApiKey};
use serde::{Deserialize, Serialize};
use url::Url;

use super::error::{ConfigError, ConfigResult};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct MollieConfig {
    #[serde(default = "default_api_config")]
    pub api: ApiConfig,

    #[serde(default = "default_auth_config")]
    pub auth: AuthConfig,
}

impl MollieConfig {
    pub fn live_api_key(&self) -> Option<&ApiKey> {
        self.auth.api_keys.as_ref()?.live.as_ref()
    }

    pub fn test_api_key(&self) -> Option<&ApiKey> {
        self.auth.api_keys.as_ref()?.test.as_ref()
    }

    pub fn access_code(&self) -> Option<&AccessCode> {
        Some(&self.auth.access_code.as_ref()?.token)
    }

    pub fn bearer_token(&self) -> ConfigResult<ApiBearerToken> {
        if let Some(access_code) = &self.auth.access_code {
            return Ok(ApiBearerToken::AccessCode(access_code.token.clone()));
        };

        debug!("No access code found in config, trying API keys");

        if let Some(ApiKeysConfig {
            live: Some(live_api_key),
            ..
        }) = &self.auth.api_keys
        {
            return Ok(ApiBearerToken::ApiKey(live_api_key.clone()));
        };

        Err(ConfigError::NoAuthenticationMethodSet)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ApiConfig {
    #[serde(default = "default_api_url")]
    pub url: Url,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AuthConfig {
    pub api_keys: Option<ApiKeysConfig>,
    pub access_code: Option<AccessCodeConfig>,
    pub connect: Option<ConnectConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AccessCodeConfig {
    pub token: AccessCode,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ApiKeysConfig {
    pub live: Option<ApiKey>,
    pub test: Option<ApiKey>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ConnectConfig {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<DateTime<chrono::Utc>>,
    pub access_token: Option<String>,
}

fn default_api_config() -> ApiConfig {
    ApiConfig {
        url: default_api_url(),
    }
}

fn default_auth_config() -> AuthConfig {
    AuthConfig {
        api_keys: None,
        access_code: None,
        connect: None,
    }
}

fn default_api_url() -> Url {
    Url::parse("https://api.mollie.com/v2").unwrap()
}
