# Multistage docker image building
# build-env -> dist

FROM golang:1.17.3-alpine as build-env
RUN apk add --no-cache make
WORKDIR /build
COPY go.mod .
COPY go.sum .
RUN go mod download
COPY . .
RUN make build

# Executable stage

FROM alpine:3.14.2
WORKDIR /app
COPY --from=build-env /build/app .
RUN adduser -D appuser
USER appuser
ENTRYPOINT ["./app"]
