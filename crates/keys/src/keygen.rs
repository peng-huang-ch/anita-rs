use rayon::prelude::*;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc,
};

use r_storage::prelude::Chain;
use r_tracing::tracing::info;

use crate::{KeypairContext, Keypairs};

/// key blockchain generator.
///
/// # Examples
///
/// ```
/// use r_keys::{Chain, KeypairContext, Keypairs, keygen::keygen};
/// let num_threads = 4;
/// let target_suffix = "p";
/// let keypair = keygen(num_threads, target_suffix, Chain::SOLANA);
/// assert!(keypair.pubkey.ends_with(target_suffix));
/// ```
pub fn keygen(num_threads: u8, target_suffix: &str, chain: Chain) -> Keypairs {
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
            let pairs = context.generate_keypair();
            let pubkey = pairs.pubkey.clone();
            if pubkey.ends_with(&target_suffix) {
                if sender.send(pairs).is_ok() {
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
    let pairs = receiver.recv().expect("Failed to receive keypair");
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keygen() {
        let num_threads = 4;
        let target_suffix = "p";
        let pairs = keygen(num_threads, target_suffix, Chain::SOLANA);
        println!("Secret: {}", pairs.secret);
        println!("Pubkey: {}", pairs.pubkey);
        assert!(pairs.pubkey.ends_with(target_suffix));
    }
}
