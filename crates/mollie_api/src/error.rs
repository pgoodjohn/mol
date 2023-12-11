#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid api key")]
    InvalidApiKey,

    #[error("Invalid api key mode (should be either live or test)")]
    InvalidApiKeyMode,

    #[error("Invalid access token")]
    InvalidAccessToken,

    #[error("Error while performing request: {0}")]
    CouldNotPerformRequest(#[from] reqwest::Error),

    #[error("Mollie API Error {status}: {title} - {detail}. {raw_response}")]
    ApiError {
        status: u16,
        title: String,
        detail: String,
        raw_response: String,
    },
}
