use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

/// Return a tracing `Subscriber` composed of multiple layers.
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // Filter logs.
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    // Emit a Bunyan compatible formatted record.
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Initialize a `Subscriber` as global default to process span data.
///
/// # Implementation Notes
///
/// Need to be called only once.
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirect every logs events to the `Subscriber`.
    LogTracer::init().expect("Failed to initialize the log tracer");

    // Set the `Subscriber` as the global default.
    set_global_default(subscriber).expect("Failed to set the `Subscriber` as the global default");
}
