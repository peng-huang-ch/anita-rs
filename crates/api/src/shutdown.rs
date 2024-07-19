use std::future::Future;
use tokio::signal;

use crate::info;

pub async fn shutdown<F, R>(fut: F)
where
    F: Future<Output = R> + Send + 'static,
{
    // listen for ctrl-c
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
    };

    let stream = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(unix)]
    {
        tokio::select! {
            _ = ctrl_c => {
                info!("Received ctrl-c, shutting down...");
            },
            _ = stream => {
                info!("Received terminate, shutting down...");
            },
        }
        info!("signal received, starting graceful shutdown.");
        fut.await;
        info!("worker shutdown complete.");
    }

    #[cfg(not(unix))]
    {
        let ctrl_c = pin!(ctrl_c);
        let fut = pin!(fut);

        tokio::select! {
            _ = ctrl_c => {
                info!("Received ctrl-c, shutting down...");
            },
            _ = stream => {
                info!("Received terminate, shutting down...");
            },
        }
    }
}
