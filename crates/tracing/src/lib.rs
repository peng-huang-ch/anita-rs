use opentelemetry::{global, sdk::propagation::TraceContextPropagator};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{
    filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, Layer,
};
// Re-export tracing crates
pub use tracing;
pub use tracing_error::SpanTrace;
pub use tracing_subscriber;

/// A boxed tracing [Layer].
pub type DynLayer<S> = dyn Layer<S> + Send + Sync;
pub type BoxLayer<DynLayer> = Box<DynLayer>;

/// Initializes a new [Subscriber].
pub fn init_logging(srv_name: String, level: String) -> WorkerGuard {
    // Start a new Jaeger trace pipeline.
    // Spans are exported in batch - recommended setup for a production application.
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(srv_name.clone())
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Failed to install OpenTelemetry tracer.");
    // Create a `tracing` layer using the Jaeger tracer
    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    // Filter based on level - trace, debug, info, warn, error
    // Tunable via `RUST_LOG` env variable
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));

    let file_appended = tracing_appender::rolling::daily("./logs", "api");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appended);

    // Create a `tracing` layer to emit spans as structured logs to file system
    let file_layer = BunyanFormattingLayer::new(srv_name, non_blocking);

    // Create a `tracing` layer to emit spans as structured logs to stdout
    let std_layer = fmt::layer().with_writer(std::io::stderr);

    // Combined them all together in a `tracing` subscriber
    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(telemetry_layer)
        .with(file_layer)
        .with(JsonStorageLayer)
        .with(std_layer);

    subscriber.init();
    guard
}

#[cfg(test)]
mod test {

    use super::init_logging;
    use tracing::{debug, error, info, warn};
    #[tokio::main]
    #[test]
    async fn test_init_logging() {
        let guard = init_logging("App".to_string(), "debug".to_string());
        debug!(target: "logging", "debug something...");
        info!(target: "logging", "info something...");
        warn!(target: "logging", "warn something...");
        error!(target: "logging", "error something...");
        drop(guard)
    }
}
