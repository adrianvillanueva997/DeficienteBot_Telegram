# Multistage docker image building
# build-env -> dist

<<<<<<< HEAD
FROM golang:1.16.4-alpine as build-env
RUN apk add --no-cache make=4.3-r0
=======
FROM golang:1.16.5-alpine as build-env
RUN apk add --no-cache make
>>>>>>> 7d0d80c66ae12e3e75b2cacc9006d893f9e0104b
WORKDIR /build
COPY go.mod .
COPY go.sum .
RUN go mod download
COPY . .
RUN make build

# Executable stage

FROM alpine:3.13.5
WORKDIR /app
COPY --from=build-env /build/app .
RUN adduser -D appuser
USER appuser
ENTRYPOINT ["./app"]
