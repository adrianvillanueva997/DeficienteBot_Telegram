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
use tracing::{error, info, warn}; // Added error for better logging
use tracing_subscriber::layer::SubscriberExt;

const SERVICE_NAME: &str = "telegrambot_deficiente";

// Centralized endpoint configuration
struct TelemetryEndpoints {
    pub otlp_grpc_endpoint: String,
}

impl TelemetryEndpoints {
    pub fn new() -> Self {
        let base_address =
            env::var("OTLP_ADDRESS").unwrap_or_else(|_| "http://localhost".to_string());

        let grpc_endpoint = if base_address.ends_with(":4317") {
            base_address
        } else if base_address.ends_with(":4318") {
            base_address.replace(":4318", ":4317")
        } else {
            format!("{base_address}:4317")
        };

        TelemetryEndpoints {
            otlp_grpc_endpoint: grpc_endpoint,
        }
    }
}

pub struct Telemetry {
    endpoints: TelemetryEndpoints,
}

impl Telemetry {
    pub fn new() -> Self {
        Telemetry {
            endpoints: TelemetryEndpoints::new(),
        }
    }

    fn setup_span_exporter(&self) -> Result<SpanExporter, ExporterBuildError> {
        SpanExporter::builder()
            .with_tonic() // Use gRPC
            .with_endpoint(&self.endpoints.otlp_grpc_endpoint) // Use gRPC endpoint
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
            .with_tonic() // Use gRPC
            .with_endpoint(&self.endpoints.otlp_grpc_endpoint) // Use gRPC endpoint, NO /v1/metrics
            .with_protocol(Protocol::Grpc)
            .with_timeout(Duration::from_secs(3))
            .build()
    }

    fn setup_logger_provider(&self) -> SdkLoggerProvider {
        let logger_exporter = LogExporter::builder()
            .with_tonic() // Use gRPC
            .with_endpoint(&self.endpoints.otlp_grpc_endpoint) // Use gRPC endpoint
            .with_compression(Compression::Gzip) // Changed to Gzip for broader compatibility
            .build();

        match logger_exporter {
            Ok(exporter) => SdkLoggerProvider::builder()
                .with_batch_exporter(exporter)
                .build(),
            Err(e) => {
                error!("Failed to build log exporter: {}", e);
                // Fallback or panic here depending on your application's error strategy
                // For now, let's return a default provider to avoid crashing
                SdkLoggerProvider::builder().build()
            }
        }
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
        // Check if OTLP_ADDRESS is set and not empty, indicating telemetry should be enabled
        if env::var("OTLP_ADDRESS").is_ok_and(|s| !s.is_empty()) {
            let tracer_provider = self.setup_tracer_provider()?;
            let metric_exporter = self.setup_metric_exporter()?;
            let logger_provider = self.setup_logger_provider();
            let meter_provider = self.setup_meter_provider(metric_exporter);

            global::set_tracer_provider(tracer_provider);
            global::set_meter_provider(meter_provider);

            let bridge = OpenTelemetryTracingBridge::new(&logger_provider);
            tracing::subscriber::set_global_default(tracing_subscriber::registry().with(bridge))
                .map_err(|e| format!("Failed to set global tracing default: {e}"))?; // Improved error handling

            info!(
                "Telemetry stack created successfully, sending to {}",
                self.endpoints.otlp_grpc_endpoint
            );
        } else {
            warn!("OTLP_ADDRESS not set or empty. Telemetry stack not created.");
        }
        Ok(())
    }
}
