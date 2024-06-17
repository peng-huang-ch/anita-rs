use clap::Parser;

use r_api::init_api;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Command {
    /// The database to save the keys.
    #[arg(short, long, value_name = "database_url", env("DATABASE_URL"))]
    database_url: String,

    /// Number of threads to use
    #[arg(short, long, default_value_t = 3000, env("PORT"))]
    port: u16,
}

impl Command {
    /// Execute `api` command
    pub async fn execute(self) -> eyre::Result<()> {
        let _ = init_api(self.port, self.database_url.as_str()).await;
        Ok(())
    }
}
