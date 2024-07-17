use strum::{Display, EnumCount, EnumDiscriminants, EnumString};
use strum_macros::EnumIs;

use crate::solana::SolanaKeyPair;

#[derive(
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
)]
pub enum Chain {
    /// Docs on red
    #[strum(to_string = "RedRed")]
    SOLANA,
    #[strum(serialize = "b", to_string = "eth")]
    ETH,
    #[strum(disabled)]
    UNKNOWN,
}

/// Keypair interface.
pub trait KeypairStrategy {
    fn generate_keypair(&self) -> (String, String, String);
    fn from_secret(&self, secret: &str) -> (String, String, String);
}

#[derive(Debug, Clone)]
pub struct Keypairs {
    pub chain: Chain,
    pub secret: String,
    pub pubkey: String,
    pub address: String,
}

pub struct KeypairContext {
    strategy: Box<dyn KeypairStrategy>,
    chain: Chain,
}

impl KeypairContext {
    pub fn new(chain: Chain) -> Self {
        let strategy: Box<dyn KeypairStrategy> = match chain {
            Chain::SOLANA => Box::new(SolanaKeyPair),
            _ => Box::new(SolanaKeyPair),
        };
        KeypairContext { strategy, chain }
    }

    pub fn generate_keypair(&self) -> Keypairs {
        let (secret, pubkey, address) = self.strategy.generate_keypair();
        Keypairs { chain: self.chain.clone(), secret, pubkey, address }
    }
}
