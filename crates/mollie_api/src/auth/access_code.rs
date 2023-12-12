use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "String", into = "String")]
pub struct AccessCode {
    pub value: String,
}

impl TryFrom<String> for AccessCode {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Error> {
        if !value.starts_with("access_") || value.len() != 47 {
            return Err(Error::InvalidAccessToken);
        }

        Ok(AccessCode { value })
    }
}

impl Into<String> for AccessCode {
    fn into(self) -> String {
        self.value
    }
}

impl Display for AccessCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn should_parse_access_code() {
        let result =
            AccessCode::try_from("access_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx123".to_string());
        assert!(result.is_ok(), "Should parse access code successfully");

        let code = result.unwrap();
        assert_eq!(
            code.value,
            "access_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx123"
        );
    }

    #[test]
    fn should_fail_to_parse_access_code_with_invalid_prefix() {
        let result =
            AccessCode::try_from("invali_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx123".to_string());
        assert!(
            matches!(result, Err(Error::InvalidAccessToken)),
            "Should fail to parse invalid access code"
        );
    }

    #[test]
    fn should_fail_to_parse_access_code_with_invalid_length() {
        let result =
            AccessCode::try_from("access_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx123".to_string());
        assert!(
            matches!(result, Err(Error::InvalidAccessToken)),
            "Should fail to parse invalid access code"
        );
    }
}
