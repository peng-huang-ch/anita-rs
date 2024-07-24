use crate::pg::{DbError, DbRunError};

// https://docs.rs/tracing-error/latest/tracing_error/
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum DatabaseError {
    #[error("database `{0}` is not available")]
    DatabaseRunError(#[from] DbRunError),
    #[error("database is not available: `{0}`")]
    DatabaseError(#[from] DbError),
}
