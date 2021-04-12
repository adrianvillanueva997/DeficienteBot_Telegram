run:
	go run ./src/main.go

run-dev:
	gow run ./src/main.go

build:
	go build -o app ./src

docker-build:
	docker build -t deficentebottelegram .