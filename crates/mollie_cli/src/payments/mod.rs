use crate::config::ConfigurationService;
use clap::{Parser, Subcommand};
use colored::Colorize;
use mollie_api::models::payment::PaymentResource;
use std::fmt::Display;
use strum::Display;
mod cancel;
mod create;
mod get;
mod list;
mod refund;

#[derive(Parser)]
#[clap(version, about, arg_required_else_help(true))]
pub struct PaymentsCommmand {
    #[clap(short, long, global = true)]
    debug: bool,

    #[clap(long = "withResponse", global = true)]
    with_response: bool,

    #[clap(subcommand)]
    command: Option<PaymentsCommands>,
}

#[derive(clap::ValueEnum, Clone, Display)]
pub enum Status {
    Open,
    Canceled,
    Pending,
    Authorized,
    Expired,
    Failed,
    Paid,
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
        #[clap(short, long)]
        status: Option<Status>,
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

    ///Cancel a Payment
    #[clap(arg_required_else_help(true))]
    Cancel { id: String },
}

pub async fn command(
    payments_command: &PaymentsCommmand,
    config_service: &dyn ConfigurationService,
) -> anyhow::Result<()> {
    let config = config_service.read();
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
                    return create::interactive(config, debug).await;
                }
                false => {}
            }

            create::command(
                config,
                currency.as_ref(),
                amount.as_ref(),
                description.as_ref(),
                redirect_url.as_ref(),
                profile_id.as_ref(),
                debug,
            )
            .await?;
        }
        Some(PaymentsCommands::Get { id }) => {
            get::command(config, id, payments_command.with_response).await?;
        }
        Some(PaymentsCommands::List {
            limit,
            from,
            profile_id,
            test_mode,
            status,
        }) => {
            list::command(
                config,
                limit,
                from,
                profile_id,
                test_mode,
                payments_command.with_response,
                status,
            )
            .await?;
        }
        Some(PaymentsCommands::Refund {
            id,
            amount,
            description,
        }) => {
            refund::command(
                config,
                id,
                amount,
                description,
                payments_command.with_response,
            )
            .await?;
        }
        Some(PaymentsCommands::Cancel { id }) => {
            cancel::command(config, id, payments_command.with_response).await?;
        }
        None => {}
    }

    Ok(())
}

pub struct Payment {
    pub id: String,
    pub mode: String,
    pub status: String,
    pub amount: String,
    pub created_at: String,
    pub description: String,
    pub redirect_url: String,
}

impl Payment {
    pub fn header() -> String {
        format!(
            "|{:^14} {:^8} {:^4} {:^12} {:^26} {:^30} {} |",
            "ID", "STATUS", "MODE", "AMOUNT", "CREATED_AT", "DESCITPION", "REDIRECT_URL"
        )
    }
}

impl From<PaymentResource> for Payment {
    fn from(payment: PaymentResource) -> Self {
        Self {
            id: payment.id,
            mode: payment.mode,
            status: payment.status,
            amount: payment.amount.to_string(),
            created_at: payment.created_at,
            description: payment.description,
            redirect_url: payment.redirect_url,
        }
    }
}

impl Display for Payment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | {} | {} | {} | {} | {} | {}  ",
            match self.status.as_str() {
                "open" => {
                    Colorize::blue(&*self.id)
                }
                "cancelled" => {
                    Colorize::yellow(&*self.id)
                }
                "pending" => {
                    Colorize::blue(&*self.id)
                }
                "authorized" => {
                    Colorize::blue(&*self.id)
                }
                "expired" => {
                    Colorize::yellow(&*self.id)
                }
                "failed" => {
                    Colorize::red(&*self.id)
                }
                "paid" => {
                    Colorize::green(&*self.id)
                }
                &_ => {
                    Colorize::blink(&*self.id)
                }
            },
            match self.status.as_str() {
                "open" => {
                    Colorize::blue(&*self.status)
                }
                "cancelled" => {
                    Colorize::yellow(&*self.status)
                }
                "pending" => {
                    Colorize::blue(&*self.status)
                }
                "authorized" => {
                    Colorize::blue(&*self.status)
                }
                "expired" => {
                    Colorize::yellow(&*self.status)
                }
                "failed" => {
                    Colorize::red(&*self.status)
                }
                "paid" => {
                    Colorize::green(&*self.status)
                }
                &_ => {
                    Colorize::blink(&*self.status)
                }
            },
            if self.mode == "live" {
                Colorize::bright_green("LIVE")
            } else {
                Colorize::bright_black("TEST")
            },
            Colorize::green(&*self.amount.to_string()),
            Colorize::blue(&*self.created_at),
            self.description,
            self.redirect_url
        )
    }
}
