---
type: guide
title: Deficiente Telegram Bot Quickstart
description: Repository-backed setup, verification, and navigation guidance for the Rust Telegram webhook bot.
tags: [quickstart, rust, telegram, operations]
verified:
  - by: openwiki/0.9.0
    at: 2026-11-06T08:17:09.762Z
sources:
  - id: openwiki-source-351da25ea7acacd549aafec8
    resource: repo://.github/workflows/main.yaml
  - id: openwiki-source-4f2678f93d3fd3835f9f2909
    resource: repo://.github/workflows/test.yml
  - id: openwiki-source-e201e686a785f09b6d899f0b
    resource: repo://compose.yaml
  - id: openwiki-source-14e6a4e1ddc06973b783ff20
    resource: repo://Justfile
  - id: openwiki-source-b55a21a31ede1b56cd31a6a6
    resource: repo://src/main.rs
generated: { by: "pi", at: "2026-11-06T08:17:09.762Z" }
---

# Deficiente Telegram Bot Quickstart

This repository builds a Rust Telegram bot that receives webhook updates, applies message-response rules, and can rewrite social links or process supported video media.

## Local prerequisites

Install a Rust toolchain compatible with the repository's 2021-edition Cargo project. Local development also requires a public HTTPS endpoint reachable by Telegram because the application registers an Axum webhook rather than polling. The README suggests a tunneling tool for this purpose.

Set the following environment before starting the bot:

- `TELOXIDE_TOKEN`: Telegram bot token consumed by `Bot::from_env`.
- `URL`: public webhook URL registered with Telegram.

Set `RUST_LOG` to choose a valid tracing filter; without it, logging defaults to `info`.

Run the application with either `cargo run` or `just dev`. The webhook listener binds to port 8080.

## Verify changes

Use the repository's Justfile commands where applicable:

```bash
just test       # cargo nextest run
just clippy     # clippy warnings are errors
just build      # release build
just audit      # cargo audit
just hadolint   # lint Dockerfile
```

The CI test workflow installs `ffmpeg` before invoking `just test`. Conversion behavior itself depends on the system `ffmpeg` executable, so install it locally before exercising media conversion.

## Build and deploy

Build the production image with `just docker-build` or `docker build`. The Dockerfile uses a multi-stage build and produces a non-root Debian runtime image. Although the test workflow installs ffmpeg, that runtime image does not; add or supply it before deploying media conversion support.

`compose.yaml` is production-oriented: it references external Traefik and OpenTelemetry networks and reads sensitive values from the host environment. Do not commit real token or URL values. Tagged GitHub pushes publish a Linux/amd64 image to GitHub Container Registry.

## Documentation map

- [Runtime and Deployment Architecture](architecture/runtime.md): webhook startup, listener lifecycle, configuration, observability, and packaging.
- [Message Handling and Response Rules](workflows/message-handling.md): response ordering, link rewriting, date-driven behavior, and tests.
- [Media Download and Conversion](workflows/media-processing.md): direct media URLs, uploaded documents, temporary files, and ffmpeg constraints.
