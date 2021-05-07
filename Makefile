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
lint:
	golangci-lint run src/
sec:
	gosec ./...