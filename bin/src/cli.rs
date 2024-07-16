use clap::{Parser, Subcommand};

use r_tracing::init_logging;

use crate::commands::{api, db, key};
#[derive(Parser)]
#[clap(version, about)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn from_env_and_args() -> Self {
        dotenvy::dotenv().ok();
        Self::parse()
    }
}

/// Work seamlessly with Anita from the command line.
///
/// See `anita --help` for more information.
#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "api", about = "Start the API server")]
    Api(api::Command),

    #[command(name = "key", about = "Manage the keypairs")]
    Key(key::Command),

    #[command(name = "db", about = "Database tools")]
    DB(db::Command),
}

/// Parse CLI options, set up logging and run the chosen command.
pub async fn run() -> eyre::Result<()> {
    let opt = Cli::from_env_and_args();

    let server: String = std::env::var("LOG_SERVER").unwrap_or("anita::api".to_string());
    let level = std::env::var("LOG_LEVEL").unwrap_or("info".to_string());
    let guard = init_logging(server, level);

    match opt.command {
        Commands::Api(command) => command.execute().await?,
        Commands::DB(command) => command.execute().await?,
        Commands::Key(command) => command.execute().await?,
    };
    drop(guard);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    /// Tests that the help message is parsed correctly. This ensures that clap args are configured
    /// correctly and no conflicts are introduced via attributes that would result in a panic at
    /// runtime
    #[test]
    fn test_parse_help_all_subcommands() {
        let cli: clap::Command = Cli::command();
        for sub_command in cli.get_subcommands() {
            let err = Cli::try_parse_from(["key", sub_command.get_name(), "--help"])
                .err()
                .unwrap_or_else(|| {
                    panic!("Failed to parse help message {}", sub_command.get_name())
                });

            // --help is treated as error, but
            // > Not a true "error" as it means --help or similar was used. The help message will be sent to stdout.
            assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
        }
    }

    // #[test]
    // fn test_key_vanity() {
    //     let _ = Cli::try_parse_from(["key", "vanity", "-s", "p"]).unwrap();
    // }
}
