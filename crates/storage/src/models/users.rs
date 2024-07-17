use chrono;
use diesel::{insert_into, prelude::*};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize}; // Add this line to import the chrono crate
use tracing::instrument;

use crate::{schema::users, DbConnection, DbError};

/// Key details.
#[derive(Queryable, Selectable, AsChangeset, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbUser {
    pub id: i32,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// key details.
#[derive(Insertable, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[instrument(skip(conn))]
pub async fn get_user_by_email(
    conn: &mut DbConnection<'_>,
    email: &str,
) -> Result<Option<DbUser>, DbError> {
    let user =
        users::table.filter(users::email.eq(email)).first::<DbUser>(conn).await.optional()?;
    Ok(user)
}

#[instrument(skip(conn))]
pub async fn create_user(conn: &mut DbConnection<'_>, doc: &User) -> Result<usize, DbError> {
    let rows_inserted = insert_into(users::table)
        .values(doc)
        .on_conflict(users::email)
        .do_nothing()
        .execute(conn)
        .await?;
    Ok(rows_inserted)
}
