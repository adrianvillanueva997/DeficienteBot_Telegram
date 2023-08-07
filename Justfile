set dotenv-load

DOCKER_IMAGE_NAME = "telegrambot_deficiente"

dev:
    @echo "Running the app"
    cargo run
test:
    @echo "Running tests"
    cargo test --verbose --all

build:
    @echo "Building the app"
    cargo build --release

audit:
    @echo "Auditing the app"
    cargo audit

lint:
    @echo "Linting the app"
    cargo clippy --all-targets --all-features -- -D warnings

clean:
    @echo "Cleaning the app"
    cargo clean

hadolint:
    @echo "Linting the dockerfile"
    hadolint Dockerfile

docker-build:
    @echo "Building the docker image"
    docker build -t $(DOCKER_IMAGE_NAME) .
