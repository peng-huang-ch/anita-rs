use actix_web::{web, App, HttpServer};
use actix_web_opentelemetry::{RequestMetrics, RequestTracing};
use shutdown::shutdown;
use std::time::Duration;
use tokio::sync::oneshot;
use tracing::{debug, warn};
use tracing_actix_web::TracingLogger;

use r_storage::init_db;

mod errors;
mod handlers;
mod shutdown;

pub async fn init_api(port: u16, database_url: &str) -> std::io::Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    debug!(target: "init", "Initializing database...");
    let pool = init_db(database_url).await;
    debug!(target: "init", "Database initialized and listening on {}...", addr);

    let srv: actix_web::dev::Server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(RequestTracing::new())
            .wrap(RequestMetrics::default())
            .wrap(TracingLogger::default())
            .service(handlers::health::get_health)
            .service(handlers::key::get_key)
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
