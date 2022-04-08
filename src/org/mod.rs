use super::mollie;
use clap::{Parser, Subcommand};

mod me;
mod permissions;

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
    Permissions {
        #[clap(short, long)]
        granted: bool,
    },
}

pub fn command(command: &OrgCommand) {
    match command.command.as_ref() {
        Some(OrgCommands::Permissions { granted }) => {
            permissions::command(granted);
        }
        None => {
            me::command();
        }
    }
}
