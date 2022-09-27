# Multistage docker image building
# build-env -> dist

FROM golang:1.19.1-bullseye as build-env
RUN apt-get update && apt-get install make git
WORKDIR /build
COPY go.mod .
COPY go.sum .
RUN go mod download
COPY . .
RUN make build

# Executable stage
FROM debian:10.13-slim
WORKDIR /app
RUN adduser --disabled-password appuser
COPY --from=build-env /build/app .
USER appuser
ENTRYPOINT ["./app"]
