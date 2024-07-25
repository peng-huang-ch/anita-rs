use async_trait::async_trait;

use crate::{
    handlers::{
        keys::{create_key, get_key_by_suffix, get_secret_by_pubkey},
        users::{get_auth_by_email, get_user_by_id},
    },
    init_db,
    models::{Auth, Chain, Key, KeyWithSecret, NewKey, User},
    pg::DbPool,
    DatabaseError, DbConnection,
};

pub use crate::models::{KeyTrait, UserTrait};

#[derive(Clone, Debug)]
pub struct Database {
    pool: DbPool,
}

impl Database {
    /// Create a new database connection pool with the given pool.
    pub fn new_pool(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Create a new database connection pool with the given URL.
    pub async fn new_with_url(url: &str) -> Self {
        let db = init_db(url).await;
        Self::new_pool(db)
    }

    /// Get a connection from the pool.
    pub async fn with_conn(&self) -> Result<DbConnection<'_>, DatabaseError> {
        let pool = self.pool.get().await?;
        Ok(pool)
    }
}

#[async_trait]
impl UserTrait for Database {
    async fn get_auth_by_email(&self, email: &str) -> Result<Option<Auth>, DatabaseError> {
        let mut conn = self.with_conn().await?;
        let auth = get_auth_by_email(&mut conn, email).await?;
        Ok(auth)
    }

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

    async fn create_key(&self, key: NewKey) -> Result<Key, DatabaseError> {
        let mut conn = self.with_conn().await?;
        let key = create_key(&mut conn, key).await?;
        Ok(key)
    }

    async fn get_secret_by_pubkey(
        &self,
        chain: Chain,
        pubkey: &str,
    ) -> Result<Option<KeyWithSecret>, DatabaseError> {
        let mut conn = self.with_conn().await?;
        let secret = get_secret_by_pubkey(&mut conn, chain, pubkey.to_string()).await?;
        Ok(secret)
    }
}
