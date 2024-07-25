use rayon::prelude::*;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc,
};

use r_storage::prelude::Chain;
use r_tracing::tracing::info;

use crate::KeypairContext;

/// key blockchain generator.
///
/// # Examples
///
/// ```
/// use r_keys::{Chain, KeypairContext, keygen::keygen};
/// let num_threads = 4;
/// let target_suffix = "p";
/// let secret = keygen(num_threads, target_suffix, Chain::SOLANA);
/// let context = KeypairContext::from_chain_secret(Chain::SOLANA, secret.as_str());
/// let keypair = context.keypair();
/// assert!(keypair.pubkey().ends_with(target_suffix));
/// ```
pub fn keygen(num_threads: u8, target_suffix: &str, chain: Chain) -> String {
    info!(
        "Searching for addresses ending with {} and using {} threads",
        target_suffix, num_threads
    );
    let found = Arc::new(AtomicBool::new(false));
    let target_suffix = target_suffix.to_string();
    let (sender, receiver) = mpsc::channel();

    (0..num_threads).into_par_iter().for_each(|_| {
        let sender = sender.clone();

        while !found.load(Ordering::Relaxed) {
            let context = KeypairContext::from_chain(chain.clone());
            let keypair = context.keypair();
            let pubkey = keypair.pubkey();
            let secret = keypair.secret();
            if pubkey.ends_with(&target_suffix) {
                if sender.send(secret).is_ok() {
                    found
                        .compare_exchange(false, true, Ordering::SeqCst, Ordering::Relaxed)
                        .expect("Try to exchange the found failed");
                    break;
                }
            }
        }
    });

    // Close the sender to signal that no more keys will be sent
    drop(sender);

    // Received the secret
    receiver.recv().expect("Failed to receive keypair")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_keygen() {
        let num_threads = 4;
        let target_suffix = "p";
        let secret = keygen(num_threads, target_suffix, Chain::SOLANA);
        let context = KeypairContext::from_chain_secret(Chain::SOLANA, secret.as_str());
        let keypair = context.keypair();
        println!("secret: {}", secret);
        println!("pubkey: {}", keypair.pubkey());
        assert!(keypair.pubkey().ends_with(target_suffix));
    }
}
