use crate::error::ConfigError;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiKeyMode {
    /// Test API. For testing purposes only.
    Test,

    /// Live API. To be used when receiving real payments.
    Live,
}

impl<'a> TryFrom<&'a str> for ApiKeyMode {
    type Error = ConfigError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value.starts_with("test_") {
            return Ok(ApiKeyMode::Test);
        }

        if value.starts_with("live_") {
            return Ok(ApiKeyMode::Live);
        }

        Err(ConfigError::InvalidApiKeyMode)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(try_from = "String", into = "String")]
pub struct ApiKey {
    pub mode: ApiKeyMode,
    pub key: String,
}

impl Display for ApiKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)
    }
}

impl TryFrom<String> for ApiKey {
    type Error = ConfigError;

    fn try_from(key: String) -> Result<Self, Self::Error> {
        let mode = ApiKeyMode::try_from(key.as_str())?;

        if key.len() != 35 {
            return Err(ConfigError::InvalidApiKey);
        }

        Ok(ApiKey { mode, key })
    }
}

impl Into<String> for ApiKey {
    fn into(self) -> String {
        self.key
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(try_from = "String", into = "String")]
pub struct AccessCode(pub String);

impl Display for AccessCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for AccessCode {
    type Error = ConfigError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !value.starts_with("access_") || value.len() != 47 {
            return Err(ConfigError::InvalidAccessCode);
        }

        Ok(AccessCode(value))
    }
}

impl Into<String> for AccessCode {
    fn into(self) -> String {
        self.0
    }
}

pub enum ApiTokenType {
    ApiKey,
    AccessCode,
}

pub struct ApiBearerToken<'config> {
    pub value: &'config str,
    pub token_type: ApiTokenType,
}

impl<'config> ApiBearerToken<'config> {
    pub fn api_key(key: &'config str) -> Self {
        Self {
            value: key,
            token_type: ApiTokenType::ApiKey,
        }
    }

    pub fn access_code(code: &'config str) -> Self {
        Self {
            value: code,
            token_type: ApiTokenType::AccessCode,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_test_api_key() {
        let result = ApiKey::try_from("test_xxxxxxxxxxxxxxxxxxxxxxxxxxx123".to_string());
        assert!(result.is_ok(), "Should parse test API keys successfully");

        let key = result.unwrap();
        assert_eq!(key.mode, ApiKeyMode::Test);
        assert_eq!(key.key, "test_xxxxxxxxxxxxxxxxxxxxxxxxxxx123");
    }

    #[test]
    fn should_parse_live_api_key() {
        let result = ApiKey::try_from("live_xxxxxxxxxxxxxxxxxxxxxxxxxxx123".to_string());
        assert!(result.is_ok(), "Should parse live API keys successfully");

        let key = result.unwrap();
        assert_eq!(key.mode, ApiKeyMode::Live);
        assert_eq!(key.key, "live_xxxxxxxxxxxxxxxxxxxxxxxxxxx123");
    }

    #[test]
    fn should_fail_to_parse_invalid_api_key_mode() {
        let result = ApiKey::try_from("invalid_xxxxxxxxxxxxxxxxxxxxxxxxxxx123".to_string());
        assert!(
            matches!(result, Err(ConfigError::InvalidApiKeyMode)),
            "Should fail to parse invalid API key mode"
        );
    }

    #[test]
    fn should_fail_to_parse_invalid_length_api_key() {
        let result = ApiKey::try_from("test_xxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string());
        assert!(
            matches!(result, Err(ConfigError::InvalidApiKey)),
            "Should fail to parse invalid length API key"
        );
    }

    #[test]
    fn should_parse_access_code() {
        let result =
            AccessCode::try_from("access_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx123".to_string());
        assert!(result.is_ok(), "Should parse access code successfully");

        let code = result.unwrap();
        assert_eq!(code.0, "access_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx123");
    }

    #[test]
    fn should_fail_to_parse_access_code_with_invalid_prefix() {
        let result =
            AccessCode::try_from("invali_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx123".to_string());
        assert!(
            matches!(result, Err(ConfigError::InvalidAccessCode)),
            "Should fail to parse invalid access code"
        );
    }

    #[test]
    fn should_fail_to_parse_access_code_with_invalid_length() {
        let result =
            AccessCode::try_from("access_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx123".to_string());
        assert!(
            matches!(result, Err(ConfigError::InvalidAccessCode)),
            "Should fail to parse invalid access code"
        );
    }
}
