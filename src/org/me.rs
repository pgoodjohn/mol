use super::console;
use super::mollie;
use super::mollie::organizations::OrganizationsApi;
use log::info;

pub fn command() {
    let client = mollie::ApiClientBuilder::new()
        .blocking()
        .url(super::config::api_url().unwrap())
        .auth(super::config::get_bearer_token().unwrap())
        .spawn();

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
