FROM rust:1.93.1-bookworm AS build

WORKDIR /build
RUN apt-get update && \
    apt-get install -y apt-utils pkg-config libssl-dev --no-install-recommends  && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /tmp/* /var/tmp/*
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY assets ./assets
RUN cargo build --release --locked

FROM debian:bookworm-slim AS prod
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl3 && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

COPY --from=build /build/target/release/deficiente_telegram_bot ./bot
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /app
USER appuser
ENV RUST_LOG=info
EXPOSE 8080
ENTRYPOINT ["./bot"]
