use clap::{Parser, Subcommand};

mod get;
mod list;

#[derive(Parser)]
#[clap(version, about)]
pub struct BalanceCommand {
    #[clap(short, long, global = true)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<BalanceCommands>,
}

#[derive(Subcommand)]
pub enum BalanceCommands {
    /// List balances
    Get {
        #[clap(default_value = "primary")]
        id: String,
    },
    List {
        #[clap(short, long)]
        limit: Option<i32>,
        #[clap(short, long)]
        from: Option<String>,
    },
}

pub async fn command(command: &BalanceCommand) -> anyhow::Result<()> {
    match command.command.as_ref() {
        Some(BalanceCommands::Get { id }) => {
            get::command(id).await
        }
        Some(BalanceCommands::List { limit, from }) => {
            list::command(limit, from).await
        }
        None => Ok(())
    }
}
