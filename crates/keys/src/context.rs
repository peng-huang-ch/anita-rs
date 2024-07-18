use crate::SolanaKeyPair;
use crate::{Chain, KeypairStrategy, Keypairs};

pub struct KeypairContext {
    pub strategy: Box<dyn KeypairStrategy>,
    pub chain: Chain,
}

impl KeypairContext {
    pub fn from_chain(chain: Chain) -> Self {
        let strategy: Box<dyn KeypairStrategy> = match chain {
            Chain::SOLANA => Box::new(SolanaKeyPair),
            _ => Box::new(SolanaKeyPair),
        };
        KeypairContext { strategy, chain }
    }

    pub fn generate_keypair(&self) -> Keypairs {
        self.strategy.generate_keypair()
    }

    pub fn sign(&self, secret: &str, message: &str) -> String {
        let message = message.as_bytes();
        self.strategy.sign(secret, message)
    }
}
