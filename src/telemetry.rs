use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{propagation::TraceContextPropagator, trace, trace::Tracer, Resource};

use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name.clone(), sink);

    // Create a tracing layer with the configured tracer
    let tracer = setup_otel_tracing_pipeline(name.clone());
    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());
    let opentelemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(opentelemetry_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}

fn setup_otel_tracing_pipeline(name: String) -> Tracer {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new("service.name", name)])),
        )
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(format!(
                    "{}{}",
                    std::env::var("OTEL_EXPORTER_OTLP_TRACES_ENDPOINT")
                        .unwrap_or_else(|_| "http://localhost:4317".to_string()),
                    "/v1/traces"
                )),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .expect("failed to initialize otel tracing pipeline")
}
