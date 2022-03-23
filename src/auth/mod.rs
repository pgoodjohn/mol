use super::config;
use clap::{Parser, Subcommand};

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
    Add {
        #[clap(short, long)]
        interactive: bool,

        #[clap(long)]
        api_key: Option<String>,

        #[clap(long)]
        access_code: Option<String>,
    },
}

pub fn command(command: &AuthCommand) {
    match command.command.as_ref() {
        Some(AuthCommands::Add {
            interactive,
            api_key,
            access_code
        }) => {
            match interactive {
                true => {
                    store::interactive();
                    return;
                }
                false => {}
            };

            match api_key {
                Some(api_key) => {
                    store::api_key(&api_key);
                    return;
                }
                None => {}
            }

            match access_code {
                Some(access_code) => {
                    store::access_code(&access_code);
                    return;
                }
                None => {}
            }
        }
        None => {}
    }
}
