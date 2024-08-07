use diesel::{sql_query, sql_types::Text, QueryableByName};

use crate::{prelude::RunQueryDsl, tracing, DbConnection};

#[derive(QueryableByName)]
pub struct SqlVersion {
    #[diesel(sql_type = Text)]
    pub version: String,
}

#[tracing::instrument(skip(conn))]
pub async fn get_db_version(conn: &mut DbConnection<'_>) -> String {
    let version = sql_query("SELECT version()")
        .get_result::<SqlVersion>(conn)
        .await
        .expect("could not get version");
    version.version
}
