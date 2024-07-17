use clap::{Parser, Subcommand};

use r_keys::{keygen::keygen, keypair::Chain};
use r_storage::{
    init_db,
    models::keys::{create_key, get_valid_suffix_key, Key},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Command {
    /// The database to save the keys.
    #[arg(short, long, value_name = "database_url", env("DATABASE_URL"))]
    database_url: String,

    /// The chain to use
    #[clap(short, long, value_enum, default_value_t = Chain::SOLANA)]
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
        let pool = init_db(self.database_url.as_str()).await;
        match self.command {
            Subcommands::Get => {
                let mut conn = pool.get().await?;
                let key = get_valid_suffix_key(&mut conn, suffix).await?;
                println!("key: {:?}", key);
            }
            Subcommands::New { count, .. } => {
                let pairs = keygen(count.into(), suffix.as_str(), chain);
                let mut conn = pool.get().await?;
                let key = Key {
                    chain: pairs.chain.to_string(),
                    secret: pairs.secret.clone(),
                    pubkey: pairs.pubkey.clone(),
                    address: pairs.address.clone(),
                    suffix: suffix.clone(),
                    used_at: Some(chrono::Utc::now().naive_utc()),
                };
                let _ = create_key(&mut conn, key).await?;
                println!("key: {}", pairs.secret);
                println!("address : {}", pairs.address);
            }
            Subcommands::Vanity { count, .. } => loop {
                let mut conn = pool.get().await?;
                let pairs = keygen(count.into(), suffix.as_str(), chain);
                let key = Key {
                    chain: pairs.chain.to_string(),
                    secret: pairs.secret.clone(),
                    pubkey: pairs.pubkey.clone(),
                    address: pairs.address.clone(),
                    suffix: suffix.clone(),
                    used_at: Some(chrono::Utc::now().naive_utc()),
                };
                let _ = create_key(&mut conn, key).await?;
                println!("key: {}", pairs.secret);
                println!("address : {}", pairs.address);
            },
        }

        Ok(())
    }
}
