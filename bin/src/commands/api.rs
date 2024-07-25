use clap::Parser;

use r_api::{init_api, Database};

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

    /// Number of threads to use
    #[arg(short, long, default_value_t = 3000, env("PORT"))]
    port: u16,
}

impl Command {
    /// Execute `api` command
    pub async fn execute(self) -> eyre::Result<()> {
        let seed = self
            .seed
            .map(|s| Database::to_seed(s.as_str()).expect("Seed must be a valid hex string"));
        let database = Database::new_with_url(self.database_url.as_str(), seed).await;
        let _ = init_api(self.port, database).await?;
        Ok(())
    }
}
