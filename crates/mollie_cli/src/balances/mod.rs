use crate::config::ConfigurationService;
use clap::{Parser, Subcommand};
use colored::Colorize;
use mollie_api::models::balance::BalanceResource;
use std::fmt::Display;

mod get;
mod list;

#[derive(Parser)]
#[clap(version, about)]
pub struct BalancesCommand {
    /// Enable debug logging
    #[clap(short, long, global = true)]
    debug: bool,

    /// Print the API response after performing an API call
    #[clap(long = "withResponse", global = true)]
    with_response: bool,


    #[clap(subcommand)]
    command: Option<BalanceCommands>,
}

#[derive(Subcommand)]
pub enum BalanceCommands {
    /// Get information about a single balance (defaults to your primary balance)
    Get {
        #[clap(default_value = "primary")]
        id: String,
    },
    /// List and get information about all your balances
    List {
        #[clap(short, long)]
        limit: Option<i32>,
        #[clap(short, long)]
        from: Option<String>,
    },
}

pub async fn command(
    command: &BalancesCommand,
    config_service: &dyn ConfigurationService,
) -> miette::Result<()> {
    let config = config_service.read();
    match command.command.as_ref() {
        Some(BalanceCommands::Get { id }) => get::command(config, id, command.with_response).await,
        Some(BalanceCommands::List { limit, from }) => {
            list::command(config, limit, from, command.with_response).await
        }
        None => Ok(()),
    }
}

pub struct Balance {
    pub id: String,
    pub mode: String,
    pub status: String,
    pub available_amount: String,
    pub pending_amount: String,
}

impl Balance {
    pub fn header() -> String {
        format!(
            "|  {:^24} {:^4} {:^12} {} |",
            "ID", "MODE", "AVAILABLE", "PENDING"
        )
    }
}

impl From<BalanceResource> for Balance {
    fn from(balance: BalanceResource) -> Self {
        Self {
            id: balance.id,
            mode: balance.mode,
            status: balance.status,
            available_amount: balance.available_amount.to_string(),
            pending_amount: balance.pending_amount.to_string(),
        }
    }
}

impl Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | {} | {} | {}",
            if self.status == "active" {
                Colorize::green(&*self.id)
            } else {
                Colorize::blink(&*self.id)
            },
            if self.mode == "live" {
                Colorize::bright_green("LIVE")
            } else {
                Colorize::bright_black("TEST")
            },
            Colorize::green(&*self.available_amount.to_string()),
            Colorize::yellow(&*self.pending_amount.to_string()),
        )
    }
}
