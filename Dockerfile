# Multistage docker image building
# build-env -> dist

FROM rust:1.64-slim-bullseye as build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:11.5-slim as prod
WORKDIR /app
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
COPY --from=build /app/target/release/deficiente_telegram .
RUN adduser --disabled-password appuser
USER appuser
ENV RUST_LOG=info
ENTRYPOINT [ "./deficiente_telegram" ]