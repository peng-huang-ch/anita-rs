use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{models::Auth, schema::users, tracing, DbConnection, DbError};

#[tracing::instrument(skip(conn))]
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
