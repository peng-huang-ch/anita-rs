use clap::{Parser, Subcommand};

use crate::{
    keys::keygen::keygen,
    storage::{Chain, Database, KeyTrait, NewKey},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Command {
    /// The database to save the keys.
    #[arg(
        short,
        long,
        value_name = "database_url",
        env("DATABASE_URL"),
        hide_env_values = true,
        required = true
    )]
    database_url: String,

    /// The database seed.
    #[arg(long, value_name = "seed", env("SEED"), hide_env_values = true)]
    seed: Option<String>,

    /// The chain to use
    #[clap(short, long, value_enum, default_value_t = Chain::Solana)]
    chain: Chain,

    /// The suffix to search
    #[arg(short, long, default_value = "sol")]
    suffix: String,

    #[clap(subcommand)]
    command: Subcommands,
}

#[derive(Debug, Subcommand)]
/// `anita key` subcommands
pub enum Subcommands {
    /// Get a keypair
    Get,
    /// New a keypair
    New {
        /// Number of threads to use
        #[arg(short, long, default_value_t = 4)]
        count: u8,
    },
    /// Vanity keypairs
    Vanity {
        /// Number of threads to use
        #[arg(short, long, default_value_t = 4)]
        count: u8,
    },
}

impl Command {
    /// Execute `key` command
    pub async fn execute(self) -> eyre::Result<()> {
        dotenvy::dotenv().ok();

        let chain = self.chain;
        let suffix = self.suffix;
        let seed = self
            .seed
            .map(|s| Database::to_seed(s.as_str()).expect("Seed must be a valid hex string"));
        let database = Database::new_with_url(self.database_url.as_str(), seed).await;
        match self.command {
            Subcommands::Get => {
                let key = database.get_key_by_suffix(chain, suffix.as_str()).await?;
                println!("key: {:?}", key);
            }
            Subcommands::New { count, .. } => {
                let context = keygen(count, suffix.as_str(), chain);
                let keypair = context.keypair();

                let key = NewKey::from_keypair(keypair, Some(suffix.clone()));
                let _ = database.create_key(key).await?;

                println!("key: {}", keypair.secret());
                println!("address : {}", keypair.address());
            }
            Subcommands::Vanity { count, .. } => loop {
                let context = keygen(count, suffix.as_str(), chain);
                let keypair = context.keypair();

                let mut key = NewKey::from_keypair(keypair, Some(suffix.clone()));
                key.used_at = Some(chrono::Utc::now().naive_utc());

                let _ = database.create_key(key).await?;
                println!("key: {}", keypair.secret());
                println!("address : {}", keypair.address());
            },
        }

        Ok(())
    }
}
