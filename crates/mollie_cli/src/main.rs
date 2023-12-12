use clap::{Parser, Subcommand};
use log::debug;
mod auth;
mod balances;
mod config;
mod console;
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
    /// Do Auth things
    Auth(auth::AuthCommand),
    /// Do Balance things
    Balances(balances::BalancesCommand),
    /// Do Organizationy things
    Org(org::OrgCommand),
    /// Do things with Payments
    Payments(payments::PaymentsCommmand),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    logger::init(cli.debug);

    if cli.debug {
        debug!("Debug mode enabled");
    }

    match cli.command {
        Some(Commands::Payments(command)) => payments::command(&command).await,
        Some(Commands::Auth(command)) => auth::command(&command),
        Some(Commands::Balances(command)) => balances::command(&command).await?,
        Some(Commands::Org(command)) => org::command(&command).await?,
        Some(Commands::Payments(command)) => payments::command(&command),
        None => {}
    };

    Ok(())
}
