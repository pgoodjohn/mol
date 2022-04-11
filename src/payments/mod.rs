use super::config;
use super::console;
use super::mollie;
use clap::{Parser, Subcommand};

mod create;
mod get;
mod list;

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
    Create {
        #[clap(short, long, global = true)]
        debug: bool,

        #[clap(short, long)]
        interactive: bool,

        #[clap(long)]
        currency: Option<String>,

        #[clap(long)]
        amount: Option<String>,

        #[clap(long)]
        description: Option<String>,

        #[clap(long)]
        redirect_url: Option<String>,

        #[clap(long)]
        profile_id: Option<String>,
    },
    /// Get a payment's info
    #[clap(arg_required_else_help(true))]
    Get {
        #[clap(parse(try_from_str))]
        id: String,
    },
    /// List payments
    List {},
}

pub fn command(payments_command: &PaymentsCommmand) {
    match payments_command.command.as_ref() {
        Some(PaymentsCommands::Create {
            debug,
            interactive,
            currency,
            amount,
            description,
            redirect_url,
            profile_id,
        }) => {
            match interactive {
                true => {
                    create::interactive(debug);
                    return;
                }
                false => {}
            }

            create::command(
                currency.as_ref(),
                amount.as_ref(),
                description.as_ref(),
                redirect_url.as_ref(),
                profile_id.as_ref(),
                debug,
            );
        }
        Some(PaymentsCommands::Get { id }) => {
            get::command(id);
        }
        Some(PaymentsCommands::List {}) => {
            list::command();
        }
        None => {}
    }
}
