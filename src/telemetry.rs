#![allow(clippy::unused_self)]
use std::{env, time::Duration};

use opentelemetry::{
    global::{self},
    KeyValue,
};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{
    Compression, ExporterBuildError, LogExporter, MetricExporter, Protocol, SpanExporter,
    WithExportConfig, WithTonicConfig,
};
use opentelemetry_sdk::{
    logs::SdkLoggerProvider,
    metrics::SdkMeterProvider,
    trace::{RandomIdGenerator, Sampler, SdkTracerProvider},
    Resource,
};
use tracing::{info, warn};
use tracing_subscriber::layer::SubscriberExt;

const SERVICE_NAME: &str = "telegrambot_deficiente";

// #[derive()]
// struct TelemetryEndpoints {
//     span_endpoint: String,
//     metrics_endpoint: String,
//     logger_endpoint: String,
// }

// impl TelemetryEndpoints {
//     pub fn new() -> Self {
//         let base =
//             std::env::var("OTLP_ADDRESS").unwrap_or_else(|_| "http://localhost:4318".to_string());
//         TelemetryEndpoints {
//             span_endpoint: base.clone(),
//             metrics_endpoint: format!("{base}/v1/metrics"),
//             logger_endpoint: base,
//         }
//     }
// }

pub struct Telemetry {}

impl Telemetry {
    pub fn new() -> Self {
        Telemetry {}
    }

    fn setup_span_exporter(&self) -> Result<SpanExporter, ExporterBuildError> {
        SpanExporter::builder()
            .with_tonic()
            .with_endpoint("http://localhost:4318")
            .with_timeout(Duration::from_secs(3))
            .build()
    }

    fn setup_tracer_provider(&self) -> Result<SdkTracerProvider, ExporterBuildError> {
        let span_exporter = self.setup_span_exporter()?;
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

    fn setup_metric_exporter(&self) -> Result<MetricExporter, ExporterBuildError> {
        MetricExporter::builder()
            .with_tonic()
            .with_endpoint("http://localhost:4318/v1/metrics")
            .with_protocol(Protocol::Grpc)
            .with_timeout(Duration::from_secs(3))
            .build()
    }
    fn fetch_otel_address(&self) -> Option<String> {
        env::var("OTLP_ADDRESS").ok()
    }

    fn setup_logger_provider(&self) -> SdkLoggerProvider {
        let logger_exporter = LogExporter::builder()
            .with_tonic()
            .with_endpoint("http://localhost:4318")
            .with_compression(Compression::Zstd)
            .build();
        SdkLoggerProvider::builder()
            .with_batch_exporter(logger_exporter.unwrap())
            .build()
    }

    fn setup_meter_provider(&self, metric_exporter: MetricExporter) -> SdkMeterProvider {
        SdkMeterProvider::builder()
            .with_periodic_exporter(metric_exporter)
            .with_resource(
                Resource::builder_empty()
                    .with_attributes([KeyValue::new("service.name", SERVICE_NAME)])
                    .build(),
            )
            .build()
    }
    pub fn setup_opentelemetry(
        self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        if self.fetch_otel_address().is_some() {
            let tracer_provider = self.setup_tracer_provider()?;
            let metric_exporter = self.setup_metric_exporter()?;
            let logger_provider = self.setup_logger_provider();
            let meter_provider = self.setup_meter_provider(metric_exporter);

            global::set_tracer_provider(tracer_provider);
            global::set_meter_provider(meter_provider);

            let bridge = OpenTelemetryTracingBridge::new(&logger_provider);
            tracing::subscriber::set_global_default(tracing_subscriber::registry().with(bridge))
                .ok();

            info!("Telemetry stack created successfully");
        } else {
            warn!("Telemetry stack not created");
        }
        Ok(())
    }
}
