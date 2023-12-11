use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use super::{amount::Amount, link::Link};

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundPaymentRequest {
    pub amount: Amount,
    pub description: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Refund {
    pub id: String,
    pub amount: Amount,
    pub status: String,
    pub created_at: String,
    pub description: String,
    pub payment_id: String,
    #[serde(rename = "_links")]
    pub links: HashMap<String, Link>,
}