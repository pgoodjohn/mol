use super::mollie;
use super::mollie::permissions::Permissions;
use log::{debug, info, warn};
use pad::{Alignment, PadStr};

pub fn command(filter_granted: &bool) {
    let client = mollie::ApiClient::new();

    let response = client.get_permissions();

    match response {
        Ok(success) => {
            if *filter_granted {
                list_granted_permissions(success.embedded);
                return;
            }
            list_permissions(success.embedded);
        }
        Err(err) => handle_error(err),
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

fn handle_error(err: super::mollie::errors::ApiClientError) {
    debug!("{:?}", err);
    warn!("🚨 There was an error communicating with the Mollie API 🚨");

    match err {
        super::mollie::errors::ApiClientError::CouldNotPerformRequest(i) => {
            warn!("Could not reach the Mollie API to perform the request.");
            debug!("{:?}", i);
        },
        super::mollie::errors::ApiClientError::CouldNotUnderstandResponse(i) => {
            warn!("Request failed catastrophically and could not understand the Mollie API error response.");
            debug!("{:?}", i);
        },
        super::mollie::errors::ApiClientError::MollieApiReturnedAnError(i) => {
            warn!("Request to the Mollie API Failed: {}", i.detail);

            if i.status == 401 {
                warn!("Run mol auth to set up your authentication with the Mollie API");
                warn!("Run mol env url {{prod|dev}} to switch environments");
            }
        },
        super::mollie::errors::ApiClientError::CouldNotFindValidAuthorizationMethodToPerformRequest() => {
            warn!("Could not find an access token to authenticate this request");
            warn!("Run mol auth -i or mol auth add --access-code {{access-code}} to be able to perform this request");
        },
    }
}
