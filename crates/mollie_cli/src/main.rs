use clap::{Parser, Subcommand};
use log::debug;
mod auth;
mod balance;
mod config;
mod console;
mod env;
mod logger;
mod mollie;
mod org;
mod payments;

#[derive(Parser)]
#[clap(version, about, arg_required_else_help(true))]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,

    #[clap(short, long, global = true)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Do things with Payments
    Payments(payments::PaymentsCommmand),
    /// Do Auth things
    Auth(auth::AuthCommand),
    /// Do environmenty things
    Env(env::EnvCommand),
    /// Do Organizationy things
    Org(org::OrgCommand),
    /// Do Balance things
    Balance(balance::BalanceCommand),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    logger::init(cli.debug);

    if cli.debug {
        debug!("Debug mode enabled");
    }

    match cli.command {
        Some(Commands::Payments(command)) => payments::command(&command),
        Some(Commands::Auth(command)) => auth::command(&command),
        Some(Commands::Env(command)) => env::command(&command),
        Some(Commands::Org(command)) => org::command(&command).await?,
        Some(Commands::Balance(command)) => balance::command(&command).await?,
        None => {}
    };

    Ok(())
}
