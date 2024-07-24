use crate::SolanaKeyPair;
use crate::{Chain, KeypairStrategy, Keypairs};

/// A context for generating and signing keypairs.
pub struct KeypairContext {
    keypair: Box<dyn KeypairStrategy>,
    pub chain: Chain,
}

/// A context for generating and signing keypairs.
impl KeypairContext {
    /// Create a new keypair context.
    pub fn from_chain(chain: Chain) -> Self {
        let keypair: Box<dyn KeypairStrategy> = match chain {
            Chain::SOLANA => Box::new(SolanaKeyPair::new()),
            _ => Box::new(SolanaKeyPair::new()),
        };
        KeypairContext { keypair, chain }
    }

    pub fn keypair(&self) -> &Box<dyn KeypairStrategy> {
        &self.keypair
    }

    /// Generate a new keypair.
    pub fn generate_keypair(&self) -> Keypairs {
        self.keypair.generate()
    }

    /// Sign a message with a secret key.
    pub fn sign(&self, secret: &str, message: &str) -> String {
        let message = message.as_bytes();
        self.keypair.sign(secret, message)
    }
}
