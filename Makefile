all: help


REPOSITORY=matthausen

# https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
help: ## This help
	@awk 'BEGIN {FS = ":.*?## "} /^[a-z0-9A-Z_-]+:.*?## / {printf "\033[36m%-45s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

install:
	cargo install --path .

run:
	cargo run

test:
	cargo test

build-release:
	cargo build --release

docker-build:
	docker build -t ${REPOSITORY}/crypto-gains .

docker-run:
	docker run -p 9000:9000 ${REPOSITORY}/crypto-gains

docker-push:
	docker push ${REPOSITORY}/crypto-gains

kill-all:
	docker kill $(docker container ls -q)

.PHONY: help install run test docker-build docker-push kill-all