use std::future::Future;

use tokio::signal;
use tracing::info;

pub async fn shutdown<F, R>(fut: F)
where
    F: Future<Output = R> + Send + 'static,
{
    // listen for ctrl-c
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {
            info!("Received ctrl-c, shutting down...");
        },
        _ = terminate => {
            info!("Received terminate, shutting down...");
        },
    }
    info!("signal received, starting graceful shutdown.");
    fut.await;
    info!("worker shutdown complete.");
}
