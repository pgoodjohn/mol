use crate::models::amount::Amount;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BalancesListResource {
    pub count: i32,
    #[serde(rename(deserialize = "_embedded", serialize = "_embedded"))]
    pub embedded: EmbeddedBalanceResource,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmbeddedBalanceResource {
    pub balances: Vec<BalanceResource>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceResource {
    pub id: String,
    pub mode: String,
    pub created_at: String,
    pub currency: String,
    pub status: String,
    pub available_amount: Amount,
    pub pending_amount: Amount,
    pub transfer_frequency: String,
    pub transfer_threshold: Amount,
    pub transfer_reference: Option<String>,
    pub transfer_destination: TransferDestination,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferDestination {
    #[serde(rename = "type")]
    pub destination_type: String,
    pub beneficiary_name: String,
    pub bank_account: String,
}
