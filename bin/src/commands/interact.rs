use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, Password, Select};
use reqwest::{cookie::Jar, Client, Url};
use std::sync::Arc;

use crate::handlers::auth::{key_gen, key_sign, login, logout};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Command {
    /// The remote server host
    #[arg(long, value_name = "server host", env("KM"))]
    host: String,
}

impl Command {
    pub async fn execute(self) -> eyre::Result<()> {
        let cookie_jar = Arc::new(Jar::default());
        let client = Client::builder()
            .cookie_store(true)
            .cookie_provider(Arc::clone(&cookie_jar))
            .build()?;

        let theme = ColorfulTheme::default();
        let selections = &[
            "solana",
            // "ethereum",
            // "bitcoin",
        ];
        let selection = Select::with_theme(&theme)
            .with_prompt("Chain to use")
            .default(0)
            .items(&selections[..])
            .interact()?;
        let chain = selections[selection];

        let email: String = Input::with_theme(&theme).with_prompt("Email:").interact_text()?;

        let password = Password::with_theme(&theme)
            .with_prompt("Password:")
            .with_confirmation("Repeat password:", "Error: the passwords don't match.")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.chars().count() > 3 {
                    Ok(())
                } else {
                    Err("Password must be longer than 3")
                }
            })
            .interact()?;

        let host = self.host;
        let base = Url::parse(&host)?;
        login(&client, &base, email, password).await?;

        let selections = &[
            "Generate a new keypair", // Generate a new keypair
            "Sign a message",         // Sign a message
            "Exit",                   // Sign a message
        ];

        loop {
            let selection: usize = Select::with_theme(&theme)
                .with_prompt("Operation:")
                .default(0)
                .items(&selections[..])
                .interact()?;

            match selection {
                0 => {
                    let key = key_gen(&client, &base, chain).await?;
                    println!("{:?}", key.to_string());
                }
                1 => {
                    let pubkey: String =
                        Input::with_theme(&theme).with_prompt("Pubkey:").interact_text()?;
                    let message: String =
                        Input::with_theme(&theme).with_prompt("Message:").interact_text()?;
                    let signature =
                        key_sign(&client, &base, chain, pubkey.as_str(), message.as_str()).await?;
                    println!("{:?}", signature.to_string());
                }
                _ => {
                    logout(&client, &base).await.expect("failed to logout");
                    break;
                }
            }
        }

        Ok(())
    }
}
