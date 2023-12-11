use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use super::{amount::Amount, link::Link};

/// Organization model; More information:
/// - <https://docs.mollie.com/reference/v2/organizations-api/get-organization#response>
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    /// Unique identifier of the organization
    pub id: String,
    pub mode: String,
    pub created_at: String,
    pub status: String,
    pub is_cancelable: Option<bool>,
    pub authorized_at: Option<String>,
    pub paid_at: Option<String>,
    pub expires_at: Option<String>,
    pub expired_at: Option<String>,
    pub failed_at: Option<String>,
    pub amount: Amount,
    pub amount_refunded: Option<Amount>,
    pub amount_remaining: Option<Amount>,
    pub amount_captured: Option<Amount>,
    pub amount_chargedback: Option<Amount>,
    pub settlement_amount: Option<Amount>,
    pub description: String,
    pub redirect_url: String,
    pub webhook_url: Option<String>,
    pub locale: Option<String>,
    pub country_code: Option<String>,
    pub method: Option<String>,
    pub restrict_payment_methods_to_country: Option<String>,
    pub profile_id: String,
    pub settlement_id: Option<String>,
    #[serde(rename = "_links")]
    pub links: HashMap<String, Link>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentsListResponse {
    pub count: i32,
    #[serde(rename = "_embedded")]
    pub embedded: PaymentsList,
    #[serde(rename = "_links")]
    pub links: HashMap<String, Option<Link>>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentsList {
    pub payments: Vec<Payment>
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentCreateRequest {
    pub amount: Amount,
    pub description: String,
    pub redirect_url: String,
    pub profile_id: Option<String>, 
}
