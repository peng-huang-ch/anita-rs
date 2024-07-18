use actix_identity::Identity;
use actix_web::HttpRequest;
use actix_web::{web, HttpMessage, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use r_storage::{
    prelude::users::{get_auth_by_email, get_user_by_id},
    DbPool,
};
use r_tracing::tracing::instrument;

use crate::errors::{SrvError, SrvErrorKind};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
#[doc = r#"API Resource: /auth/login [POST]

Login the user that matches the provided credentials to the application.

If successful, a cookie is set with the JWT token for the user. 200 Ok is returned with the token value as well.

ErrorCode::AUTH / 400 Bad Request - invalid email or password.
ErrorCode::INTERNAL / 500 Bad Request - any other error.
"#]
#[instrument(name = "login", skip(pool, body, request), fields(email = %body.email))]
#[actix_web::post("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    body: web::Json<LoginRequest>,
    request: HttpRequest,
) -> actix_web::Result<impl Responder, SrvError> {
    let mut conn = pool.get().await?;
    let auth = get_auth_by_email(&mut conn, &body.email)
        .await?
        .ok_or_else(|| SrvErrorKind::InvalidEmailOrPassword)?;

    if !auth.verify_password(body.password.as_str()) {
        return Err(SrvErrorKind::InvalidEmailOrPassword.into());
    }

    let user = get_user_by_id(&mut conn, auth.id).await?;

    // Attached a verified user identity to the active session.
    Identity::login(&request.extensions(), auth.id.to_string())?;
    Ok(HttpResponse::Ok().json(user))
}

#[doc = r#"API Resource: /auth/logout [POST]

Logout the user that matches the provided credentials to the application.

"#]
#[tracing::instrument(name = "auth logout", skip(identity))]
#[actix_web::post("/logout")]
pub async fn logout(identity: Identity) -> actix_web::Result<impl Responder, SrvError> {
    identity.logout();
    Ok(HttpResponse::NoContent())
}
