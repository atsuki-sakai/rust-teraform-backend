.PHONY: help build run dev test check fmt lint clean docs docker-up docker-down docker-logs migrate

# Default target
help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "Development:"
	@echo "  dev          Run API server in development mode"
	@echo "  build        Build the project"
	@echo "  run          Run the API server"
	@echo "  check        Run cargo check"
	@echo "  test         Run tests"
	@echo "  fmt          Format code"
	@echo "  lint         Run clippy linter"
	@echo "  clean        Clean build artifacts"
	@echo ""
	@echo "Documentation:"
	@echo "  docs         Generate API documentation (JSON, YAML, Markdown)"
	@echo ""
	@echo "Docker:"
	@echo "  docker-up    Start all services (API, PostgreSQL, pgAdmin)"
	@echo "  docker-down  Stop all services"
	@echo "  docker-logs  View container logs"
	@echo "  db-up        Start only database services"
	@echo "  db-down      Stop database services"
	@echo ""
	@echo "Database:"
	@echo "  migrate      Run database migrations"
	@echo ""
	@echo "Services:"
	@echo "  API:         http://localhost:8080"
	@echo "  Swagger UI:  http://localhost:8080/swagger-ui"
	@echo "  pgAdmin:     http://localhost:5051"

# =============================================================================
# Development
# =============================================================================

build:
	cargo build

run:
	cargo run

dev:
	RUST_LOG=debug cargo run

dev-up: db-up
	cargo run

test:
	cargo test

check:
	cargo check

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

clean:
	cargo clean

# =============================================================================
# Documentation
# =============================================================================

docs:
	cargo run --bin generate_docs

# =============================================================================
# Docker
# =============================================================================

docker-up:
	docker compose up -d

docker-up-build:
	docker compose up -d --build

docker-down:
	docker compose down

docker-logs:
	docker compose logs -f

docker-build:
	docker compose build

db-up:
	docker compose up -d postgres pgadmin

db-down:
	docker compose down postgres pgadmin

# =============================================================================
# Database
# =============================================================================

migrate:
	sqlx migrate run

migrate-revert:
	sqlx migrate revert
