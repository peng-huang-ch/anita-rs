use crate::{Chain, DatabaseError, KeypairStrategy, SolanaKeyPair};

/// A context for generating and signing keypairs.
pub struct KeypairContext {
    keypair: Box<dyn KeypairStrategy>,
    pub chain: Chain,
}

/// A context for generating and signing keypairs.
impl KeypairContext {
    pub fn create_keypair(chain: Chain) -> Box<dyn KeypairStrategy> {
        match chain {
            Chain::Solana => Box::new(SolanaKeyPair::new()),
            _ => Box::new(SolanaKeyPair::new()),
        }
    }

    /// Create a new keypair context.
    pub fn from_chain(chain: Chain) -> Self {
        let keypair = Self::create_keypair(chain);
        KeypairContext { keypair, chain }
    }

    /// Create a new keypair context with secret.
    pub fn from_secret(chain: Chain, secret: &str) -> Result<Self, DatabaseError> {
        let mut keypair = Self::create_keypair(chain);
        keypair.recover_secret(secret)?;
        Ok(KeypairContext { keypair, chain })
    }

    /// Get the chain.
    pub fn chain(&self) -> Chain {
        self.chain.clone()
    }

    pub fn recover_keypair(&mut self, secret: &str) -> Result<(), DatabaseError> {
        let kp = self.keypair.as_mut();
        kp.recover_secret(secret)?;
        Ok(())
    }

    /// Gets an immutable reference to the keypair strategy.
    ///
    /// Returns an immutable reference to the underlying `KeypairStrategy` implementation.
    /// While this reference cannot be used to modify the `KeypairStrategy` itself,
    /// you can call methods on it to perform operations related to generating or using
    /// keypairs. The specific methods available depend on the exact implementation
    /// of the `KeypairStrategy` trait.
    pub fn keypair(&self) -> &Box<dyn KeypairStrategy> {
        &self.keypair
    }

    /// Sign a message with a secret key and return the signature.
    /// The secret key is not stored in the context and the keypair will not be changed.
    pub fn sign(&self, secret: &[u8], message: &[u8]) -> Result<String, DatabaseError> {
        let mut keypair = Self::create_keypair(self.chain);
        keypair.recover_from_bytes(secret)?;
        keypair.sign(message)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use solana_sdk::signature::Keypair;

    #[test]
    fn test_keypair_context() {
        let context = KeypairContext::from_chain(Chain::Solana);
        let keypair = context.keypair();
        assert_eq!(keypair.chain(), Chain::Solana);
    }

    #[test]
    fn test_keypair_context_sign() {
        let context = KeypairContext::from_chain(Chain::Solana);
        let keypair = context.keypair();
        let message = b"hello";
        let signature = keypair.sign(message).unwrap();
        assert_eq!(signature.len(), 88);
    }

    #[test]
    fn test_keypair_context_recover_keypair() {
        let keypair = Keypair::new();
        let solana_kp = SolanaKeyPair::from_secret(keypair.to_base58_string().as_str());

        {
            let mut context = KeypairContext::from_chain(Chain::Solana);
            let inner = context.keypair.as_mut();
            assert_ne!(inner.secret(), solana_kp.secret());

            inner.recover_from_bytes(keypair.to_bytes().as_slice()).unwrap();

            assert_eq!(context.keypair().secret(), solana_kp.secret());
            assert_eq!(context.keypair().secret(), keypair.to_base58_string());
        }

        {
            let mut context = KeypairContext::from_chain(Chain::Solana);
            assert_ne!(context.keypair().secret(), solana_kp.secret());

            context.recover_keypair(keypair.to_base58_string().as_str()).unwrap();

            assert_eq!(context.keypair().secret(), solana_kp.secret());
            assert_eq!(context.keypair().secret(), keypair.to_base58_string());
        }
    }

    #[test]
    fn test_keypair_context_from_secret() {
        let keypair = Keypair::new();
        let solana_kp = SolanaKeyPair::from_secret(keypair.to_base58_string().as_str());
        let context =
            KeypairContext::from_secret(Chain::Solana, keypair.to_base58_string().as_str())
                .unwrap();
        assert_eq!(context.keypair().secret(), solana_kp.secret());
        assert_eq!(context.keypair().secret(), keypair.to_base58_string());

        let context =
            KeypairContext::from_secret(Chain::Solana, solana_kp.secret().as_str()).unwrap();
        assert_eq!(context.keypair().secret(), solana_kp.secret());
        assert_eq!(context.keypair().secret(), keypair.to_base58_string());

        let mut context = KeypairContext::from_chain(Chain::Solana);
        let inner = context.keypair.as_mut();
        assert_ne!(inner.secret(), solana_kp.secret());

        inner.recover_from_bytes(keypair.to_bytes().as_slice()).unwrap();

        assert_eq!(context.keypair().secret(), solana_kp.secret());
        assert_eq!(context.keypair().secret(), keypair.to_base58_string());
    }
}
