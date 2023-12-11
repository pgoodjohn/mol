use super::config;
use clap::{Parser, Subcommand};
use log::info;
use oauth2;
use oauth2::basic::BasicClient;
use oauth2::{AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken, Scope, TokenUrl};
use oauth2::reqwest::async_http_client;
use reqwest::Url;

mod store;

#[derive(Parser)]
#[clap(version, about, arg_required_else_help(true))]
pub struct AuthCommand {
    #[clap(short, long, global = true)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<AuthCommands>,
}

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Add a new API key
    #[clap(arg_required_else_help(true))]
    Add {
        #[clap(short, long)]
        interactive: bool,

        #[clap(long)]
        api_key: Option<String>,

        #[clap(long)]
        access_code: Option<String>,
    },
    #[clap(arg_required_else_help(true))]
    Connect {
        #[clap(long)]
        client_id: String,

        #[clap(long)]
        client_secret: Option<String>,

        #[clap(long)]
        finish: Option<String>,
    },
    /// Get Auth information
    Get {},
}

#[derive(Subcommand)]
pub enum AuthConnectFinishCommand {
    #[clap(arg_required_else_help(true))]
    Redirect {
        #[clap(long)]
        url: String,
    },
}

pub async fn command(command: &AuthCommand) -> anyhow::Result<()> {
    match command.command.as_ref() {
        Some(AuthCommands::Add {
            interactive,
            api_key,
            access_code,
        }) => {
            if *interactive {
                store::interactive()
            }

            match api_key {
                Some(api_key) => store::api_key(api_key),
                None => {}
            }

            match access_code {
                Some(access_code) => store::access_code(access_code),
                None => {}
            }
        }
        Some(AuthCommands::Get {}) => {
            info!("Retrieving current configuration");
            info!("Live API Key: {:?}", config::api_key().ok());
            info!("Test API Key: {:?}", config::api_key_test().ok());
            info!("Access Token: {:?}", config::access_code().ok());
        }
        Some(AuthCommands::Connect { client_id, client_secret, finish }) => {
            let client =
                BasicClient::new(
                    ClientId::new(client_id.into()),
                    client_secret.clone().map(ClientSecret::new),
                    AuthUrl::new("https://my.mollie.com/oauth2/authorize".into())?,
                    Some(TokenUrl::new("https://api.mollie.com/oauth2/tokens".into())?)
                );

            if let Some(finish) = finish {
                let url = Url::parse(&finish).expect("Invalid finish url");
                let code = url.query_pairs().find(|(key, _)| key == "code").unwrap().1;

                let request = client
                    .exchange_code(AuthorizationCode::new(code.into()));
                info!("{:#?}", request);

                let result = request
                    .request_async(async_http_client)
                    .await;
                info!("{:#?}", result);

            } else {
                let (auth_url, csrf_token) = client
                    .authorize_url(CsrfToken::new_random)
                    .add_extra_param("approval_prompt", "force")
                    .add_scope(Scope::new("organizations.read".to_string()))
                    .url();

                info!("Browse to: {}", auth_url);
            }
        }
        None => {}
    }
    Ok(())
}
