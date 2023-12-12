
use mollie_api::auth::{AuthProvider, ApiBearerToken, AccessCode};
use oauth2::{basic::BasicClient, ClientId, AuthUrl, TokenUrl, reqwest::http_client, RefreshToken, TokenResponse};

use crate::config::{MollieConfig, ConfigurationService, ConnectConfig};

pub struct CliAuthProvider<'config> {
    config: &'config mut dyn ConfigurationService
}

impl<'config> CliAuthProvider<'config> {
    pub fn new(config: &'config mut dyn ConfigurationService) -> Self {
        Self {
            config,
        }
    }

    fn get_basic_client(&self, config: &MollieConfig) -> BasicClient {
        if config.auth.connect.is_none() {
            panic!("setup oauth before using client");
        }
        let oauth = config.auth.connect.clone().unwrap();
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

impl<'config> AuthProvider for CliAuthProvider<'config> {
    fn get_auth_token(&mut self) -> ApiBearerToken {
        let config = self.config.read();
        if let Some(connect) = &config.auth.connect {
            if let Some(existing_access_token) = &connect.access_token {
                let mut access_token = existing_access_token.clone();
                if let (Some(expires_at), Some(refresh_token)) = (&connect.expires_at, &connect.refresh_token) {
                    if expires_at <= &chrono::Utc::now() {
                        // refresh the token!
                        let client = self.get_basic_client(config);
                        match client.exchange_refresh_token(&RefreshToken::new(refresh_token.to_owned())).request(http_client) {
                            Ok(res) => {
                                access_token = res.access_token().secret().to_owned();
                                let refresh_token = res.refresh_token().unwrap().secret().to_owned();

                                let mut updated_connect = connect.clone();
                                updated_connect.access_token = Some(access_token.clone());
                                updated_connect.refresh_token = Some(refresh_token);
                                self.config.update(&|config| {
                                    config.auth.connect = Some(ConnectConfig{
                                        client_id: connect.client_id,
                                        client_secret: connect.client_secret,
                                        access_token: Some(access_token.clone()),
                                        refresh_token: Some(refresh_token),
                                        expires_at: None,
                                    });
                                });
                            },
                            Err(_) => panic!("failed to refresh token"),
                        }
                    }
                }
                return ApiBearerToken::AccessCode(AccessCode::try_from(access_token.to_string()).unwrap());
            }
        }
        if let Some(key) = config.live_api_key() {
            return ApiBearerToken::ApiKey(key.clone()); // todo, support test
        }
        panic!("no token available");
    }
}

