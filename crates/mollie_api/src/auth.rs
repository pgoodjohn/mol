use crate::error::Error;

use crate::Result;

#[derive(Debug)]
pub struct ApiKey {
    /// The API key value.
    pub value: String,

    /// API key mode. Can be either `Live` or `Test`.
    pub mode: ApiKeyMode,
}

impl ApiKey {
    /// Try to create an `ApiKey` from a string.
    pub fn from_string(value: impl Into<String>) -> Result<Self> {
        let key: String = value.into();

        if !Self::has_valid_length(&key) || !Self::has_valid_prefix(&key) {
            return Err(Error::InvalidApiKey);
        }

        let mode = ApiKeyMode::from_string(&key)?;

        Ok(Self { value: key, mode })
    }

    pub fn has_valid_prefix(value: &str) -> bool {
        value.starts_with("live_") || value.starts_with("test_")
    }

    /// Check if api key has a valid length (including prefix)
    pub fn has_valid_length(value: &str) -> bool {
        value.len() == 35
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ApiKeyMode {
    /// Live API. To be used when receiving real payments.
    Live,

    /// Test API. For testing purposes only.
    Test,
}

impl ApiKeyMode {
    /// Try to get the api key mode from a string.
    pub fn from_string(value: impl Into<String>) -> Result<ApiKeyMode> {
        let key: &str = &value.into();
        match key {
            _ if key.starts_with("live_") => Ok(ApiKeyMode::Live),
            _ if key.starts_with("test_") => Ok(ApiKeyMode::Test),
            _ => Err(Error::InvalidApiKeyMode),
        }
    }
}

pub struct AccessCode {
    pub value: String,
}

impl AccessCode {
    /// Try to create an `AccessCode` from a string.
    pub fn from_string(value: impl Into<String>) -> Result<Self> {
        let key: String = value.into();
        if !Self::has_valid_prefix(&key) || !Self::has_valid_length(&key) {
            return Err(Error::InvalidAccessCode);
        }
        Ok(AccessCode { value: key })
    }

    pub fn has_valid_prefix(value: &str) -> bool {
        value.starts_with("access_")
    }

    /// Check if access key has a valid length (including prefix)
    pub fn has_valid_length(value: &str) -> bool {
        value.len() == 47
    }
}

#[derive(Debug)]
pub struct OAuth {
    /// The API key value.
    pub refresh_token: String,

    pub access_token: Option<String>,
}

#[cfg(test)]
mod test {
    use crate::auth::AccessCode;

    use super::ApiKey;

    #[test]
    fn should_throw_error_if_invalid_api_key() {
        let key = ApiKey::from_string("inva_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
        assert!(key.is_err());

        let key = ApiKey::from_string("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxy");
        assert!(key.is_err());

        let key = ApiKey::from_string("live_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxy");
        assert!(key.is_err());
    }

    #[test]
    fn should_return_ok_if_valid_api_key() {
        let test_key = ApiKey::from_string("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
        assert!(test_key.is_ok());
        let test_key = test_key.unwrap();
        assert!(&test_key.mode.eq(&super::ApiKeyMode::Test));
        assert!(&test_key.value.eq("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"));

        let live_key = ApiKey::from_string("live_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
        assert!(live_key.is_ok());
        let live_key = live_key.unwrap();
        assert!(&live_key.mode.eq(&super::ApiKeyMode::Live));
        assert!(&live_key.value.eq("live_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"));
    }

    #[test]
    fn should_return_error_if_invalid_access_code() {
        let key = AccessCode::from_string("invali_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
        assert!(key.is_err());

        let key = AccessCode::from_string("access_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxyyyy");
        assert!(key.is_err());
    }

    #[test]
    fn should_return_ok_if_valud_access_code() {
        let key = AccessCode::from_string("access_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
        assert!(key.is_ok());
        assert!(key.unwrap().value == "access_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    }
}
