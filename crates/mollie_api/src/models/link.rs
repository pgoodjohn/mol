use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Link {
    pub href: String,
    pub r#type: String,
}
