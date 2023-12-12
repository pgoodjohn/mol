use crate::config::ConfigurationService;
use clap::{Parser, Subcommand};
use log::info;
use reqwest::Url;
use strum::EnumString;

#[derive(Parser)]
#[clap(version, about, arg_required_else_help(false))]
pub struct EnvCommand {
    #[clap(short, long, global = true)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<EnvCommands>,
}

#[derive(Subcommand)]
pub enum EnvCommands {
    /// Set the API url (switch between dev and production)
    Url {
        #[clap(help = "prod, dev")]
        environment: Environments,
    },
}

#[derive(EnumString, Debug, Clone)]
pub enum Environments {
    #[strum(ascii_case_insensitive)]
    Prod,
    #[strum(ascii_case_insensitive)]
    Dev,
}

pub fn command(
    command: &EnvCommand,
    config_service: &mut dyn ConfigurationService,
) -> anyhow::Result<()> {
    match command.command.as_ref() {
        Some(EnvCommands::Url { environment }) => {
            config_service.update(&|config| {
                config.api.url = match environment {
                    Environments::Dev => Url::parse("https://api.mollie.dev").unwrap(),
                    Environments::Prod => Url::parse("https://api.mollie.com").unwrap(),
                };
            })?;
        }
        None => {
            let config = config_service.read();
            info!(
                "Your mol-cli is configured to talk to: {}",
                config.api.url.as_str()
            );
            info!("To switch your configuration, run 'mol env url prod' or 'mol env url dev'")
        }
    }
    Ok(())
}
