use actix_identity::Identity;
use actix_web::HttpRequest;
use actix_web::{http::StatusCode, web, HttpMessage, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use r_storage::{
    prelude::{hash::verify_password, users::get_user_by_email},
    DbPool,
};
use r_tracing::tracing::instrument;

use crate::errors::{SrvError, SrvErrorKind};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

#[doc = r#"API Resource: /auth/login [POST]

Login the user that matches the provided credentials to the application.

If successful, a cookie is set with the JWT token for the user. 200 Ok is returned with the token value as well.

ErrorCode::AUTH / 400 Bad Request - Invalid email or password.
ErrorCode::INTERNAL / 500 Bad Request - Any other error.
"#]
#[instrument(name = "auth login", skip(pool, body))]
#[actix_web::post("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    body: web::Json<LoginRequest>,
    request: HttpRequest,
) -> actix_web::Result<impl Responder, SrvError> {
    let mut conn = pool.get().await?;
    let body = body.into_inner();
    let user = get_user_by_email(&mut conn, &body.email).await?.ok_or_else(|| {
        SrvErrorKind::Custom(StatusCode::BAD_REQUEST, "Invalid email or password".to_string())
    })?;

    if !verify_password(body.password.as_str(), user.password.as_str()) {
        return Err(SrvError::from(SrvErrorKind::Custom(
            StatusCode::BAD_REQUEST,
            "invalid email or password".to_string(),
        )));
    }

    // Attached a verified user identity to the active session.
    Identity::login(&request.extensions(), user.id.to_string()).unwrap();
    Ok(HttpResponse::Ok().json(user))
}

#[doc = r#"API Resource: /logout [POST]

Logout the user that matches the provided credentials to the application.

"#]
#[tracing::instrument(name = "auth logout", skip(identity))]
#[actix_web::post("/logout")]
pub async fn logout(identity: Identity) -> actix_web::Result<impl Responder, SrvError> {
    identity.logout();
    Ok(HttpResponse::NoContent())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_login() {
        let app = App::new().service(login);
        let app = test::init_service(app).await;

        let req = test::TestRequest::post().uri("/login").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
