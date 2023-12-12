use crate::config::ConfigurationService;
use clap::{Parser, Subcommand};
use log::info;
use mollie_api::auth::{AccessCode, ApiKey};

mod store;

#[derive(Parser)]
#[clap(version, about, arg_required_else_help(true))]
pub struct AuthCommand {
    #[clap(short, long, global = true)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<AuthCommands>,
}

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Add a new API key
    #[clap(arg_required_else_help(true))]
    Add {
        #[clap(short, long)]
        interactive: bool,

        #[clap(long)]
        api_key: Option<String>,

        #[clap(long)]
        access_code: Option<String>,
    },
    /// Get Auth information
    Get {},
}

pub async fn command(
    command: &AuthCommand,
    config_service: &mut dyn ConfigurationService,
) -> miette::Result<()> {
    match command.command.as_ref() {
        Some(AuthCommands::Add {
            interactive,
            api_key,
            access_code,
        }) => {
            let mut store = store::Store::new(config_service);

            if *interactive {
                return store.interactive();
            }

            if let Some(api_key) = api_key {
                let parsed_api_key = ApiKey::try_from(api_key.clone())?;
                return store.store_api_key(parsed_api_key);
            }

            if let Some(access_code) = access_code {
                let parsed_access_code = AccessCode::try_from(access_code.clone())?;
                return store.store_access_code(parsed_access_code);
            }
        }
        Some(AuthCommands::Get {}) => {
            let config = config_service.read();

            info!("Retrieving current configuration");
            info!("Live API Key: {:?}", config.live_api_key());
            info!("Test API Key: {:?}", config.test_api_key());
            info!("Access Token: {:?}", config.access_code());
        }
        None => {}
    }
    Ok(())
}
