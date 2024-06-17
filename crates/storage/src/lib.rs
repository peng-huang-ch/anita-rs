// pub mod book;
// pub mod database;

pub use diesel;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
pub mod models;
pub mod pg;
pub mod schema;

pub use pg::{
    init_db, run_migrations, DbConnection, DbConnectionManger, DbError, DbPool, DbRunError,
};

pub mod prelude {
    pub use crate::{
        init_db, models::version::get_db_version, run_migrations, schema::keys, DbConnection,
        DbConnectionManger, DbError, DbPool, DbRunError,
    };

    pub use diesel_async::RunQueryDsl;
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
