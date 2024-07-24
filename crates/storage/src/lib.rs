pub use diesel;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
pub use r_tracing::tracing;

mod database;
mod error;
mod handlers;
mod models;
mod pg;
mod schema;
mod utils;

pub use database::Database;
pub use error::DatabaseError;
use pg::{init_db, DbConnection, DbError};

pub mod prelude {
    pub use crate::{database::Database, models::*, pg::run_migrations, utils::*, DatabaseError};
    pub use diesel_async::RunQueryDsl;
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
