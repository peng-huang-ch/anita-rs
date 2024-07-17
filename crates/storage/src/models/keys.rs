use crate::{schema::keys, DbConnection, DbError};
use chrono;
use diesel::{insert_into, prelude::*, update};
use diesel_async::{AsyncConnection, RunQueryDsl};
use serde::{Deserialize, Serialize}; // Add this line to import the chrono crate
use tracing::instrument;

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
pub async fn get_valid_suffix_key(
    conn: &mut DbConnection<'_>,
    suffix: String,
) -> Result<Option<DbKey>, DbError> {
    let result = conn
        .transaction::<Option<DbKey>, DbError, _>(|conn| {
            Box::pin(async move {
                let key: Option<DbKey> = keys::table
                    .filter(keys::used_at.is_null())
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
pub async fn create_key(conn: &mut DbConnection<'_>, key: Key) -> Result<usize, DbError> {
    let rows_inserted = insert_into(keys::table)
        .values(&key)
        .on_conflict(keys::secret)
        .do_nothing()
        .execute(conn)
        .await?;
    Ok(rows_inserted)
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
