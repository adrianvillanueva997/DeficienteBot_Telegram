---
type: "Reference"
title: "Runtime and Deployment Architecture"
openwiki_generated: true
verified:
  - by: openwiki/0.9.0
    at: 2026-11-06T08:17:09.762Z
sources:
  - id: openwiki-source-e201e686a785f09b6d899f0b
    resource: repo://compose.yaml
  - id: openwiki-source-bb1ebe868e35e9e500714501
    resource: repo://Dockerfile
  - id: openwiki-source-ed8bf05e307c6278442542c2
    resource: repo://src/lib.rs
  - id: openwiki-source-b55a21a31ede1b56cd31a6a6
    resource: repo://src/main.rs
  - id: openwiki-source-9ea71732609d3446c422d5af
    resource: repo://src/telemetry.rs
generated: { by: "pi", at: "2026-11-06T08:17:09.762Z" }
---

# Runtime and Deployment Architecture

The `deficiente_telegram_bot` binary is an asynchronous Telegram bot built with Tokio and Teloxide. It exposes a webhook listener on `0.0.0.0:8080`, then hands every received update to the library's message dispatcher.

## Startup and webhook listener

`main` initializes tracing, constructs `Bot` from Teloxide's environment-based configuration, and creates an Axum webhook listener. The public webhook URL is read from the required `URL` environment variable and parsed as a URL; missing or malformed values abort startup. Teloxide's token is supplied through its standard `TELOXIDE_TOKEN` environment variable.

The listener is passed to `parse_messages`, which registers Teloxide's `repl_with_listener` callback. Each incoming message is processed independently by `handle_messages`; processing failures are logged and do not terminate the listener loop.

## Dispatch boundary

The public library accepts text messages and document messages. Text processing performs response-rule evaluation, social-link rewriting, and direct-media handling; document processing can convert qualifying `.webm` uploads. Other Telegram message types are ignored by this dispatcher.

The message-processing details and their side effects are documented in [Message Handling and Response Rules](../workflows/message-handling.md). Download and conversion behavior is documented in [Media Download and Conversion](../workflows/media-processing.md).

## Logging and configuration

`telemetry::init` installs a `tracing_subscriber` formatter. It uses `RUST_LOG` when it contains a valid filter and otherwise defaults to `info`. Instrumented functions provide tracing spans; top-level handler failures are emitted as error logs.

The runtime has no database or persistent application state. Its external runtime dependencies are Telegram's API, the configured public webhook endpoint, and—when media conversion is triggered—the `ffmpeg` executable available on the host image.

## Container and deployment

The production Dockerfile builds the release binary with Rust 1.94.1 on Debian Bookworm, copies it into a Debian slim runtime image, and runs it as a non-root `appuser`. The runtime image exposes port 8080 and installs CA certificates and OpenSSL; it does **not** install `ffmpeg`.

The root Compose file is a production-oriented Swarm-style deployment definition. It publishes the bot's internal port only for ingress, connects it to external `proxy` and `otlp` networks, and configures Traefik TLS routing for `deficiente.thexiao77.com`. It supplies `TELOXIDE_TOKEN`, `URL`, `ENVIRONMENT=prod`, and `OTLP_ADDRESS` from its environment. Tagged GitHub releases build and publish the Linux/amd64 image to GitHub Container Registry.

For local commands and environment setup, see [Deficiente Telegram Bot Quickstart](../quickstart.md).
