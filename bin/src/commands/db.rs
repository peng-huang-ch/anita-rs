//! Database debugging tool

use clap::{Parser, Subcommand};
use r_storage::prelude::{get_db_version, run_migrations, Database};

#[derive(Debug, Parser)]
pub struct Command {
    /// The database to save the keys.
    #[arg(short, long, value_name = "database_url", env("DATABASE_URL"))]
    database_url: String,

    #[clap(subcommand)]
    command: Subcommands,
}

#[derive(Subcommand, Debug)]
/// `anita db` subcommands
pub enum Subcommands {
    /// Execute database migrations
    Migration,
    /// Lists current and local database versions
    Version,
}

impl Command {
    /// Execute `db` command
    pub async fn execute(self) -> eyre::Result<()> {
        let database_url = self.database_url.as_str();

        match self.command {
            Subcommands::Migration => {
                let _migrated = run_migrations(database_url).await;
                println!("database migrations complete")
            }
            Subcommands::Version {} => {
                let database = Database::new_with_url(database_url, None).await;
                let mut conn = database.with_conn().await.expect("could not get connection");

                let version = get_db_version(&mut conn).await;
                println!("database version {}", version);
            }
        }
        Ok(())
    }
}
