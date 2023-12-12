use clap::{Parser, Subcommand};
use config::FigmentConfigurationService;
use log::debug;

extern crate jsonxf;

mod auth;
mod balances;
mod config;
mod logger;
mod org;
mod payments;

#[derive(Parser)]
#[clap(version, about, arg_required_else_help(true))]
struct Cli {
    /// The Mollie API you want to invoke
    #[clap(subcommand)]
    command: Option<Commands>,

    /// Enable debug logging
    #[clap(short, long, global = true)]
    debug: bool,

    /// Print the API response after performing an API call
    #[clap(long = "withResponse", global = true)]
    with_response: bool,
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

    let mut config_service = FigmentConfigurationService::new();

    match cli.command {
        Some(Commands::Auth(command)) => auth::command(&command, &mut config_service).await?,
        Some(Commands::Balances(command)) => balances::command(&command, &config_service).await?,
        Some(Commands::Org(command)) => org::command(&command, &config_service).await?,
        Some(Commands::Payments(command)) => payments::command(&command, &config_service).await?,
        None => {}
    };

    Ok(())
}
