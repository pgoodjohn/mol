use crate::config::error::ConfigResult;
use figment::value::{Uncased, UncasedStr};
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use log::debug;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

pub use crate::config::config::*;

mod config;
mod error;

pub trait ConfigurationService {
    fn read(&self) -> &MollieConfig;
    fn update(&mut self, updater: &dyn Fn(&mut MollieConfig)) -> ConfigResult<MollieConfig>;
}

pub struct FigmentConfigurationService {
    config: OnceLock<MollieConfig>,
}

impl FigmentConfigurationService {
    pub fn new() -> Self {
        Self {
            config: OnceLock::new(),
        }
    }

    fn config_path() -> PathBuf {
        let mut config_path = PathBuf::new();

        if cfg!(debug_assertions) {
            config_path.push("/tmp/.mol/conf.toml");
        } else {
            config_path.push(dirs::home_dir().unwrap());
            config_path.push(".mol/conf.toml");
        }

        config_path
    }

    fn map_env_variables(str: &UncasedStr) -> Uncased {
        match str {
            _ if str == "api_url" => "api.url".into(),
            _ if str == "api_key" => "auth.api_keys.live".into(),
            _ if str == "access_token" => "auth.access_code.token".into(),
            _ => str.as_str().replace("__", ".").into(),
        }
    }
}

impl ConfigurationService for FigmentConfigurationService {
    fn read(&self) -> &MollieConfig {
        self.config.get_or_init(|| {
            // Figment's test mode can only read config files from the current working directory.
            let figment = if cfg!(test) {
                Figment::new().merge(Toml::file("conf.toml"))
            } else {
                Figment::new().merge(Toml::file(Self::config_path()))
            };

            figment
                .merge(Env::prefixed("MOLLIE_").map(Self::map_env_variables))
                .extract::<MollieConfig>()
                .unwrap()
        })
    }

    fn update(&mut self, updater: &dyn Fn(&mut MollieConfig)) -> ConfigResult<MollieConfig> {
        let mut config = self.read().clone();
        updater(&mut config);

        let path = Self::config_path();
        let new_config = toml::to_string_pretty(&config)?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, new_config)?;
        self.config.take();

        debug!("Saved configuration file to {}", path.to_string_lossy());

        Ok(config)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mollie_api::auth;
    use url::Url;

    #[test]
    fn should_read_config() {
        figment::Jail::expect_with(|jail| {
            jail.clear_env();
            jail.create_file(
                "conf.toml",
                r#"
                    [api]
                    url = "https://test.com/"
                    
                    [auth.access_code]
                    token = "access_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx123"
                    
                    [auth.api_keys]
                    live = "live_xxxxxxxxxxxxxxxxxxxxxxxxxxx123"
                    test = "test_xxxxxxxxxxxxxxxxxxxxxxxxxxx456"
                    
                    [auth.connect]
                    client_id = "client_id"
                    client_secret = "client_secret"
                    refresh_token = "refresh_token"
                    access_token = "access_token"
                "#,
            )?;

            let service = FigmentConfigurationService::new();
            let config = service.read();

            assert_eq!(
                config,
                &MollieConfig {
                    api: ApiConfig {
                        url: Url::parse("https://test.com/").unwrap(),
                    },
                    auth: AuthConfig {
                        access_code: Some(AccessCodeConfig {
                            token: auth::AccessCode {
                                value: "access_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx123"
                                    .to_string()
                            },
                        }),
                        api_keys: Some(ApiKeysConfig {
                            live: Some(auth::ApiKey {
                                mode: auth::ApiKeyMode::Live,
                                value: "live_xxxxxxxxxxxxxxxxxxxxxxxxxxx123".to_string(),
                            }),
                            test: Some(auth::ApiKey {
                                mode: auth::ApiKeyMode::Test,
                                value: "test_xxxxxxxxxxxxxxxxxxxxxxxxxxx456".to_string(),
                            }),
                        }),
                        connect: Some(ConnectConfig {
                            client_id: "client_id".to_string(),
                            client_secret: "client_secret".to_string(),
                            refresh_token: Some("refresh_token".to_string()),
                            access_token: Some("access_token".to_string()),
                        }),
                    },
                }
            );

            Ok(())
        });
    }

    #[test]
    fn should_use_env_overrides() {
        figment::Jail::expect_with(|jail| {
            jail.clear_env();
            jail.create_file(
                "conf.toml",
                r#"
                    [api]
                    url = "https://test.com/"
                "#,
            )?;

            jail.set_env("MOLLIE_API__URL", "https://env.com/");
            jail.set_env(
                "MOLLIE_AUTH__ACCESS_CODE__TOKEN",
                "access_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx123",
            );
            jail.set_env(
                "MOLLIE_AUTH__API_KEYS__LIVE",
                "live_xxxxxxxxxxxxxxxxxxxxxxxxxxx123",
            );

            let service = FigmentConfigurationService::new();
            let config = service.read();

            assert_eq!(
                config,
                &MollieConfig {
                    api: ApiConfig {
                        url: Url::parse("https://env.com/").unwrap(),
                    },
                    auth: AuthConfig {
                        api_keys: Some(ApiKeysConfig {
                            live: Some(auth::ApiKey {
                                mode: auth::ApiKeyMode::Live,
                                value: "live_xxxxxxxxxxxxxxxxxxxxxxxxxxx123".to_string(),
                            }),
                            test: None,
                        }),
                        access_code: Some(AccessCodeConfig {
                            token: auth::AccessCode {
                                value: "access_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx123"
                                    .to_string()
                            },
                        }),
                        connect: None,
                    }
                }
            );

            Ok(())
        });
    }

    #[test]
    fn should_use_env_variable_shorthands() {
        figment::Jail::expect_with(|jail| {
            jail.clear_env();
            jail.create_file(
                "conf.toml",
                r#"
                    [api]
                    url = "https://test.com/"
                "#,
            )?;

            jail.set_env("MOLLIE_API_URL", "https://env.com/");
            jail.set_env(
                "MOLLIE_ACCESS_TOKEN",
                "access_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx123",
            );
            jail.set_env("MOLLIE_API_KEY", "live_xxxxxxxxxxxxxxxxxxxxxxxxxxx123");

            let service = FigmentConfigurationService::new();
            let config = service.read();

            assert_eq!(
                config,
                &MollieConfig {
                    api: ApiConfig {
                        url: Url::parse("https://env.com/").unwrap(),
                    },
                    auth: AuthConfig {
                        api_keys: Some(ApiKeysConfig {
                            live: Some(auth::ApiKey {
                                mode: auth::ApiKeyMode::Live,
                                value: "live_xxxxxxxxxxxxxxxxxxxxxxxxxxx123".to_string(),
                            }),
                            test: None,
                        }),
                        access_code: Some(AccessCodeConfig {
                            token: auth::AccessCode {
                                value: "access_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx123"
                                    .to_string()
                            },
                        }),
                        connect: None,
                    }
                }
            );

            Ok(())
        });
    }
}
