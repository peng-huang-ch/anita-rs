/// Common error handling for the application.
///
/// This module defines the common error types and handling mechanisms used throughout the application.
/// It provides a unified way to manage and propagate errors, making the code more maintainable and easier to debug.
///
/// ## Inspiration
///
/// This error handling approach was inspired by the following project:
/// - [LemmyNet/lemmy](https://github.com/LemmyNet/lemmy/blob/main/crates/utils/src/error.rs)
///
/// ## Usage
///
/// The main error type `SrvError` is used to represent all possible errors in the application.
/// You can use the `From` trait to convert from other error types into `SrvError`.
///
/// ```rust
/// use thiserror::Error;
///
/// #[derive(Error, Debug)]
/// pub enum SrvError {
///     #[error("An I/O error occurred: {0}")]
///     Io(#[from] std::io::Error),
///     #[error("A parsing error occurred: {0}")]
///     Parse(#[from] std::num::ParseIntError),
///
///     #[error("An unknown error occurred")]
///     Unknown,
/// }
///
/// // Example function that returns an SrvError
/// fn example_function() -> Result<(), SrvError> {
///     // Some code that may produce an I/O error
///     let _file = std::fs::File::open("non_existent_file.txt")?;
///
///     Ok(())
/// }
/// ```
use actix_web::{error::BlockingError, http::StatusCode};
use std::{
    fmt,
    fmt::{Debug, Display},
};

use r_storage::DatabaseError;
use r_tracing::SpanTrace;

pub type SrvResult<T> = Result<T, SrvError>;

// https://docs.rs/tracing-error/latest/tracing_error/
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SrvErrorKind {
    #[error("{0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    LoginError(#[from] actix_identity::error::LoginError),

    #[error("{0}")]
    IdentityError(#[from] actix_identity::error::GetIdentityError),

    #[error("invalid email or password")]
    InvalidEmailOrPassword,

    #[error("{0}")]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("the data for key {0} is not found")]
    NotFound(String),

    #[error("{1}")]
    Http(StatusCode, String),

    #[error("{0}")]
    Any(#[from] anyhow::Error),

    #[error("server is busy, try again later.  {:?}", .0)]
    BlockingError(#[from] BlockingError),

    #[error("{0}")]
    DatabaseError(#[from] DatabaseError),
}

#[derive(Debug)]
pub struct SrvError {
    pub context: SpanTrace,
    pub error_kind: SrvErrorKind,
    pub inner: anyhow::Error,
}

impl Display for SrvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: ", &self.error_kind)?;
        // print anyhow including trace
        // https://docs.rs/anyhow/latest/anyhow/struct.Error.html#display-representations
        // this will print the anyhow trace (only if it exists)
        // and if RUST_BACKTRACE=1, also a full backtrace
        writeln!(f, "{:?}", self.inner)?;
        // writeln!(f, "source {:?}", self.inner.backtrace())?;
        // print the tracing span trace.
        fmt::Display::fmt(&self.context, f)
    }
}

impl<T> From<T> for SrvError
where
    T: Into<SrvErrorKind>,
{
    fn from(t: T) -> Self {
        let into = t.into();
        SrvError {
            inner: anyhow::anyhow!("{:?}", &into),
            error_kind: into,
            context: SpanTrace::capture(),
        }
    }
}
