use clap::{Parser, Subcommand};
use log::debug;
mod auth;
mod config;
mod logger;
mod molliesdk;
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
}

fn main() {
    let cli = Cli::parse();

    logger::init(cli.debug);

    if cli.debug {
        debug!("Debug mode enabled");
    }

    match cli.command {
        Some(Commands::Payments(command)) => payments::command(&command),
        Some(Commands::Auth(command)) => auth::command(&command),
        None => {}
    }
}
