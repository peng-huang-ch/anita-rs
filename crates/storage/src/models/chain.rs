use serde::Deserialize;
use strum::{Display, EnumCount, EnumDiscriminants, EnumString};
use strum_macros::{AsRefStr, EnumIs};

use crate::DatabaseError;

#[derive(
    AsRefStr,
    clap::ValueEnum,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    EnumString,
    Display,
    EnumCount,
    EnumDiscriminants,
    EnumIs,
    Deserialize,
)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Chain {
    /// Docs on red
    #[strum(ascii_case_insensitive, serialize = "solana", serialize = "sol")]
    Solana,
    #[strum(ascii_case_insensitive, serialize = "eth", to_string = "eth")]
    Ethereum,
    #[strum(disabled)]
    Unknown,
}

/// Keypair interface.
pub trait KeypairStrategy: Send {
    /// Get the chain.
    fn chain(&self) -> Chain;
    /// Generate a new keypair.
    fn generate(&mut self);
    /// Recover a keypair from a secret string.
    fn recover_secret(&mut self, secret: &str) -> Result<(), DatabaseError>;
    /// Recover a keypair from a bytes.
    fn recover_from_bytes(&mut self, bytes: &[u8]) -> Result<(), DatabaseError>;
    /// Get the secret key.
    fn to_vec(&self) -> Vec<u8>;
    /// Get the secret key.
    fn secret(&self) -> String;
    /// Get the public key.
    fn pubkey(&self) -> String;
    /// Get the address key.
    fn address(&self) -> String;
    /// Sign a message with a external secret.
    fn sign(&self, message: &[u8]) -> Result<String, DatabaseError>;
}

#[cfg(test)]
mod tests {
    use super::Chain;

    #[test]
    fn test_from_str() {
        let chain = Chain::try_from("solana").expect("invalid chain");
        assert_eq!(chain, Chain::Solana);

        let chain = Chain::try_from("SOLANA").expect("invalid chain");
        assert_eq!(chain, Chain::Solana);
    }
}
