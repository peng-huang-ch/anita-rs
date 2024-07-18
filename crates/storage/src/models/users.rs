use chrono;
use diesel::{insert_into, prelude::*};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

use tracing::instrument;

use crate::{prelude::hash::verify_password, schema::users, DbConnection, DbError};

/// User details.
#[derive(Queryable, Selectable, AsChangeset, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Auth {
    pub id: i32,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

impl Auth {
    pub fn verify_password(&self, password: &str) -> bool {
        verify_password(password, self.password.as_str())
    }
}

/// New user details.
#[derive(Insertable, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
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
pub async fn get_auth_by_email(
    conn: &mut DbConnection<'_>,
    email: &str,
) -> Result<Option<Auth>, DbError> {
    let auth = users::table
        .filter(users::email.eq(email))
        .select(Auth::as_select())
        .first(conn)
        .await
        .optional()?;
    Ok(auth)
}

#[instrument(skip(conn))]
pub async fn get_user_by_id(conn: &mut DbConnection<'_>, id: i32) -> Result<Option<User>, DbError> {
    let user = users::table
        .filter(users::id.eq(id))
        .select(User::as_select())
        .first(conn)
        .await
        .optional()?;
    Ok(user)
}

#[instrument(skip(conn, doc), fields(email = %doc.email))]
pub async fn create_user(conn: &mut DbConnection<'_>, doc: &NewUser) -> Result<usize, DbError> {
    let rows_inserted = insert_into(users::table)
        .values(doc)
        .on_conflict(users::email)
        .do_nothing()
        .execute(conn)
        .await?;
    Ok(rows_inserted)
}
