use miette::Diagnostic;

#[derive(thiserror::Error, Debug, Diagnostic)]
pub enum Error {
    #[error("Invalid api key")]
    #[diagnostic(
        code("api::auth::api_key"),
        help("Please check your api key and try again.")
    )]
    InvalidApiKey,

    #[error("Invalid api key mode (should be either live or test)")]
    #[diagnostic(
        code("api::auth::api_key::mode"),
        help("Your api key should either start with the 'live_' or the 'test_' prefix, which sets the mode of the key.")
    )]
    InvalidApiKeyMode,

    #[error("Invalid access token")]
    #[diagnostic(
        code("api::auth::access_token"),
        help("Access tokens should start with the 'access_' prefix and should be 47 characters long.")
    )]
    InvalidAccessToken,

    #[error("Error while performing request: {0}")]
    #[diagnostic(code("api::request"), help("Please try again later"))]
    CouldNotPerformRequest(#[from] reqwest::Error),

    #[error("Mollie API Error {status}: {title} - {detail}. {raw_response}")]
    #[diagnostic(code("api::mollie_error"), help("Please try again later"))]
    ApiError {
        status: u16,
        title: String,
        detail: String,
        raw_response: String,
    },
}
