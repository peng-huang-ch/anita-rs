use actix_web::http::StatusCode;
use serde_json::json;

use crate::{SrvError, SrvErrorKind};

impl actix_web::error::ResponseError for SrvError {
    fn status_code(&self) -> StatusCode {
        match self.error_kind {
            SrvErrorKind::Http(code, _) => code,
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
