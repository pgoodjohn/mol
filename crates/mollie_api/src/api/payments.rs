//! Payments API module
//!
//! Used to retrieve information about current payments//! Payments API module
//!
//! Used to retrieve information about 
use crate::{
    models::{payment::{PaymentResource, PaymentsListResource, CreatePaymentRequest}},
    ApiClient,
};

/// [Organizations Api](https://docs.mollie.com/reference/v2/organizations-api/overview)
/// Used to retrieve information about an organization.
pub struct PaymentsApi<'client> {
    api_client: &'client ApiClient<'client>,
}

impl<'client> PaymentsApi<'client> {
    /// Create a new `OrganizationsApi`.
    pub fn new(api_client: &'client ApiClient) -> Self {
        Self { api_client }
    }

    /// [Get organization by id](https://docs.mollie.com/reference/v2/payments-api/get-payment)
    pub async fn get_by_id(&self, id: &str) -> crate::Result<PaymentResource> {
        let endpoint = format!("/payments/{}", id);
        self.api_client.get(&endpoint, None).await
    }

    // [List Payments](https://docs.mollie.com/reference/v2/payments-api/list-payments)
    pub async fn list(&self, limit: &Option<i32>, from: &Option<String>, profile_id: &Option<String>, test_mode: &Option<bool>) -> crate::Result<PaymentsListResource> {
        let endpoint = "/payments";
        let mut params = std::collections::HashMap::new();
        if let Some(l) = limit {
            params.insert("limit", l.to_string());
        }

        if let Some(f) = from {
            params.insert("from", f.to_string());
        }

        if let Some(p) = profile_id {
            params.insert("profileId", p.to_string());
        }

        if let Some(t) = test_mode {
            params.insert("testmode", t.to_string());
        }

        self.api_client.get(&endpoint, Some(params)).await
    }

    /// [Create Payment](https://docs.mollie.com/reference/v2/payments-api/create-payment)
    pub async fn create_payment(&self, body: &CreatePaymentRequest) -> crate::Result<PaymentResource> {
        let endpoint = "/payments";
        
        self.api_client.post(&endpoint, body).await
    }
}
