use crate::SolanaKeyPair;
use crate::{Chain, DatabaseError, KeypairStrategy};

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

    /// Create a new keypair context with secret.
    pub fn from_chain_secret(chain: Chain, secret: &str) -> Self {
        let keypair: Box<dyn KeypairStrategy> = match chain {
            Chain::SOLANA => Box::new(SolanaKeyPair::from_secret(secret)),
            _ => Box::new(SolanaKeyPair::from_secret(secret)),
        };
        KeypairContext { keypair, chain }
    }

    pub fn keypair(&self) -> &Box<dyn KeypairStrategy> {
        &self.keypair
    }

    /// Generate a new keypair.
    pub fn generate_keypair(&mut self) -> &Box<dyn KeypairStrategy> {
        let _ = &mut self.keypair.generate();
        &self.keypair
    }

    /// Sign a message with a secret key.
    pub fn sign(&self, secret: &[u8], message: &[u8]) -> Result<String, DatabaseError> {
        self.keypair.sign(secret, message)
    }
}
