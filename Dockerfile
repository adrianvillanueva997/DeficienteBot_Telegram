# Multistage docker image building
# build-env -> dist

FROM golang:1.16.2-alpine as build-env
RUN apk add --no-cache gcc libc-dev
WORKDIR /build
COPY go.mod .
COPY go.sum .
RUN go mod download
COPY . .
RUN go build -o app ./src
# Executable stage
FROM alpine:3.13.2
WORKDIR /app
COPY --from=build-env /build/app .
RUN adduser -D appuser
USER appuser
ENTRYPOINT ./app
