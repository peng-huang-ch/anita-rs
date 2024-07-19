use actix_web::{error::BlockingError, http::StatusCode};
use serde_json::json;
use std::fmt;
use tracing_error::SpanTrace;

use r_storage::{DbError, DbRunError};

#[allow(dead_code)]
pub type SrvResult<T> = Result<T, SrvError>;

// https://docs.rs/tracing-error/latest/tracing_error/
#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
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
    Custom(StatusCode, String),

    #[error("{0}")]
    Any(#[from] anyhow::Error),

    #[error("server is busy, try again later.  {:?}", .0)]
    BlockingError(#[from] BlockingError),
    #[error("database `{0}` is not available")]
    DatabaseRunError(#[from] DbRunError),
    #[error("database is not available: `{0}`")]
    DatabaseError(#[from] DbError),
}

#[derive(Debug)]
pub struct SrvError {
    pub context: SpanTrace,
    pub error_kind: SrvErrorKind,
    pub inner: anyhow::Error,
}

impl fmt::Display for SrvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: ", &self.error_kind)?;
        // print anyhow including trace
        // https://docs.rs/anyhow/latest/anyhow/struct.Error.html#display-representations
        // this will print the anyhow trace (only if it exists)
        // and if RUST_BACKTRACE=1, also a full backtrace
        writeln!(f, "{:?}", self.inner)?;
        // writeln!(f, "source {:?}", self.inner.backtrace())?;
        // print the tracing span trace
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

impl actix_web::error::ResponseError for SrvError {
    fn status_code(&self) -> StatusCode {
        match self.error_kind {
            SrvErrorKind::Custom(code, _) => code,
            SrvErrorKind::InvalidEmailOrPassword => StatusCode::BAD_REQUEST,
            SrvErrorKind::ValidationError(_) => StatusCode::BAD_REQUEST,
            SrvErrorKind::NotFound(_) => StatusCode::NOT_FOUND,
            SrvErrorKind::Any(_) => StatusCode::INTERNAL_SERVER_ERROR,
            SrvErrorKind::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let status_code = self.status_code();
        let error_response = json!({
            "success": false,
            "code": status_code.as_u16(),
            "error": status_code.canonical_reason().unwrap_or("Unknown").to_string(),
            "message": self.error_kind.to_string(),
        });
        actix_web::HttpResponse::build(self.status_code()).json(error_response)
    }
}
