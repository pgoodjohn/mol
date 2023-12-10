//! Organizations API module
//!
//! Used to retrieve information about current organization (/me), other organizations
//! by id and partner information.
use crate::{
    models::{organization::Organization, partner::Partner},
    ApiClient,
};

/// [Organizations Api](https://docs.mollie.com/reference/v2/organizations-api/overview)
/// Used to retrieve information about an organization.
pub struct OrganizationsApi<'client> {
    api_client: &'client ApiClient<'client>,
}

impl<'client> OrganizationsApi<'client> {
    /// Create a new `OrganizationsApi`.
    pub fn new(api_client: &'client ApiClient) -> Self {
        Self { api_client }
    }

    /// [Get organization by id](https://docs.mollie.com/reference/v2/organizations-api/get-organization)
    pub async fn get_by_id(&self, id: &str) -> crate::Result<Organization> {
        let endpoint = format!("/organizations/{}", id);
        self.api_client.get(&endpoint, None).await
    }

    /// [Retrieve current organization](https://docs.mollie.com/reference/v2/organizations-api/current-organization)
    pub async fn me(&self) -> crate::Result<Organization> {
        self.api_client.get("/organizations/me", None).await
    }

    /// [Get partner](https://docs.mollie.com/reference/v2/organizations-api/get-partner)
    pub async fn get_partner(&self) -> crate::Result<Partner> {
        self.api_client.get("/organizations/me/partner", None).await
    }
}
