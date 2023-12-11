use crate::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "String", into = "String")]
pub struct ConnectToken {
    pub value: String,
}

impl TryFrom<String> for ConnectToken {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Error> {
        if !value.starts_with("access_") || value.len() != 47 {
            return Err(Error::InvalidAccessToken);
        }

        Ok(ConnectToken { value })
    }
}

impl Into<String> for ConnectToken {
    fn into(self) -> String {
        self.value
    }
}
