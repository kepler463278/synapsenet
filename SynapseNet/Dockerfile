# Multi-stage build for SynapseNet

# Build stage
FROM rust:1.75-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libsqlite3-dev \
    cmake \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

# Build release
RUN cargo build --release --bin syn

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libsqlite3-0 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/syn /usr/local/bin/syn

# Create data directory
RUN mkdir -p /data/.synapsenet

WORKDIR /data

# Expose P2P port (future)
EXPOSE 9000

ENTRYPOINT ["syn"]
CMD ["--help"]
