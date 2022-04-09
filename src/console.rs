use super::mollie::*;
use log::{debug, warn};

pub fn handle_mollie_client_error(err: errors::ApiClientError) {
    debug!("{:?}", err);
    warn!("🚨 There was an error communicating with the Mollie API 🚨");

    match err {
        errors::ApiClientError::CouldNotPerformRequest(i) => {
            warn!("Could not reach the Mollie API to perform the request.");
            debug!("{:?}", i);
        }
        errors::ApiClientError::CouldNotUnderstandResponse(i) => {
            warn!("Request failed catastrophically and could not understand the Mollie API error response.");
            debug!("{:?}", i);
        }
        errors::ApiClientError::MollieApiReturnedAnError(i) => {
            warn!("Request to the Mollie API Failed: {}", i.detail);

            if i.status == 401 {
                warn!("Run mol auth to set up your authentication with the Mollie API");
                warn!("Run mol env url {{prod|dev}} to switch environments");
            }
        }
        errors::ApiClientError::CouldNotFindValidAuthorizationMethodToPerformRequest() => {
            warn!("Could not find a valid authorization method to perform this request");
            warn!("Run mol auth to set up your authentication with the Mollie API");
        }
    }
}

#[cfg(test)]
mod console_tests {
    use crate::mollie::errors;
    extern crate testing_logger;
    use log::Level;

    #[test]
    fn test_it_shows_authentication_command_if_error_is_unauthenticated() {
        testing_logger::setup();

        let error =
            errors::ApiClientError::MollieApiReturnedAnError(crate::mollie::MollieApiError {
                status: 401,
                title: String::from("Unauthorized Request"),
                detail: String::from("Missing authentication, or failed to authenticate"),
            });

        super::handle_mollie_client_error(error);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 5);
            assert_eq!(captured_logs[0].level, Level::Debug);
            assert_eq!(
                captured_logs[1].body,
                "🚨 There was an error communicating with the Mollie API 🚨"
            );
            assert_eq!(captured_logs[1].level, Level::Warn);
            assert_eq!(
                captured_logs[2].body,
                "Request to the Mollie API Failed: Missing authentication, or failed to authenticate"
            );
            assert_eq!(captured_logs[2].level, Level::Warn);
            assert_eq!(
                captured_logs[3].body,
                "Run mol auth to set up your authentication with the Mollie API"
            );
            assert_eq!(captured_logs[3].level, Level::Warn);
            assert_eq!(
                captured_logs[4].body,
                "Run mol env url {prod|dev} to switch environments"
            );
            assert_eq!(captured_logs[4].level, Level::Warn);
        });
    }

    #[test]
    fn test_it_shows_authentication_command_if_error_is_no_valid_authentication() {
        testing_logger::setup();

        let error = errors::ApiClientError::CouldNotFindValidAuthorizationMethodToPerformRequest();

        super::handle_mollie_client_error(error);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 4);
            assert_eq!(captured_logs[0].level, Level::Debug);
            assert_eq!(
                captured_logs[1].body,
                "🚨 There was an error communicating with the Mollie API 🚨"
            );
            assert_eq!(captured_logs[1].level, Level::Warn);
            assert_eq!(
                captured_logs[2].body,
                "Could not find a valid authorization method to perform this request"
            );
            assert_eq!(captured_logs[2].level, Level::Warn);
            assert_eq!(
                captured_logs[3].body,
                "Run mol auth to set up your authentication with the Mollie API"
            );
            assert_eq!(captured_logs[3].level, Level::Warn);
        });
    }
}
