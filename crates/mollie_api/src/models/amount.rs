use std::fmt::Display;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct Amount{
    pub value: String,
    pub currency: String,
}

impl Display for Amount {
    fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{} {}", self.currency, self.value))
    }
}
