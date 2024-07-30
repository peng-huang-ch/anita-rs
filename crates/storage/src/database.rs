use async_trait::async_trait;

use crate::{
    handlers::{
        keys::{create_key, get_key_by_suffix, get_secret_by_pubkey},
        users::{get_auth_by_email, get_user_by_id},
    },
    init_db,
    models::{Auth, Chain, Key, KeyWithSecret, NewKey, User},
    pg::DbPool,
    tracing,
    utils::encryption::{decrypt, encrypt, to_seed},
    DatabaseError, DbConnection,
};

pub use crate::models::{KeyTrait, UserTrait};

#[derive(Clone)]
pub struct Database {
    pool: DbPool,
    seed: Option<Vec<u8>>,
}

impl Database {
    /// Create a new database connection pool with the given pool.
    pub fn new_pool(pool: DbPool, seed: Option<Vec<u8>>) -> Self {
        Self { pool, seed }
    }

    pub fn to_seed(seed: &str) -> Result<Vec<u8>, DatabaseError> {
        to_seed(seed)
    }

    /// Create a new database connection pool with the given URL.
    pub async fn new_with_url(url: &str, seed: Option<Vec<u8>>) -> Self {
        let db = init_db(url).await;
        Self::new_pool(db, seed)
    }

    /// Get a connection from the pool.
    pub async fn with_conn(&self) -> Result<DbConnection<'_>, DatabaseError> {
        let pool = self.pool.get().await?;
        Ok(pool)
    }
}

#[async_trait]
impl UserTrait for Database {
    #[tracing::instrument(skip(self))]
    async fn get_auth_by_email(&self, email: &str) -> Result<Option<Auth>, DatabaseError> {
        let mut conn = self.with_conn().await?;
        let auth = get_auth_by_email(&mut conn, email).await?;
        Ok(auth)
    }

    #[tracing::instrument(skip(self))]
    async fn get_user_by_id(&self, id: i32) -> Result<Option<User>, DatabaseError> {
        let mut conn = self.with_conn().await?;
        let user = get_user_by_id(&mut conn, id).await?;
        Ok(user)
    }
}

#[async_trait]
impl KeyTrait for Database {
    async fn get_key_by_suffix(
        &self,
        chain: Chain,
        suffix: &str,
    ) -> Result<Option<Key>, DatabaseError> {
        let mut conn = self.with_conn().await?;
        let key = get_key_by_suffix(&mut conn, chain, suffix.to_string()).await?;
        Ok(key)
    }

    /// Create a key.
    /// If the seed is set, the secret will be encrypted with the seed.
    /// Otherwise, the secret will be stored in plain text.
    async fn create_key(&self, key: NewKey) -> Result<Key, DatabaseError> {
        let mut conn = self.with_conn().await?;
        let mut key = key;
        if let Some(seed) = self.seed.clone() {
            let secret_bytes = key.get_secret();
            let encrypted = encrypt(seed.as_slice(), secret_bytes.as_slice())
                .map_err(|e| DatabaseError::SecretError(e.to_string()))?;
            key.set_secret(encrypted.as_slice());
        }
        let saved = create_key(&mut conn, key.clone()).await?;
        Ok(saved)
    }

    /// Get a key by pubkey.
    /// If the seed is set, the secret will be decrypted with the seed.
    /// Otherwise, the secret will be returned as is.
    async fn get_secret_by_pubkey(
        &self,
        chain: Chain,
        pubkey: &str,
    ) -> Result<Option<KeyWithSecret>, DatabaseError> {
        let mut conn = self.with_conn().await?;
        let mut key = get_secret_by_pubkey(&mut conn, chain, pubkey.to_string()).await?;
        if let Some(key) = key.as_mut() {
            if let Some(seed) = self.seed.clone() {
                let original = decrypt(seed.as_slice(), key.secret().as_slice())
                    .map_err(|e| DatabaseError::SecretError(e.to_string()))?;
                key.set_secret(&original);
            }
        }
        Ok(key)
    }
}
