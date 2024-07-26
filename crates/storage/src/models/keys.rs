use async_trait::async_trait;
use chrono;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    models::chain::{Chain, KeypairStrategy},
    schema::keys,
    DatabaseError,
};

/// Key details.
#[derive(Queryable, Selectable, AsChangeset, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = keys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Key {
    pub id: i32,
    #[serde(rename = "chain")]
    pub chain: String,
    #[serde(skip_serializing)]
    secret: Vec<u8>,
    #[serde(rename = "pubkey")]
    pub pubkey: String,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "suffix")]
    pub suffix: String,
    #[serde(rename = "usedAt")]
    pub used_at: Option<chrono::NaiveDateTime>,
    #[serde(rename = "createdAt")]
    #[diesel(skip_insertion)]
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// Key details.
#[derive(Queryable, Selectable, Deserialize)]
#[diesel(table_name = keys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct KeyWithSecret {
    #[diesel(embed)]
    #[serde(flatten)]
    pub key: Key,
    secret: Vec<u8>, // hex string
}

impl KeyWithSecret {
    /// Get the secret key.
    pub fn secret(&self) -> Vec<u8> {
        self.secret.clone()
    }

    /// set the secret key.
    pub fn set_secret(&mut self, src: &[u8]) {
        self.secret = src.to_vec();
    }

    /// Sign a message with the key pair sign method.
    pub fn sign(
        &self,
        mut keypair: Box<dyn KeypairStrategy>,
        message: &[u8],
    ) -> Result<String, DatabaseError> {
        keypair.recover_from_bytes(self.secret.as_slice())?;

        let signature = keypair.sign(message)?;
        Ok(signature)
    }
}

/// key details.
#[derive(Insertable, PartialEq, Debug, Clone, Deserialize)]
#[diesel(table_name = keys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewKey {
    #[serde(rename = "chain")]
    pub chain: String,
    #[serde(rename = "secret")]
    secret: Vec<u8>,
    #[serde(rename = "pubkey")]
    pub pubkey: String,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "suffix")]
    pub suffix: String,
    #[serde(rename = "usedAt")]
    pub used_at: Option<chrono::NaiveDateTime>,
}

impl NewKey {
    pub fn from_keypair(keypair: &Box<dyn KeypairStrategy>, suffix: Option<String>) -> NewKey {
        let address: String = keypair.address();
        let secret = keypair.to_vec();
        let suffix = suffix
            .map(|f| f.to_ascii_lowercase())
            .unwrap_or_else(|| address[address.len() - 4..].to_ascii_lowercase());
        NewKey {
            chain: keypair.chain().to_string(),
            secret,
            pubkey: keypair.pubkey(),
            address,
            suffix,
            used_at: None,
        }
    }

    pub fn get_secret(&self) -> Vec<u8> {
        self.secret.clone()
    }

    /// Get the secret key.
    /// The secret key is encrypted with the seed.
    pub fn set_secret(&mut self, encrypted: &[u8]) {
        self.secret = encrypted.to_vec();
    }
}

/// KeyTrait is an abstraction that would allow us to implement the same methods for different types of keys.
#[async_trait]
pub trait KeyTrait {
    /// Get a key by suffix and chain.
    async fn get_key_by_suffix(
        &self,
        chain: Chain,
        suffix: &str,
    ) -> Result<Option<Key>, DatabaseError>;
    /// Create a key.
    async fn create_key(&self, key: NewKey) -> Result<Key, DatabaseError>;

    // /// Create multiple keys.
    // async fn create_keys(keys: Vec<NewKey>) -> Result<Vec<i32>, DatabaseError>;

    async fn get_secret_by_pubkey(
        &self,
        chain: Chain,
        pubkey: &str,
    ) -> Result<Option<KeyWithSecret>, DatabaseError>;
}
