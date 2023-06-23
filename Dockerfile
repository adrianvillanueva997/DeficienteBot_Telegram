# Multistage docker image building
# build-env -> dist

FROM golang:1.20.5-alpine AS build-env
RUN apk update && \
	apk add --no-cache make git
WORKDIR /build
COPY go.mod .
COPY go.sum .
RUN go mod download
COPY . .
RUN make build

# Executable stage
FROM alpine:3.18.2
RUN apk update && \
	apk add --no-cache ca-certificates ffmpeg
WORKDIR /app
COPY --from=build-env /build/app .
EXPOSE 2112
ENTRYPOINT ["./app"]
