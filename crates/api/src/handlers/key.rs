use crate::errors::SrvError;

use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use tracing::instrument;

use r_storage::{models::keys::get_valid_suffix_key, DbPool};

#[derive(Debug, Deserialize)]
pub struct KeyRequest {
    suffix: String,
}

#[instrument(skip(pool))]
#[get("/key")]
pub async fn get_key(
    pool: web::Data<DbPool>,
    query: web::Query<KeyRequest>,
) -> actix_web::Result<impl Responder, SrvError> {
    let suffix = query.suffix.to_ascii_lowercase();
    let mut conn = pool.get().await?;
    let key = get_valid_suffix_key(&mut conn, suffix).await?;
    Ok(HttpResponse::Ok().json(key))
}
