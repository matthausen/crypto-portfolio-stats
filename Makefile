all: help


REPOSITORY=matthausen

# https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
help: ## This help
	@awk 'BEGIN {FS = ":.*?## "} /^[a-z0-9A-Z_-]+:.*?## / {printf "\033[36m%-45s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

install:
	cargo install --path .

run:
	cargo run

docker-build:
	docker build -t ${REPOSITORY}/hello-world-rust .

docker-run:
	docker run -p 9000:9000 ${REPOSITORY}/hello-world-rust

docker-push:
	docker push ${REPOSITORY}/hello-world-rust

kill-all:
	docker kill $(docker container ls -q)

.PHONY: help install run docker-build docker-push kill-all