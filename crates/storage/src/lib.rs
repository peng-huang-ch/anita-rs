pub use diesel;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

pub mod models;
mod pg;
mod schema;
mod utils;

pub use pg::{
    init_db, run_migrations, DbConnection, DbConnectionManger, DbError, DbPool, DbRunError,
};

pub mod prelude {
    pub use crate::{
        init_db, models::version::get_db_version, models::*, run_migrations, utils::*,
        DbConnection, DbConnectionManger, DbError, DbPool, DbRunError,
    };

    pub use diesel_async::RunQueryDsl;
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
