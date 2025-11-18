.PHONY: help build run test clean docker-build docker-up docker-down migrate

help:
	@echo "Available commands:"
	@echo "  make build        - Build the Rust project"
	@echo "  make run          - Run the API server"
	@echo "  make test         - Run tests"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make docker-up    - Start all services with Docker Compose"
	@echo "  make docker-down  - Stop all services"
	@echo "  make migrate      - Run database migrations"

build:
	cargo build --release

run:
	cargo run --release

test:
	cargo test

clean:
	cargo clean

docker-up:
	docker-compose up -d postgres redis prometheus grafana nginx

docker-down:
	docker-compose down

docker-build:
	docker build -t fastestapi .

migrate:
	export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/fastestapi" && \
	cargo install sqlx-cli --no-default-features --features postgres && \
	sqlx migrate run

