use crate::error::Error;

use crate::Result;

#[derive(Debug)]
pub struct ApiKey {
    pub value: String,
    pub mode: ApiKeyMode,
}

impl ApiKey {
    pub fn from_string(value: impl Into<String>) -> Result<Self> {
        let key: String = value.into();
        let mode = ApiKeyMode::from_string(&key)?;
        Ok(Self { value: key, mode })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ApiKeyMode {
    Live,
    Test,
}

impl ApiKeyMode {
    pub fn from_string(value: impl Into<String>) -> Result<ApiKeyMode> {
        let key: &str = &value.into();

        if key.len() != 35 {
            return Err(Error::InvalidApiKey);
        }

        match key {
            _ if key.starts_with("live_") => Ok(ApiKeyMode::Live),
            _ if key.starts_with("test_") => Ok(ApiKeyMode::Test),
            _ => Err(Error::InvalidApiKey),
        }
    }
}

pub struct AccessCode {
    pub value: String,
}

impl AccessCode {
    pub fn from_string(value: impl Into<String>) -> Result<Self> {
        let key: String = value.into();
        if key.starts_with("access_") && key.len() != 47 {
            return Err(Error::InvalidAccessCode);
        }
        Ok(AccessCode { value: key })
    }
}
