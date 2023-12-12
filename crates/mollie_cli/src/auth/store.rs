use crate::config::{AccessCodeConfig, ApiKeysConfig, ConfigurationService};
use mollie_api::auth::{AccessCode, ApiKey, ApiKeyMode};
use requestty::Question;

pub struct Store<'config> {
    config_service: &'config mut dyn ConfigurationService,
}

impl<'config> Store<'config> {
    pub fn new(config: &'config mut dyn ConfigurationService) -> Self {
        Self {
            config_service: config,
        }
    }

    pub fn interactive(&mut self) -> anyhow::Result<()> {
        let new_api_key = self.ask_api_key()?;
        self.store_api_key(new_api_key)
    }

    pub fn store_api_key(&mut self, new_api_key: ApiKey) -> anyhow::Result<()> {
        self.config_service.update(&|config| {
            let api_keys = config.auth.api_keys.get_or_insert(ApiKeysConfig::default());
            match new_api_key.mode {
                ApiKeyMode::Live => {
                    api_keys.live = Some(new_api_key.clone());
                }
                ApiKeyMode::Test => {
                    api_keys.test = Some(new_api_key.clone());
                }
            }
        })?;
        Ok(())
    }

    pub fn store_access_code(&mut self, new_access_code: AccessCode) -> anyhow::Result<()> {
        self.config_service.update(&|config| {
            config.auth.access_code = Some(AccessCodeConfig {
                token: new_access_code.clone(),
            });
        })?;
        Ok(())
    }

    fn ask_api_key(&self) -> anyhow::Result<ApiKey> {
        let question = Question::input("api_key")
            .message("Input your new API key")
            .build();

        let answer = requestty::prompt_one(question)?
            .try_into_string()
            .map_err(|_| anyhow::anyhow!("Could not read API key"))?;

        Ok(ApiKey::try_from(answer)?)
    }
}
