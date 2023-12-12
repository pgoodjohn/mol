use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApiKeyMode {
    /// Live API. To be used when receiving real payments.
    Live,

    /// Test API. For testing purposes only.
    Test,
}

impl<'a> TryFrom<&'a str> for ApiKeyMode {
    type Error = Error;

    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        if value.starts_with("test_") {
            return Ok(ApiKeyMode::Test);
        }

        if value.starts_with("live_") {
            return Ok(ApiKeyMode::Live);
        }

        Err(Error::InvalidApiKeyMode)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "String", into = "String")]
pub struct ApiKey {
    /// The API key value.
    pub value: String,

    /// API key mode. Can be either `Live` or `Test`.
    pub mode: ApiKeyMode,
}

impl TryFrom<String> for ApiKey {
    type Error = Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        let mode = ApiKeyMode::try_from(value.as_str())?;

        if value.len() != 35 {
            return Err(Error::InvalidApiKey);
        }

        Ok(ApiKey { mode, value })
    }
}

impl Into<String> for ApiKey {
    fn into(self) -> String {
        self.value
    }
}

impl Display for ApiKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
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
        assert_eq!(key.value, "test_xxxxxxxxxxxxxxxxxxxxxxxxxxx123");
    }

    #[test]
    fn should_parse_live_api_key() {
        let result = ApiKey::try_from("live_xxxxxxxxxxxxxxxxxxxxxxxxxxx123".to_string());
        assert!(result.is_ok(), "Should parse live API keys successfully");

        let key = result.unwrap();
        assert_eq!(key.mode, ApiKeyMode::Live);
        assert_eq!(key.value, "live_xxxxxxxxxxxxxxxxxxxxxxxxxxx123");
    }

    #[test]
    fn should_fail_to_parse_invalid_api_key_mode() {
        let result = ApiKey::try_from("invalid_xxxxxxxxxxxxxxxxxxxxxxxxxxx123".to_string());
        assert!(
            matches!(result, Err(Error::InvalidApiKeyMode)),
            "Should fail to parse invalid API key mode"
        );
    }

    #[test]
    fn should_fail_to_parse_invalid_length_api_key() {
        let result = ApiKey::try_from("test_xxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string());
        assert!(
            matches!(result, Err(Error::InvalidApiKey)),
            "Should fail to parse invalid length API key"
        );
    }
}
