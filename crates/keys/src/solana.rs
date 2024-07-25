use solana_sdk::bs58;
use solana_sdk::signature::{Keypair, Signer};

use crate::{Chain, DatabaseError, KeypairStrategy};

pub struct SolanaKeyPair(Keypair);

impl SolanaKeyPair {
    pub fn new() -> Self {
        SolanaKeyPair(Keypair::new())
    }

    pub fn from_secret(s: &str) -> Self {
        SolanaKeyPair(Keypair::from_base58_string(s))
    }
}

impl KeypairStrategy for SolanaKeyPair {
    fn chain(&self) -> Chain {
        Chain::SOLANA
    }

    fn generate(&mut self) {
        self.0 = Keypair::new();
    }

    fn recover_secret(&mut self, secret: &str) -> Result<(), DatabaseError> {
        let keypair = Keypair::from_base58_string(secret);
        self.0 = keypair;
        Ok(())
    }

    fn recover_from_bytes(&mut self, bytes: &[u8]) -> Result<(), DatabaseError> {
        let keypair =
            Keypair::from_bytes(bytes).map_err(|e| DatabaseError::SecretError(e.to_string()))?;
        self.0 = keypair;
        Ok(())
    }

    fn to_vec(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }

    fn secret(&self) -> String {
        self.0.to_base58_string()
    }

    fn pubkey(&self) -> String {
        self.0.pubkey().to_string()
    }

    fn address(&self) -> String {
        self.pubkey()
    }

    /// sign message with hex secret u8a
    fn sign(&self, secret: &[u8], message: &[u8]) -> Result<String, DatabaseError> {
        let keypair =
            Keypair::from_bytes(secret).map_err(|e| DatabaseError::SecretError(e.to_string()))?;
        let signature = keypair.sign_message(message);
        let signature = bs58::encode(signature).into_string();
        Ok(signature)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_generator() {
        let mut strategy = Box::new(SolanaKeyPair::new());
        let secret: String = strategy.secret();
        let pairs = Keypair::from_base58_string(secret.as_str());

        strategy.recover_from_bytes(pairs.to_bytes().as_slice()).unwrap();

        assert!(pairs.to_base58_string().eq_ignore_ascii_case(strategy.secret().as_str()));
        assert!(pairs.pubkey().to_string().eq_ignore_ascii_case(strategy.pubkey().as_str()));
        assert!(pairs.pubkey().to_string().eq_ignore_ascii_case(strategy.address().as_str()));
    }
}
