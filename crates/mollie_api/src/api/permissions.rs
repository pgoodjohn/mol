//! Permissions API module
//!
//! Used to retrieve information about current access token permissions
use crate::{
    models::permission::PermissionsListResource,
    ApiClient,
};

/// [Permissions Api](https://docs.mollie.com/reference/v2/permissions-api/overview)
/// Used to retrieve information about an organization.
pub struct PermissionsApi<'client> {
    api_client: &'client ApiClient<'client>,
}

impl<'client> PermissionsApi<'client> {
    /// Create a new `PermissionsApi`.
    pub fn new(api_client: &'client ApiClient) -> Self {
        Self { api_client }
    }

    /// [List permissions](https://docs.mollie.com/reference/v2/permissions-api/list-permissions)
    pub async fn list(&self) -> crate::Result<PermissionsListResource> {
        self.api_client.get("/permissions", None).await
    }
}