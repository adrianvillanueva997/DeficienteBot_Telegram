run:
	go run ./src/main.go
run-dev:
	gow run ./src/main.go
build:
	go build -o app ./src
install:
	go mod download
docker-build:
	docker build -t deficentebottelegram .
golancilint:
	golangci-lint run src/
fmt:
	go fmt ./...
sec:
	gosec ./...
docker-lint:
	hadolint --ignore DL3008 --ignore DL3015 Dockerfile
gokart:
	gokart scan ./... -v

lint: golancilint fmt sec docker-lint gokart


