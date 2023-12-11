use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::link::Link;

/// Model used to represent [error responses from the Mollie API](https://docs.mollie.com/overview/handling-errors#id2).
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    /// [Http status code](https://docs.mollie.com/overview/handling-errors#all-possible-status-codes)
    pub status: u16,

    /// Error title
    pub title: String,

    /// More detailed error message
    pub detail: String,

    /// List of links to relevant documentation
    #[serde(rename(deserialize = "_links"))]
    pub links: HashMap<String, Link>,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self).unwrap_or("Response error".to_string())
        )
    }
}
