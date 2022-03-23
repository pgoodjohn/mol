use super::config;
use super::molliesdk;
use clap::{Parser, Subcommand};

mod create_payment;

#[derive(Parser)]
#[clap(version, about, arg_required_else_help(true))]
pub struct PaymentsCommmand {
    #[clap(short, long, global = true)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<PaymentsCommands>,
}

#[derive(Subcommand)]
pub enum PaymentsCommands {
    /// Create a new payment
    Create {},
}

pub fn command(payments_command: &PaymentsCommmand) {
    match payments_command.command {
        Some(PaymentsCommands::Create {}) => {
            create_payment::command().unwrap();
        }
        None => {}
    }
}
