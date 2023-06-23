# Multistage docker image building
# build-env -> dist

FROM golang:1.20.5-bullseye AS build-env
RUN apt-get update && \
	apt-get install -y make git && \
	apt-get clean && \
	rm -rf /var/lib/apt/lists/*
WORKDIR /build
COPY go.mod .
COPY go.sum .
RUN go mod download
COPY . .
RUN make build

# Executable stage
FROM debian:12.0-slim
RUN apt-get update && \
	apt-get install -y ca-certificates ffmpeg \
	apt-get clean && \
	rm -rf /var/lib/apt/lists/*
WORKDIR /app
RUN adduser --disabled-password appuser
COPY --from=build-env /build/app .
USER appuser
EXPOSE 2112
ENTRYPOINT ["./app"]
