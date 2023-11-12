use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

///Compose multiple layers into a 'tracing''s subscriber
/// Implementation notes: 'impl Subscriber' is the return type, but 
/// we will also need to add 'Send' and 'Sync' to make it possible to 
/// pass the returned type to the 'init_subscriber' function later on.
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync 
    where //Sink implements 'MakeWriter' trait for all choices of the Lifetime parameter `'a`
        Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
    {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(
        name,
        sink
    );
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
    }

    ///Register a subscriber as a global default to process span data. It should only be called once.
    pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
        LogTracer::init().expect("Failed to set Logger!");
        set_global_default(subscriber).expect("Failed to set Subscriber.");
    }
