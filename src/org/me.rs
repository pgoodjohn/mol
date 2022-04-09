use super::console;
use super::mollie;
use super::mollie::organizations::OrganizationsApi;
use log::{debug, info, warn};

pub fn command() {
    let client = mollie::ApiClient::new();

    let response = client.get_current_organization();

    match response {
        Ok(success) => {
            info!(
                "Successfully authenticated as Organization {} ({})",
                success.name, success.id
            );
        }
        Err(e) => console::handle_mollie_client_error(e),
    }
}
