use super::config;
use super::molliesdk;
use clap::{Parser, Subcommand};

mod create_payment;
mod get_payment;

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
    /// Get a payment's info
    #[clap(arg_required_else_help(true))]
    Get {
        #[clap(short, long)]
        id: String,
    },
}

pub fn command(payments_command: &PaymentsCommmand) {
    match payments_command.command.as_ref() {
        Some(PaymentsCommands::Create {}) => {
            create_payment::command().unwrap();
        }
        Some(PaymentsCommands::Get { id }) => {
            get_payment::command(id).unwrap();
        }
        None => {}
    }
}
