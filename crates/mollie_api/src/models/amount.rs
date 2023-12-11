use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Amount{

    pub value: String,

    pub currency: String,
}
