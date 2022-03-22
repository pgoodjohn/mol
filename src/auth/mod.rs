use clap::{Parser, Subcommand};
use log::debug;

mod store_api_key;

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
    Add {},
}

pub fn command(command: &AuthCommand) {
    match command.command {
        Some(AuthCommands::Add {}) => {
            store_api_key::command();
        }
        None => {}
    }
}
