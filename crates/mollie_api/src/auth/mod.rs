pub use self::access_code::*;
pub use self::api_key::*;
pub use self::connect::*;

mod access_code;
mod api_key;
mod connect;

#[derive(Debug, Clone)]
pub enum ApiBearerToken {
    ApiKey(ApiKey),
    AccessCode(AccessCode),
    ConnectToken(ConnectToken),
}

impl ApiBearerToken {
    pub fn get_token(&self) -> &str {
        match self {
            ApiBearerToken::ApiKey(key) => key.value.as_str(),
            ApiBearerToken::AccessCode(code) => code.value.as_str(),
            ApiBearerToken::ConnectToken(code) => code.value.as_str(),
        }
    }

    pub fn is_api_key(&self) -> bool {
        matches!(self, ApiBearerToken::ApiKey(_))
    }

    pub fn is_access_code(&self) -> bool {
        matches!(self, ApiBearerToken::AccessCode(_))
    }

    pub fn is_connect_token(&self) -> bool {
        matches!(self, ApiBearerToken::ConnectToken(_))
    }
}

pub trait AuthProvider {
    fn get_auth_token(&mut self) -> ApiBearerToken;
}
