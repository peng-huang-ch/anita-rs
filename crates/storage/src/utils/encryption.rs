use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Error, Key, Nonce,
};

use crate::DatabaseError;

/// This function generates a random AES-256 key.
#[allow(dead_code)]
pub fn generate_key() -> Key<Aes256Gcm> {
    Aes256Gcm::generate_key(&mut OsRng)
}

/// This function checks if a given string is a valid AES-256 key.
pub fn to_seed(key: &str) -> Result<Vec<u8>, DatabaseError> {
    let vec = hex::decode(key)?;
    let key = Key::<Aes256Gcm>::from_slice(vec.as_slice());
    Ok(key.to_vec())
}

/// This function encrypts a plaintext message using AES-256 in CBC mode with the provided initialization vector and secret key.
///
/// # Arguments
///
/// * `key` - A secret key used to encrypt the message.
/// * `text` - The plaintext message to be encrypted.
/// * Returns a vector of bytes containing the encrypted message. format: nonce + ciphertext
///
/// # Example
///
/// ```
/// use r_storage::prelude::encryption::encrypt;
///
/// let key = b"12345678123456781234567812345678"; // 32 bytes
/// let text = b"Hello, world!";
/// let ciphertext = encrypt(key, text).unwrap();
/// println!("{:?}", ciphertext);
/// ```
pub fn encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, Error> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let cipher = Aes256Gcm::new(&key);
    let encrypted = cipher.encrypt(&nonce, plaintext)?;

    // combining nonce and encrypted data together
    // for storage purpose
    let mut encrypted_data = nonce.to_vec();
    encrypted_data.extend_from_slice(&encrypted);
    Ok(encrypted_data)
}

/// This function decrypt an AES-256 encrypted message in CBC mode with the provided initialization vector and secret key.
///
/// # Arguments
///
/// * `key` - A secret key used to decrypt the message.
/// * `encryptedtext` - The ciphertext message to be decrypted. format: nonce + ciphertext
/// * Returns a vector of bytes containing the decrypted message.
/// # Example
///
/// ```
/// use r_storage::prelude::encryption::decrypt;
///
/// let key = b"12345678123456781234567812345678";
/// let ciphertext = [112, 162, 101, 119, 98, 131, 207, 227, 169, 31, 216, 125, 153, 98, 91, 229, 80, 29, 115, 37, 222, 231, 240, 31, 141, 80, 206, 131, 104, 229, 140, 195, 238, 64, 73, 20, 192, 35, 197, 115, 81];
/// let plaintext = decrypt(key, &ciphertext).unwrap();
/// println!("{:?}", ciphertext);
/// ```
pub fn decrypt(key: &[u8], encryptedtext: &[u8]) -> Result<Vec<u8>, Error> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let (nonce_arr, ciphertext) = encryptedtext.split_at(12);
    let nonce = Nonce::from_slice(nonce_arr);

    let cipher = Aes256Gcm::new(key);
    let decrypted = cipher.decrypt(nonce, ciphertext)?;
    Ok(decrypted)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = b"12345678123456781234567812345678"; // 32 bytes
        let text = b"Hello, world!";

        let encryptedtext = encrypt(&key.to_vec(), text).expect("failed to encrypt");
        let decrypted = decrypt(&key.to_vec(), &encryptedtext[..]).expect("failed to decrypt");

        assert_eq!(text.to_vec(), decrypted);
        assert_eq!("Hello, world!".to_string(), String::from_utf8(text.to_vec()).unwrap());
    }

    #[test]
    fn test_generate_key() {
        let key = generate_key();
        let text = hex::encode(key.as_slice());
        println!("{}", text);
    }
}
