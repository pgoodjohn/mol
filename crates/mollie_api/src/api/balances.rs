//! Balances API module
//!
//! Used to retrieve information about your balances.
use crate::models::balance::{BalanceResource, BalancesListResource};
use crate::ApiClient;
use std::collections::HashMap;

/// [Balances Api](https://docs.mollie.com/reference/v2/balances-api/overview)
/// Used to retrieve information about a balance.
pub struct BalancesApi<'client> {
    api_client: &'client ApiClient<'client>,
}

impl<'client> BalancesApi<'client> {
    /// Create a new `BalancesApi`.
    pub fn new(api_client: &'client ApiClient) -> Self {
        Self { api_client }
    }

    pub async fn get_by_id(&self, balance_id: &String) -> crate::Result<BalanceResource> {
        self.api_client
            .get(&format!("/balances/{}", balance_id), None)
            .await
    }

    pub async fn list(
        &self,
        limit: Option<i32>,
        from: &Option<String>,
    ) -> crate::Result<BalancesListResource> {
        let mut query_params: HashMap<&str, String> = HashMap::new();

        if let Some(l) = limit {
            query_params.insert("limit", l.to_string());
        }

        if let Some(f) = from {
            query_params.insert("from", f.to_string());
        }

        self.api_client.get("/balances", Some(query_params)).await
    }
}
