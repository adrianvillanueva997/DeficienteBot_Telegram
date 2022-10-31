docker-lint:
	hadolint --ignore DL3018 Dockerfile

lint:  docker-lint
