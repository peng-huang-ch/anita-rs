use crate::keypair::KeypairStrategy;
use solana_sdk::bs58;
use solana_sdk::signature::{Keypair, Signer};

pub struct SolanaKeyPair;

impl KeypairStrategy for SolanaKeyPair {
    fn generate_keypair(&self) -> (String, String, String) {
        let keypair = Keypair::new();
        let secret = keypair.to_base58_string();
        let pubkey = keypair.pubkey();
        let address = bs58::encode(pubkey).into_string();
        (secret, address.clone(), address)
    }

    fn from_secret(&self, secret: &str) -> (String, String, String) {
        let keypair = Keypair::from_base58_string(secret);
        let secret = keypair.to_base58_string();
        let pubkey = keypair.pubkey();
        let address = bs58::encode(pubkey).into_string();
        (secret, address.clone(), address)
    }

    fn sign(&self, secret: &str, message: &[u8]) -> String {
        let keypair = Keypair::from_base58_string(secret);
        let signature = keypair.sign_message(message);
        bs58::encode(signature).into_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {
        let strategy = Box::new(SolanaKeyPair);
        let (_secret, pubkey, addr) = strategy.generate_keypair();
        assert!(pubkey.eq_ignore_ascii_case(addr.as_str()));
    }
}
