use std::str::FromStr;

use actix_identity::Identity;
use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use r_keys::keypair::KeypairContext;
use r_storage::{
    models::{
        chain::Chain,
        keys::{create_key, get_key_by_id, get_key_by_suffix, Key},
    },
    DbPool,
};
use r_tracing::tracing::{info, instrument};

use crate::errors::{SrvError, SrvErrorKind};

#[derive(Debug, Deserialize)]
pub struct SuffixKeyGenRequest {
    chain: Chain,
    suffix: String,
}

#[instrument(skip(pool, identity))]
#[get("/suffix")]
pub async fn get_key(
    pool: web::Data<DbPool>,
    query: web::Query<SuffixKeyGenRequest>,
    identity: Identity,
) -> actix_web::Result<impl Responder, SrvError> {
    let chain = query.chain;
    let suffix = query.suffix.to_ascii_lowercase();

    let mut conn = pool.get().await?;
    let key = get_key_by_suffix(&mut conn, chain, suffix).await?;

    if let Some(ref key) = key {
        info!("{:?} use the key {:?}", identity.id(), key.id);
    }

    Ok(HttpResponse::Ok().json(key))
}

#[derive(Debug, Deserialize)]
pub struct KeyGenRequest {
    chain: Chain,
}

#[instrument(skip(pool, identity))]
#[post("/keys")]
pub async fn key_gen(
    pool: web::Data<DbPool>,
    body: web::Json<KeyGenRequest>,
    identity: Identity,
) -> actix_web::Result<impl Responder, SrvError> {
    let _identity = identity;
    let chain = body.chain;

    // gen a new key
    let context = KeypairContext::new(chain);
    let keypair = context.generate_keypair();
    let address: String = keypair.address.clone();
    let key: Key = Key {
        chain: chain.to_string(),
        secret: keypair.secret,
        pubkey: keypair.pubkey,
        address: keypair.address,
        suffix: address[address.len() - 4..].to_ascii_lowercase(),
        used_at: None,
    };
    let mut conn = pool.get().await?;
    let saved = create_key(&mut conn, key).await?;

    Ok(HttpResponse::Ok().json(saved))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeySignRequest {
    id: i32,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeySignResponse {
    signature: String,
    message: String,
    pubkey: String,
}

#[instrument(skip(pool, identity))]
#[post("/sign")]
pub async fn key_sign(
    identity: Identity,
    pool: web::Data<DbPool>,
    body: web::Json<KeySignRequest>,
) -> actix_web::Result<impl Responder, SrvError> {
    let _identity = identity;
    let body = body.into_inner();
    let mut conn = pool.get().await?;
    let key = get_key_by_id(&mut conn, body.id).await?.ok_or_else(|| {
        SrvErrorKind::Custom(StatusCode::BAD_REQUEST, "Key not found".to_string())
    })?;
    let chain = Chain::from_str(&key.chain)
        .map_err(|_| SrvErrorKind::Custom(StatusCode::BAD_REQUEST, "Invalid chain".to_string()))?;

    // sign the message with the key
    let context = KeypairContext::new(chain);
    let signature = context.sign(&key.secret, &body.message);

    Ok(HttpResponse::Ok().json(KeySignResponse {
        signature,
        message: body.message,
        pubkey: key.pubkey,
    }))
}
