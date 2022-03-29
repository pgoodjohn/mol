use super::config;
use clap::{Parser, Subcommand};
use log::info;
use serde::{Deserialize, Serialize};
use strum::EnumString;

mod update_environment;

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

#[derive(EnumString, Debug)]
pub enum Environments {
    #[strum(ascii_case_insensitive)]
    Prod,
    #[strum(ascii_case_insensitive)]
    Dev,
}

pub fn command(command: &EnvCommand) {
    match command.command.as_ref() {
        Some(EnvCommands::Url { environment }) => {
            update_environment::set_environment(&environment);
        }
        None => {
            info!(
                "Your mol-cli is configured to talk to: {}",
                config::api_url().unwrap()
            );
            info!("To switch your configuration, run 'mol env url prod' or 'mol env url dev'")
        }
    }
}
