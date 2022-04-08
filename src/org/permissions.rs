use super::mollie;
use log::{debug, info};
use reqwest::StatusCode;
use serde::Deserialize;

pub fn command() {
    get_permissions_from_api();
}

#[derive(Deserialize, Debug)]
struct PermissionsResponse {
    #[serde(rename(deserialize = "_embedded"))]
    embedded: PermissionsResources,
}

#[derive(Deserialize, Debug)]
struct PermissionsResources {
    permissions: Vec<PermissionResource>,
}

#[derive(Deserialize, Debug)]
struct PermissionResource {
    id: String,
    description: String,
    granted: bool,
}

fn get_permissions_from_api() {
    let client = mollie::ApiClient::new();
    let response = client.get(String::from("v2/permissions"), None).unwrap();

    // HTTP 200 Response means the request was successful
    if response.status() == StatusCode::OK {
        debug!("Successfull call to the Mollie API!");
        let decoded_response = response.json::<PermissionsResponse>().unwrap();
        debug!("{:?}", decoded_response);

        for permission in decoded_response.embedded.permissions {
            info!(
                "{} - {} - Granted: {}",
                permission.id, permission.description, permission.granted
            )
        }

        return;
    }

    // Any other response is an error
    mollie::handle_mollie_api_error(response);
}
