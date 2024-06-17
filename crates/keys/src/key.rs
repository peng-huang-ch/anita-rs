use rayon::prelude::*;
use solana_sdk::bs58;
use solana_sdk::signature::{Keypair, Signer};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc, Mutex,
};

use r_tracing::tracing::info;

/// # Examples
///
/// ```
/// use solana_sdk::signature::{Keypair,Signer};
/// use r_keys::key::generator_v1;
/// let num_threads = 4;
/// let target_suffix = "p";
/// let secret = generator_v1(num_threads, target_suffix);
/// let keypair = Keypair::from_base58_string(&secret.as_str());
/// let pubkey = keypair.pubkey().to_string();
/// assert!(pubkey.ends_with(target_suffix));
/// ```
#[allow(dead_code)]
pub fn generator_v1(num_threads: u32, target_suffix: &str) -> String {
    info!(
        "Searching for addresses ending with {} and using {} threads",
        target_suffix, num_threads
    );
    let found = Arc::new(AtomicBool::new(false));
    let result: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

    (0..num_threads).into_par_iter().for_each(|_| {
        while !found.load(Ordering::Relaxed) {
            let keypair = Keypair::new();
            let pubkey = keypair.pubkey();
            let pubkey_str = bs58::encode(pubkey).into_string();

            if pubkey_str.ends_with(target_suffix) {
                if found
                    .compare_exchange(false, true, Ordering::SeqCst, Ordering::Relaxed)
                    .is_ok()
                {
                    let mut guard = result.lock().unwrap();

                    *guard = Some(keypair.to_base58_string());
                }
            }
        }
    });
    // Extract and return the secret once found
    let secret = result.lock().unwrap();
    secret.clone().unwrap().clone()
}

/// key generator
///
/// # Examples
///
/// ```
/// use solana_sdk::signature::{Keypair,Signer};
/// use r_keys::key::generator;
/// let num_threads = 4;
/// let target_suffix = "p";
/// let secret = generator(num_threads, target_suffix);
/// let keypair = Keypair::from_base58_string(&secret.as_str());
/// let pubkey = keypair.pubkey().to_string();
/// assert!(pubkey.ends_with(target_suffix));
/// ```
#[allow(dead_code)]
pub fn generator(num_threads: u32, target_suffix: &str) -> String {
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
            let keypair = Keypair::new();
            let pubkey = keypair.pubkey();
            let pubkey_str = bs58::encode(pubkey).into_string();

            if pubkey_str.ends_with(&target_suffix) {
                if sender.send(keypair.to_base58_string()).is_ok() {
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
    let secret: String = receiver.recv().expect("Failed to receive keypair");
    secret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {
        let num_threads = 4;
        let target_suffix = "p";
        let secret = generator(num_threads, target_suffix);
        let keypair = Keypair::from_base58_string(&secret.as_str());
        let pubkey = keypair.pubkey().to_string();
        println!("Secret: {}", secret);
        println!("Pubkey: {}", pubkey);
        assert!(pubkey.ends_with(target_suffix));
    }
}
