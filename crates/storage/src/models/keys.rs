use chrono;
use diesel::{insert_into, prelude::*, update};
use diesel_async::{AsyncConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

use tracing::instrument;

use super::chain::{Chain, KeypairStrategy, Keypairs};
use crate::{schema::keys, DbConnection, DbError};

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

#[instrument(skip(conn))]
pub async fn get_key_by_suffix(
    conn: &mut DbConnection<'_>,
    chain: Chain,
    suffix: String,
) -> Result<Option<Key>, DbError> {
    let chain_str = chain.to_string();
    let result = conn
        .transaction::<Option<Key>, DbError, _>(|conn| {
            Box::pin(async move {
                let key: Option<Key> = keys::table
                    .filter(keys::used_at.is_null())
                    .filter(keys::chain.eq(chain_str))
                    .filter(keys::suffix.eq(suffix))
                    .first::<Key>(conn)
                    .await
                    .optional()?;

                if let Some(key) = key {
                    let updated_key = update(keys::table)
                        .filter(keys::id.eq(key.id))
                        .filter(keys::used_at.is_null())
                        .set(keys::used_at.eq(chrono::Utc::now().naive_utc()))
                        .get_result::<Key>(conn)
                        .await
                        .optional()?;
                    Ok(updated_key)
                } else {
                    Ok(None)
                }
            })
        })
        .await?;
    Ok(result)
}

#[instrument(skip(conn, key))]
pub async fn create_key(conn: &mut DbConnection<'_>, key: NewKey) -> Result<Key, DbError> {
    let key = insert_into(keys::table)
        .values(&key)
        .on_conflict(keys::secret)
        .do_nothing()
        .returning(Key::as_returning())
        .get_result(conn)
        .await?;
    Ok(key)
}

#[tracing::instrument(skip(conn))]
pub async fn create_keys<'a>(
    conn: &mut DbConnection<'a>,
    keys: Vec<NewKey>,
) -> Result<Vec<i32>, DbError> {
    let inserted = insert_into(keys::table)
        .values(&keys)
        .on_conflict(keys::secret)
        .do_nothing()
        .returning(keys::id)
        .get_results(conn)
        .await?;
    Ok(inserted)
}

#[instrument(skip(conn))]
pub async fn get_secret_by_id(
    conn: &mut DbConnection<'_>,
    id: i32,
) -> Result<Option<KeyWithSecret>, DbError> {
    let key = keys::table
        .filter(keys::id.eq(id))
        .select(KeyWithSecret::as_select())
        .first::<KeyWithSecret>(conn)
        .await
        .optional()?;
    Ok(key)
}
