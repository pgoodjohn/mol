use super::console;
use super::mollie;
use super::mollie::permissions::Permissions;
use log::info;
use pad::{Alignment, PadStr};

pub fn command(filter_granted: &bool) {
    let client = mollie::ApiClientBuilder::new()
        .blocking()
        .url(super::config::api_url().unwrap())
        .auth(super::config::get_bearer_token().unwrap())
        .spawn();

    let response = client.get_permissions();

    match response {
        Ok(success) => {
            if *filter_granted {
                list_granted_permissions(success.embedded);
                return;
            }
            list_permissions(success.embedded);
        }
        Err(err) => console::handle_mollie_client_error(err),
    }
}

fn list_permissions(permissions: super::mollie::permissions::PermissionsResources) {
    for permission in permissions.permissions {
        info!(
            "{} | Granted: {} | {}",
            permission
                .id
                .pad_to_width_with_alignment(20, Alignment::Right),
            permission.granted as i32,
            permission.description
        );
    }
}

fn list_granted_permissions(permissions: super::mollie::permissions::PermissionsResources) {
    for permission in permissions.permissions {
        if permission.granted {
            info!(
                "{} | {}",
                permission
                    .id
                    .pad_to_width_with_alignment(20, Alignment::Right),
                permission.description
            );
        }
    }
}
