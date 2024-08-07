use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpServer};
use actix_web_opentelemetry::{RequestMetrics, RequestTracing};

use shutdown::shutdown;
use std::time::Duration;
use tokio::sync::oneshot;
use tracing_actix_web::TracingLogger;

// re-export the dependencies
pub use r_errors::{SrvError, SrvErrorKind};
pub use r_keys::KeypairContext;
pub use r_tracing::{
    tracing,
    tracing::{debug, error, info, warn},
};
pub mod storage {
    pub use r_storage::prelude::*;
}
pub use storage::Database;

mod handlers;
// mod middlewares;
mod shutdown;

fn get_session_key_from_env() -> Key {
    let key = std::env::var("SECRET_KEY")
        .ok()
        .and_then(|key| bs58::decode(key).into_vec().ok())
        .and_then(|decoded| Key::try_from(decoded.as_slice()).ok())
        .map_or_else(Key::generate, |key| key);
    key
}

#[tracing::instrument(skip(database))]
pub async fn init_api(port: u16, database: Database) -> std::io::Result<()> {
    let addr = format!("0.0.0.0:{}", port);

    let session_key = get_session_key_from_env();
    let srv: actix_web::dev::Server = HttpServer::new(move || {
        let session_mw =
            SessionMiddleware::builder(CookieSessionStore::default(), session_key.clone())
                // disable secure cookie for local testing
                .cookie_secure(false)
                .build();
        App::new()
            .app_data(web::Data::new(database.clone()))
            .wrap(RequestTracing::new())
            .wrap(RequestMetrics::default())
            .wrap(TracingLogger::default())
            .wrap(IdentityMiddleware::default())
            // The identity system is built on top of sessions. You must install the session
            // middleware to leverage `actix-identity`. The session middleware must be mounted
            // AFTER the identity middleware: `actix-web` invokes middleware in the OPPOSITE
            // order of registration when it receives an incoming request.
            .wrap(session_mw)
            .service(handlers::health::get_health)
            .service(
                web::scope("/auth").service(handlers::auth::login).service(handlers::auth::logout),
            )
            .service(
                web::scope("/keys")
                    .service(handlers::key::get_suffix_key)
                    .service(handlers::key::get_key)
                    .service(handlers::key::key_gen)
                    .service(handlers::key::key_sign),
            )
    })
    .disable_signals()
    .bind(addr)?
    .run();

    let srv_handle = srv.handle();
    let server_task = tokio::spawn(srv);

    let shutdown_handle = shutdown(async move {
        srv_handle.stop(true).await;
        let (tx, rx) = oneshot::channel();
        tokio::task::spawn_blocking(|| {
            debug!("shutting down the tracer provider.");
            opentelemetry::global::shutdown_tracer_provider();
            debug!("shutdown the tracer provider.");
            let _ = tx.send(());
        })
        .await
        .expect("shutdown tracer provider failed.");

        // Wrap the future with a `Timeout` set to expire in 10 seconds.
        if tokio::time::timeout(Duration::from_secs(10), rx).await.is_err() {
            warn!("timed out while shutting down tracing, exiting anyway");
        };
    });

    let shutdown_task = tokio::spawn(shutdown_handle);
    let _ = tokio::try_join!(server_task, shutdown_task).expect("unable to join tasks");
    Ok(())
}
