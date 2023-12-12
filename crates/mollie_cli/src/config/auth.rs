use chrono::{Utc, Duration};
use oauth2::{basic::BasicClient, ClientId, AuthUrl, TokenUrl, reqwest::http_client, RefreshToken, TokenResponse, ClientSecret};

use crate::config::ConnectConfig;

pub(crate) fn create_client(config: &ConnectConfig) -> BasicClient {
    BasicClient::new(
        ClientId::new(config.client_id.clone()),
        Some(ClientSecret::new(config.client_secret.clone())),
        AuthUrl::new("https://my.mollie.com/oauth2/authorize".into()).unwrap(),
        Some(TokenUrl::new("https://api.mollie.com/oauth2/tokens".into()).unwrap())
    )
}

pub(crate) fn refresh_token(config: &mut ConnectConfig) {
    if config.refresh_token.is_none() {
        config.expires_at = Some(Utc::now() + Duration::hours(2));
        //panic!("can't refresh token without a refresh token");
        return;
    }
    let refresh_token = config.refresh_token.as_ref().unwrap().to_string();
    let client = create_client(config);
    match client.exchange_refresh_token(&RefreshToken::new(refresh_token)).request(http_client) {
        Ok(res) => {
            config.access_token = Some(res.access_token().secret().to_string());
            config.refresh_token = match res.refresh_token() {
                Some(token) => Some(token.secret().to_string()),
                _ => None
            };
            config.expires_at = match res.expires_in() {
                Some(duration) => Some(Utc::now() + duration),
                _ => None
            };
        },
        _ => panic!("failed to refresh token")
    }
}
