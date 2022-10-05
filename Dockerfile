# Multistage docker image building
# build-env -> dist

FROM golang:1.19.2-bullseye as build-env
RUN apt-get update && apt-get install -y make git
WORKDIR /build
COPY go.mod .
COPY go.sum .
RUN go mod download
COPY . .
RUN make build

# Executable stage
FROM debian:10.1-slim
RUN apt-get update && apt-get install -y ca-certificates
WORKDIR /app
RUN adduser --disabled-password appuser
COPY --from=build-env /build/app .
USER appuser
ENTRYPOINT ["./app"]
