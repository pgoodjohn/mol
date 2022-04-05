use super::mollie_sdk;
use clap::{Parser, Subcommand};

mod me;

#[derive(Parser)]
#[clap(version, about)]
pub struct OrgCommand {
    #[clap(short, long, global = true)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<OrgCommands>,
}

#[derive(Subcommand)]
pub enum OrgCommands {
    /// Get the permissions for the currently stored access token
    Permissions {},
}

pub fn command(command: &OrgCommand) {
    match command.command.as_ref() {
        Some(OrgCommands::Permissions {}) => {
            panic!("Not yet built")
        }
        None => {
            me::command();
        }
    }
}
