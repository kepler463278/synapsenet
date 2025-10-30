# SynapseNet Makefile

.PHONY: help build test clean fmt lint run install dev docs bench devnet

help:
	@echo "SynapseNet - Makefile commands"
	@echo ""
	@echo "  make build      - Build release binary"
	@echo "  make test       - Run tests"
	@echo "  make clean      - Clean build artifacts"
	@echo "  make fmt        - Format code"
	@echo "  make lint       - Run clippy"
	@echo "  make run        - Run CLI (syn --help)"
	@echo "  make install    - Install globally"
	@echo "  make dev        - Build debug version"
	@echo "  make docs       - Generate documentation"
	@echo "  make bench      - Run benchmarks"
	@echo "  make devnet     - Start local devnet"

build:
	cargo build --release

test:
	cargo test

clean:
	cargo clean

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

run:
	cargo run --release --bin syn -- --help

install:
	cargo install --path crates/cli

dev:
	cargo build

docs:
	cargo doc --open

bench:
	./scripts/bench.sh

devnet:
	./scripts/devnet.sh start

# Quick development cycle
check:
	cargo check

watch:
	cargo watch -x check -x test

# CI commands
ci: fmt lint test build

# Release preparation
release: clean ci
	cargo build --release
	@echo "Release binary: target/release/syn"
