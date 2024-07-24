use async_trait::async_trait;
use chrono;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    models::chain::{Chain, KeypairStrategy, Keypairs},
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
    secret: String,
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
    secret: String,
}

impl KeyWithSecret {
    pub fn sign(&self, strategy: &Box<dyn KeypairStrategy>, message: &[u8]) -> String {
        strategy.sign(&self.secret, message)
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
    pub secret: String,
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
    pub fn new(
        chain: String,
        secret: String,
        pubkey: String,
        address: String,
        suffix: Option<String>,
    ) -> NewKey {
        let suffix = suffix
            .map(|f| f.to_ascii_lowercase())
            .unwrap_or_else(|| address[address.len() - 4..].to_ascii_lowercase());
        NewKey { chain, secret, pubkey, address, suffix, used_at: None }
    }

    pub fn from_keypair(keypair: Keypairs, suffix: Option<String>) -> NewKey {
        NewKey::new(
            keypair.chain.to_string(),
            keypair.secret,
            keypair.pubkey,
            keypair.address,
            suffix,
        )
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
