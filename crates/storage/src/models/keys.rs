use chrono;
use diesel::{insert_into, prelude::*, update};
use diesel_async::{AsyncConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use super::chain::Chain;
use crate::{schema::keys, DbConnection, DbError};

/// Key details.
#[derive(Queryable, Selectable, AsChangeset, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = keys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbKey {
    pub id: i32,
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
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// key details.
#[derive(Insertable, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = keys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Key {
    #[serde(rename = "chain")]
    pub chain: String,
    #[serde(rename = "secret")]
    pub secret: String,
    #[serde(rename = "pubkey")]
    pub pubkey: String,
    #[serde(rename = "pubkey")]
    pub address: String,
    #[serde(rename = "suffix")]
    pub suffix: String,
    #[serde(rename = "usedAt")]
    pub used_at: Option<chrono::NaiveDateTime>,
}

#[instrument(skip(conn))]
pub async fn get_key_by_suffix(
    conn: &mut DbConnection<'_>,
    chain: Chain,
    suffix: String,
) -> Result<Option<DbKey>, DbError> {
    let chain_str = chain.to_string();
    let result = conn
        .transaction::<Option<DbKey>, DbError, _>(|conn| {
            Box::pin(async move {
                let key: Option<DbKey> = keys::table
                    .filter(keys::used_at.is_null())
                    .filter(keys::chain.eq(chain_str))
                    .filter(keys::suffix.eq(suffix))
                    .first::<DbKey>(conn)
                    .await
                    .optional()?;

                if let Some(key) = key {
                    let updated_key = update(keys::table)
                        .filter(keys::id.eq(key.id))
                        .filter(keys::used_at.is_null())
                        .set(keys::used_at.eq(chrono::Utc::now().naive_utc()))
                        .get_result::<DbKey>(conn)
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

#[instrument(skip(conn))]
pub async fn create_key(conn: &mut DbConnection<'_>, key: Key) -> Result<DbKey, DbError> {
    let key = insert_into(keys::table)
        .values(&key)
        .on_conflict(keys::secret)
        .do_nothing()
        .returning(DbKey::as_returning())
        .get_result(conn)
        .await?;
    Ok(key)
}

#[tracing::instrument(skip(conn))]
pub async fn create_keys<'a>(
    conn: &mut DbConnection<'a>,
    keys: Vec<Key>,
) -> Result<usize, DbError> {
    let rows_inserted = insert_into(keys::table)
        .values(&keys)
        .on_conflict(keys::secret)
        .do_nothing()
        .execute(conn)
        .await?;
    Ok(rows_inserted)
}

#[instrument(skip(conn))]
pub async fn get_key_by_id(conn: &mut DbConnection<'_>, id: i32) -> Result<Option<DbKey>, DbError> {
    let key: Option<DbKey> =
        keys::table.filter(keys::id.eq(id)).first::<DbKey>(conn).await.optional()?;
    Ok(key)
}
