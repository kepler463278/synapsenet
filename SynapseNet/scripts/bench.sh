#!/usr/bin/env bash
# Benchmark script for vector index and search performance

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=== SynapseNet Benchmarks ==="
echo ""

cd "$PROJECT_ROOT"

# Build in release mode
echo "Building in release mode..."
cargo build --release --quiet

# Run benchmarks
echo ""
echo "Running benchmarks..."
cargo test --release --all -- --nocapture --test-threads=1 bench

echo ""
echo "=== Benchmark Complete ==="
