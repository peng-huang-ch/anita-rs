use reqwest::{cookie::Jar, Client, Url};
use std::sync::Arc;

use clap::{Parser, Subcommand};

use r_storage::models::chain::Chain;

use crate::handlers::auth::{key_gen, key_sign, login, logout};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Command {
    /// The chain to use
    #[clap(short, long, value_enum, default_value_t = Chain::SOLANA)]
    chain: Chain,

    /// The remote server host
    #[arg(long, value_name = "server host", env("KM"))]
    host: String,

    /// Login Email
    #[clap(short, long, value_name = "email", env("EMAIL"))]
    email: String,

    /// Login Password
    #[clap(
        short,
        long,
        value_name = "password",
        env("PASSWORD"),
        hide_env_values = true,
        required = true
    )]
    password: String,

    #[clap(subcommand)]
    command: Subcommands,
}

#[derive(Debug, Subcommand)]
/// `anita key` subcommands
pub enum Subcommands {
    /// New a keypair
    Gen,
    /// New a keypair
    Sign {
        /// The pubkey to sign
        #[arg(short, long)]
        pubkey: String,

        /// The message to sign
        #[arg(short, long)]
        message: String,
    },
}

impl Command {
    /// Execute `key` command
    pub async fn execute(self) -> eyre::Result<()> {
        dotenvy::dotenv().ok();

        let chain = self.chain.to_string();
        let email = self.email;
        let password = self.password;

        let cookie_jar = Arc::new(Jar::default());
        let client = Client::builder()
            .cookie_store(true)
            .cookie_provider(Arc::clone(&cookie_jar))
            .build()
            .expect("Failed to build client");

        let host = self.host;
        let base = Url::parse(&host).expect("failed to parse url");
        login(&client, &base, email, password).await.expect("failed to login");

        match self.command {
            Subcommands::Gen => {
                let key = key_gen(&client, &base, chain.as_str()).await?;
                println!("keygen: {:?}", key.to_string());
            }
            Subcommands::Sign { pubkey, message, .. } => {
                let message = message;
                let data =
                    key_sign(&client, &base, chain.as_str(), pubkey.as_str(), message.as_str())
                        .await?;
                println!("signature: {:?}", data.to_string());
            }
        }

        logout(&client, &base).await.expect("failed to logout");
        Ok(())
    }
}
