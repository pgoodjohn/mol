//! Refunds API module
//!
//! Used to refund payments
use crate::{
    models::refund::{RefundPaymentRequest, RefundResource},
    ApiClient,
};

/// [Refunds Api](https://docs.mollie.com/reference/v2/organizations-api/overview)
/// Used to retrieve information about an organization.
pub struct RefundsApi<'client> {
    api_client: &'client ApiClient<'client>,
}

impl<'client> RefundsApi<'client> {
    /// Create a new `OrganizationsApi`.
    pub fn new(api_client: &'client ApiClient) -> Self {
        Self { api_client }
    }

    pub async fn refund(
        &self,
        id: &str,
        body: &RefundPaymentRequest,
    ) -> crate::Result<RefundResource> {
        let endpoint = format!("/payments/{}/refunds", id);
        let x = self.api_client.post(&endpoint, body).await;
        log::debug!("{:?}", x);
        return x;
    }
}
