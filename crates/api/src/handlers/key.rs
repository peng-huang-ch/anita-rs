use std::str::FromStr;

use actix_identity::Identity;
use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    info,
    storage::{Chain, KeyTrait, NewKey, UserTrait},
    tracing, Database, KeypairContext, SrvError, SrvErrorKind,
};

#[derive(Debug, Clone, Deserialize)]
pub struct SuffixKeyGenRequest {
    chain: Chain,
    suffix: String,
}

#[tracing::instrument(skip(db, identity))]
#[get("/suffix")]
pub async fn get_suffix_key(
    db: web::Data<Database>,
    query: web::Query<SuffixKeyGenRequest>,
    identity: Identity,
) -> actix_web::Result<impl Responder, SrvError> {
    let query = query.into_inner();
    let chain = query.chain;
    let suffix = query.suffix;

    let key = db.get_key_by_suffix(chain, suffix.as_str()).await?;

    if let Some(ref key) = key {
        info!("{:?} use the key {:?}", identity.id()?, key.id);
    }

    Ok(HttpResponse::Ok().json(key))
}

#[derive(Debug, Deserialize)]
pub struct KeyGenRequest {
    chain: Chain,
}

#[tracing::instrument(skip(db, identity))]
#[get("/{id}")]
pub async fn get_key(
    db: web::Data<Database>,
    path: web::Path<i32>,
    identity: Identity,
) -> actix_web::Result<impl Responder, SrvError> {
    let _identity = identity;

    let id = path.into_inner();
    let key = db.get_user_by_id(id).await?;

    Ok(HttpResponse::Ok().json(key))
}

#[tracing::instrument(skip(db, identity))]
#[post("/gen")]
pub async fn key_gen(
    db: web::Data<Database>,
    body: web::Json<KeyGenRequest>,
    identity: Identity,
) -> actix_web::Result<impl Responder, SrvError> {
    let _identity = identity;
    let chain = body.chain;

    let context = KeypairContext::from_chain(chain);
    let keypair = context.generate_keypair();
    let key = NewKey::from_keypair(keypair, None);
    let saved = db.create_key(key).await?;

    Ok(HttpResponse::Ok().json(saved))
}

#[derive(Debug, Deserialize)]
pub struct KeySignRequest {
    chain: Chain,
    pubkey: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeySignResponse {
    signature: String,
    message: String,
    pubkey: String,
}

#[tracing::instrument(skip(db, identity))]
#[post("/sign")]
pub async fn key_sign(
    identity: Identity,
    db: web::Data<Database>,
    body: web::Json<KeySignRequest>,
) -> actix_web::Result<impl Responder, SrvError> {
    let _identity = identity;
    let body = body.into_inner();

    let key = db
        .get_secret_by_pubkey(body.chain, body.pubkey.as_str())
        .await?
        .ok_or_else(|| SrvErrorKind::Http(StatusCode::BAD_REQUEST, "Key not found".to_string()))?;
    let chain = Chain::from_str(&key.key.chain)
        .map_err(|e| SrvErrorKind::Http(StatusCode::BAD_REQUEST, e.to_string()))?;

    let context = KeypairContext::from_chain(chain);

    let message = body.message.as_bytes();
    let signature = key.sign(context.keypair(), message);

    Ok(HttpResponse::Ok().json(KeySignResponse {
        signature,
        message: body.message,
        pubkey: key.key.pubkey,
    }))
}
