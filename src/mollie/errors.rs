#[derive(Debug)]
pub enum ApiClientError {
    CouldNotFindValidAuthorizationMethodToPerformRequest(),
    CouldNotPerformRequest(reqwest::Error),
    CouldNotUnderstandResponse(reqwest::Error),
    MollieApiReturnedAnError(super::MollieApiError),
}