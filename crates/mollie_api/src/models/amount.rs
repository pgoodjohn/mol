use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Amount{

    pub value: String,

    pub currency: String,
}
