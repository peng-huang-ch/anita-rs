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

    /// Number of threads to use
    #[arg(short, long, default_value_t = 3000, env("PORT"))]
    port: u16,
}

impl Command {
    /// Execute `api` command
    pub async fn execute(self) -> eyre::Result<()> {
        let database = Database::new_with_url(self.database_url.as_str()).await;
        let _ = init_api(self.port, database).await?;
        Ok(())
    }
}
