use super::config;
use super::console;
use super::mollie;
use clap::{Parser, Subcommand};

mod create;
mod get;
mod list;
mod refund;

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
    Get { id: String },
    /// List payments
    List {
        #[clap(short, long)]
        limit: Option<i32>,
        #[clap(short, long)]
        from: Option<String>,
        #[clap(short, long)]
        profile_id: Option<String>,
        #[clap(short, long)]
        test_mode: Option<bool>,
    },
    /// Refund a payment
    #[clap(arg_required_else_help(true))]
    Refund {
        id: String,

        #[clap(long)]
        amount: f32,

        #[clap(long)]
        description: String,
    },
}

pub async fn command(payments_command: &PaymentsCommmand) {
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
            get::command(id).await;
        }
        Some(PaymentsCommands::List { limit, from, profile_id, test_mode }) => {
            list::command(limit, from, profile_id, test_mode).await;
        }
        Some(PaymentsCommands::Refund {
            id,
            amount,
            description,
        }) => {
            refund::command(id, amount, description);
        }
        None => {}
    }
}
