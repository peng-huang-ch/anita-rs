use chrono;
use diesel::{insert_into, prelude::*, update};
use diesel_async::{AsyncConnection, RunQueryDsl};

use crate::{
    models::Chain,
    models::{Key, KeyWithSecret, NewKey},
    schema::keys,
    tracing, DbConnection, DbError,
};

#[tracing::instrument(skip(conn))]
pub async fn get_key_by_suffix<'a>(
    conn: &mut DbConnection<'a>,
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

#[tracing::instrument(skip(conn, key))]
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
pub async fn create_keys(
    conn: &mut DbConnection<'_>,
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

#[tracing::instrument(skip(conn))]
pub async fn get_secret_by_pubkey(
    conn: &mut DbConnection<'_>,
    chain: Chain,
    pubkey: String,
) -> Result<Option<KeyWithSecret>, DbError> {
    let key: Option<KeyWithSecret> = keys::table
        .filter(keys::chain.eq(chain.to_string()))
        .filter(keys::pubkey.eq(pubkey))
        .select(KeyWithSecret::as_select())
        .first::<KeyWithSecret>(conn)
        .await
        .optional()?;
    Ok(key)
}
