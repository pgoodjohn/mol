
use mollie_api::auth::{AuthProvider, ApiBearerToken, AccessCode};
use oauth2::{basic::BasicClient, ClientId, AuthUrl, TokenUrl, reqwest::http_client, RefreshToken, TokenResponse};

use crate::config::MollieConfig;

pub struct CliAuthProvider {
    config: Box<MollieConfig>
}

impl CliAuthProvider {
    pub fn new(config: &MollieConfig) -> Self {
        Self {
            config: Box::new(config.clone())
        }
    }

    fn get_basic_client(&self) -> BasicClient {
        if self.config.auth.connect.is_none() {
            panic!("setup oauth before using client");
        }
        let oauth = self.config.auth.connect.clone().unwrap();
        let base_url = "https://my.mollie.com/oauth2".to_string();
        let auth_url = AuthUrl::new(base_url.clone() + "/authorize").unwrap();
        let token_url = Some(TokenUrl::new(base_url.clone() + "/tokens").unwrap());
        BasicClient::new(
            ClientId::new(oauth.client_id),
            None,
            auth_url,
            token_url,
        )
    }

}

impl AuthProvider for CliAuthProvider {
    fn get_auth_token(&self) -> ApiBearerToken {
        if let Some(connect) = &self.config.auth.connect {
            if let Some(existing_access_token) = &connect.access_token {
                let mut access_token = existing_access_token.clone();
                if let (Some(expires_at), Some(refresh_token)) = (&connect.expires_at, &connect.refresh_token) {
                    if expires_at <= &chrono::Utc::now() {
                        // refresh the token!
                        let client = self.get_basic_client();
                        match client.exchange_refresh_token(&RefreshToken::new(refresh_token.to_owned())).request(http_client) {
                            Ok(res) => {
                                access_token = res.access_token().secret().to_owned();
                                // let refresh_token = res.refresh_token().unwrap().secret().to_owned();
                                // connect.access_token = Some(access_token.clone());
                                // connect.refresh_token = Some(refresh_token);
                            },
                            Err(_) => panic!("failed to refresh token"),
                        }
                    }
                }
                return ApiBearerToken::AccessCode(AccessCode::try_from(access_token.to_string()).unwrap());
            }
        }
        if let Some(key) = self.config.live_api_key() {
            return ApiBearerToken::ApiKey(key.clone()); // todo, support test
        }
        panic!("no token available");
    }
}

