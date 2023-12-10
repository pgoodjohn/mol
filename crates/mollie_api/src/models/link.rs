use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
    pub href: String,
    pub r#type: String,
}
