use std::collections::HashMap;
use std::fmt::Display;
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
pub struct RefundResource {
    pub id: String,
    pub amount: Amount,
    pub status: String,
    pub created_at: String,
    pub description: String,
    pub payment_id: String,
    #[serde(rename = "_links")]
    pub links: HashMap<String, Link>,
}

impl Display for RefundResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} | {} | {} | {} | {} |",
            self.id,
            self.status,
            self.amount,
            self.created_at,
            self.description,
            self.payment_id,
        )
    }
}

