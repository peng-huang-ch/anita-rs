use clap::{Parser, Subcommand};

use r_keys::key::generator;
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
        let suffix = self.suffix;
        let pool = init_db(self.database_url.as_str()).await;
        match self.command {
            Subcommands::Get => {
                let mut conn = pool.get().await?;
                let key = get_valid_suffix_key(&mut conn, suffix).await?;
                println!("key: {:?}", key);
            }
            Subcommands::New { count, .. } => {
                let secret = generator(count.into(), suffix.as_str());
                let mut conn = pool.get().await?;
                let key = Key {
                    secret: secret.clone(),
                    suffix: suffix.clone(),
                    used_at: Some(chrono::Utc::now().naive_utc()),
                };
                let _ = create_key(&mut conn, key).await?;
                println!("key: {}", secret);
            }
            Subcommands::Vanity { count, .. } => loop {
                let mut conn = pool.get().await?;
                let secret = generator(count.into(), suffix.as_str());
                let key = Key {
                    suffix: suffix.clone(),
                    secret: secret.clone(),
                    used_at: None,
                };
                let _ = create_key(&mut conn, key).await?;
                println!("key: {}", secret);
            },
        }

        Ok(())
    }
}
