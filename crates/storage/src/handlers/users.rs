use diesel::{insert_into, prelude::*};
use diesel_async::RunQueryDsl;

use crate::{
    models::{NewUser, User},
    schema::users,
    tracing, DbConnection, DbError,
};

#[tracing::instrument(skip(conn))]
pub async fn get_user_by_id(conn: &mut DbConnection<'_>, id: i32) -> Result<Option<User>, DbError> {
    let user = users::table
        .filter(users::id.eq(id))
        .select(User::as_select())
        .first(conn)
        .await
        .optional()?;
    Ok(user)
}

#[tracing::instrument(skip(conn, doc), fields(email = %doc.email))]
pub async fn create_user(conn: &mut DbConnection<'_>, doc: &NewUser) -> Result<usize, DbError> {
    let rows_inserted = insert_into(users::table)
        .values(doc)
        .on_conflict(users::email)
        .do_nothing()
        .execute(conn)
        .await?;
    Ok(rows_inserted)
}
