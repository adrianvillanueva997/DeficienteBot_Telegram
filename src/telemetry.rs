use std::time::Duration;

use opentelemetry::{
    global::{self, tracer},
    logs::LoggerProvider,
    trace::FutureExt,
    KeyValue,
};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{
    Compression, ExporterBuildError, LogExporter, MetricExporter, Protocol, SpanExporter,
    WithExportConfig, WithTonicConfig,
};
use opentelemetry_sdk::{
    logs::{LogProcessor, LoggerProviderBuilder, SdkLoggerProvider},
    metrics::SdkMeterProvider,
    trace::{RandomIdGenerator, Sampler, SdkTracerProvider},
    Resource,
};
use tracing_subscriber::EnvFilter;

const SERVICE_NAME: &str = "telegrambot_deficiente";

fn setup_span_exporter() -> Result<SpanExporter, ExporterBuildError> {
    SpanExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4317")
        .with_timeout(Duration::from_secs(3))
        .build()
}

fn setup_tracer_provider() -> Result<SdkTracerProvider, ExporterBuildError> {
    let span_exporter = setup_span_exporter()?;
    let tracer_provider = SdkTracerProvider::builder()
        .with_batch_exporter(span_exporter)
        .with_sampler(Sampler::AlwaysOn)
        .with_id_generator(RandomIdGenerator::default())
        .with_max_events_per_span(64)
        .with_max_attributes_per_span(16)
        .with_resource(
            Resource::builder_empty()
                .with_attributes([KeyValue::new("service.name", SERVICE_NAME)])
                .build(),
        )
        .build();
    Ok(tracer_provider)
}

fn setup_metric_exporter() -> Result<MetricExporter, ExporterBuildError> {
    MetricExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4318/v1/metrics")
        .with_protocol(Protocol::Grpc)
        .with_timeout(Duration::from_secs(3))
        .build()
}

fn setup_logger_provider() -> SdkLoggerProvider {
    let logger_exporter = LogExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4318")
        .with_compression(Compression::Zstd)
        .build();
    SdkLoggerProvider::builder()
        .with_batch_exporter(logger_exporter.unwrap())
        .build()
}

fn setup_meter_provider(metric_exporter: MetricExporter) -> SdkMeterProvider {
    SdkMeterProvider::builder()
        .with_periodic_exporter(metric_exporter)
        .with_resource(
            Resource::builder_empty()
                .with_attributes([KeyValue::new("service.name", SERVICE_NAME)])
                .build(),
        )
        .build()
}

pub fn setup_opentelemetry() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let tracer_provider = setup_tracer_provider()?;
    let metric_exporter = setup_metric_exporter()?;
    let logger_provider = setup_logger_provider();

    let otel_layer = OpenTelemetryTracingBridge::new(&logger_provider);

    Ok(())
}
