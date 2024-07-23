use crate::SolanaKeyPair;
use crate::{Chain, KeypairStrategy, Keypairs};

/// A context for generating and signing keypairs.
pub struct KeypairContext {
    pub strategy: Box<dyn KeypairStrategy>,
    pub chain: Chain,
}

/// A context for generating and signing keypairs.
impl KeypairContext {
    /// Create a new keypair context.
    pub fn from_chain(chain: Chain) -> Self {
        let strategy: Box<dyn KeypairStrategy> = match chain {
            Chain::SOLANA => Box::new(SolanaKeyPair::new()),
            _ => Box::new(SolanaKeyPair::new()),
        };
        KeypairContext { strategy, chain }
    }

    /// Generate a new keypair.
    pub fn generate_keypair(&self) -> Keypairs {
        self.strategy.generate_keypair()
    }

    /// Sign a message with a secret key.
    pub fn sign(&self, secret: &str, message: &str) -> String {
        let message = message.as_bytes();
        self.strategy.sign(secret, message)
    }
}
