.PHONY: build run test fmt clean check check-fmt clippy check-all help

build:
	cargo build --release

run:
	cargo run

test:
	cargo test --verbose

fmt:
	cargo fmt

check-fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

check: check-fmt clippy

check-all: check test build

clean:
	cargo clean

help:
	@echo "Available targets:"
	@echo "  make build      - Build the project (release mode)"
	@echo "  make run        - Run the project"
	@echo "  make test       - Run tests"
	@echo "  make fmt        - Format code with cargo fmt"
	@echo "  make check-fmt  - Check code formatting (no changes)"
	@echo "  make clippy     - Run clippy linter"
	@echo "  make check      - Run all checks (fmt + clippy)"
	@echo "  make check-all  - Run all checks + tests + build"
	@echo "  make clean      - Clean build artifacts"
	@echo "  make help       - Show this help message"
