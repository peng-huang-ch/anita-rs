use actix_identity::Identity;
use actix_web::HttpRequest;
use actix_web::{web, HttpMessage, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    storage::{Database, UserTrait},
    tracing, SrvError, SrvErrorKind,
};

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
#[tracing::instrument(name = "login", skip(db, body, request), fields(email = %body.email))]
#[actix_web::post("/login")]
pub async fn login(
    db: web::Data<Database>,
    body: web::Json<LoginRequest>,
    request: HttpRequest,
) -> actix_web::Result<impl Responder, SrvError> {
    let auth = db
        .get_auth_by_email(&body.email)
        .await?
        .ok_or_else(|| SrvErrorKind::InvalidEmailOrPassword)?;

    if !auth.verify_password(body.password.as_str()) {
        Err(SrvErrorKind::InvalidEmailOrPassword)?
    }

    let user = db.get_user_by_id(auth.id).await?;

    // Attached a verified user identity to the active session.
    Identity::login(&request.extensions(), auth.id.to_string())?;
    Ok(HttpResponse::Ok().json(user))
}

#[doc = r#"API Resource: /auth/logout [POST]

Logout the user that matches the provided credentials to the application.

"#]
#[tracing::instrument(name = "logout", skip(identity))]
#[actix_web::post("/logout")]
pub async fn logout(identity: Identity) -> actix_web::Result<impl Responder, SrvError> {
    identity.logout();
    Ok(HttpResponse::NoContent())
}
