.PHONY: build

build:
	cargo build
	docker-compose build
	docker-compose up

init:
	docker-compose build
	docker-compose up -d
	./scripts/init_database.sh
	docker-compose stop