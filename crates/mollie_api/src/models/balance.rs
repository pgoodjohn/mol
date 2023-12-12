use std::fmt::Display;
use serde::Deserialize;
use crate::models::amount::Amount;

#[derive(Debug, Deserialize)]
pub struct BalancesListResource {
    pub count: i32,
    #[serde(rename(deserialize = "_embedded"))]
    pub embedded: EmbeddedBalanceResource,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddedBalanceResource {
    pub balances: Vec<BalanceResource>,
}

#[derive(Debug, Deserialize)]
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

impl Display for BalanceResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} | {} | {} | {}",
            self.status,
            self.id,
            self.available_amount.to_string(),
            self.pending_amount.to_string(),
            self.mode,
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferDestination {
    #[serde(rename = "type")]
    pub destination_type: String,
    pub beneficiary_name: String,
    pub bank_account: String,
}