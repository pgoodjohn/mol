use super::mollie;
use super::mollie::organizations::OrganizationsApi;
use log::{debug, info, warn};

pub fn command() {

    let client = mollie::ApiClient::new();

    let response = client.get_current_organization();

    match response {
        Ok(success) => {
            info!("Successfully authenticated as Organization {} ({})", success.name, success.id);
        }
        Err(e) => {
            handle_error(e)
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