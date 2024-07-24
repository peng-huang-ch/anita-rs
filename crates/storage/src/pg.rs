use diesel_async::{
    async_connection_wrapper::AsyncConnectionWrapper,
    pooled_connection::{
        bb8::{Pool, PooledConnection, RunError},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::tracing;

pub type DbPool = Pool<AsyncPgConnection>;
#[allow(dead_code)]
pub type DbConnectionManger = AsyncDieselConnectionManager<AsyncPgConnection>;
pub type DbConnection<'a> = PooledConnection<'a, AsyncPgConnection>;
pub type DbRunError = RunError;
pub type DbError = diesel::result::Error;

#[allow(dead_code)]
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[tracing::instrument(skip(database_url))]
pub async fn init_db(database_url: &str) -> DbPool {
    let mgr = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    Pool::builder().build(mgr).await.expect("could not build connection pool")
}

#[tracing::instrument(skip(database_url))]
pub async fn run_migrations(database_url: &str) {
    let pool = init_db(database_url).await;
    let async_conn = pool.dedicated_connection().await.expect("could not get connection");
    let mut async_wrapper: AsyncConnectionWrapper<AsyncPgConnection> =
        AsyncConnectionWrapper::from(async_conn);

    tokio::task::spawn_blocking(move || {
        async_wrapper.run_pending_migrations(crate::MIGRATIONS).expect("failed to run migrations");
    })
    .await
    .expect("failed to run migrations in tokio::task::spawn_blocking");
}

#[cfg(test)]
mod tests {

    use super::{init_db, run_migrations};

    #[tokio::main]
    #[test]
    #[ignore]
    async fn test_init_db() {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("Expected DATABASE_URL to be set");
        let _ = init_db(database_url.as_str()).await;
    }

    #[tokio::main]
    #[test]
    #[ignore]
    async fn test_run_migrations() {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("Expected DATABASE_URL to be set");
        let _ = run_migrations(database_url.as_str()).await;
    }

    #[tokio::main]
    #[test]
    #[ignore]
    async fn test_get_db_version() {
        use crate::prelude::get_db_version;
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("Expected DATABASE_URL to be set");
        let pool = init_db(database_url.as_str()).await;
        let mut conn = pool.get().await.expect("could not get connection");
        let version = get_db_version(&mut conn).await;
        println!("database version {}", version);
    }
}
