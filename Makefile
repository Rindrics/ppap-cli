.PHONY: build run test clean help

build:
	cargo build --release

run:
	cargo run

test:
	cargo test

clean:
	cargo clean

help:
	@echo "Available targets:"
	@echo "  make build  - Build the project (release mode)"
	@echo "  make run    - Run the project"
	@echo "  make test   - Run tests"
	@echo "  make clean  - Clean build artifacts"
	@echo "  make help   - Show this help message"
