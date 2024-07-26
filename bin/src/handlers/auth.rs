use eyre::{anyhow, Result};
use reqwest::Client;
use reqwest::Url;
use serde_json::json;

/// Login to the get a session
pub async fn login(client: &Client, base: &Url, email: String, password: String) -> Result<()> {
    let url = base.join("/auth/login")?;
    let resp = client
        .post(url)
        .json(&json!({
            "email": email,
            "password": password,
        }))
        .send()
        .await?;

    if resp.status().is_client_error() {
        return Err(anyhow!("please check you email or password"));
    }

    if resp.status().is_server_error() {
        return Err(anyhow!("remote server is not available"));
    }

    Ok(())
}

/// Logout to the close the session
pub async fn logout(client: &Client, base: &Url) -> Result<()> {
    let url = base.join("/auth/logout")?;
    let resp = client.post(url).send().await?;
    if resp.status().is_client_error() {
        eprintln!("failed to logout: {:?}", resp);
    }

    if resp.status().is_server_error() {
        return Err(anyhow!("remote server is not available"));
    }
    Ok(())
}

/// Generate a key with chain
pub async fn key_gen(client: &Client, base: &Url, chain: &str) -> Result<serde_json::Value> {
    let url = base.join("/keys/gen")?;
    let resp = client
        .post(url)
        .json(&json!({ "chain": chain }))
        .send()
        .await
        .map_err(|_e| anyhow!("failed to build clint, please check you host"))?;

    if resp.status().is_client_error() {
        return Err(anyhow!("failed to gen key: {:?}", resp.text().await?));
    }

    if resp.status().is_server_error() {
        return Err(anyhow!("remote server is not available"));
    }

    let data = resp.json::<serde_json::Value>().await?;
    Ok(data)
}

/// Sign a message with a key
pub async fn key_sign(
    client: &Client,
    base: &Url,
    chain: &str,
    pubkey: &str,
    message: &str,
) -> Result<serde_json::Value> {
    let url = base.join("keys/sign")?;
    let resp = client
        .post(url)
        .json(&json!({ "chain": chain, "pubkey": pubkey, "message": message }))
        .send()
        .await
        .map_err(|_e| anyhow!("failed to build clint, please check you host"))?;

    if resp.status().is_client_error() {
        return Err(anyhow!("failed to sign key: {:?}", resp.text().await?));
    }

    if resp.status().is_server_error() {
        return Err(anyhow!("remote server is not available"));
    }

    let data = resp.json::<serde_json::Value>().await?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    use r_keys::Chain;
    use reqwest::cookie::Jar;
    use reqwest::Client;
    use reqwest::Url;

    use std::sync::Arc;

    #[test]
    fn test_url() {
        let base = Url::parse("http://127.0.0.1:8080/").expect("Failed to parse url");
        let url = base.join("/auth/login").expect("Failed to join url");
        println!("login_url: {:?}", url.to_string());
    }

    #[ignore]
    #[tokio::test]
    async fn test_login() -> eyre::Result<()> {
        dotenvy::dotenv().ok();
        let cookie_jar = Arc::new(Jar::default());
        let client = Client::builder()
            .cookie_store(true)
            .cookie_provider(Arc::clone(&cookie_jar))
            .build()
            .expect("Failed to build client");
        let email = std::env::var("EMAIL").expect("Email must be set");
        let password: String = std::env::var("PASSWORD").expect("Password must be set");

        let base = Url::parse("http://127.0.0.1:8080").expect("Failed to parse url");

        let _ = login(&client, &base, email, password).await?;

        let _ = logout(&client, &base).await?;
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_key_gen() -> eyre::Result<()> {
        dotenvy::dotenv().ok();
        let cookie_jar = Arc::new(Jar::default());
        let client = Client::builder()
            .cookie_store(true)
            .cookie_provider(Arc::clone(&cookie_jar))
            .build()
            .expect("Failed to build client");
        let email = std::env::var("EMAIL").expect("Email must be set");
        let password: String = std::env::var("PASSWORD").expect("Password must be set");

        let base = Url::parse("http://127.0.0.1:8080").expect("Failed to parse url");

        login(&client, &base, email, password).await?;

        let key = key_gen(&client, &base, "solana").await?;
        println!("key: {:?}", key.to_string());
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_key_sign() -> eyre::Result<()> {
        dotenvy::dotenv().ok();

        let cookie_jar = Arc::new(Jar::default());
        let client = Client::builder()
            .cookie_store(true)
            .cookie_provider(Arc::clone(&cookie_jar))
            .build()
            .expect("Failed to build client");

        let email = std::env::var("EMAIL").expect("Email must be set");
        let password: String = std::env::var("PASSWORD").expect("Password must be set");

        let base = Url::parse("http://127.0.0.1:8080").expect("Failed to parse url");

        login(&client, &base, email, password).await?;

        let key = key_gen(&client, &base, "solana").await?;

        let chain: Chain = Chain::Solana;
        let message = "hello".to_string();
        let result = key_sign(
            &client,
            &base,
            chain.to_string().as_str(),
            key["pubkey"].as_str().unwrap(),
            message.as_str(),
        )
        .await?;
        println!("signed result : {:?}", result.to_string());
        Ok(())
    }
}
