# Building SynapseNet with Post-Quantum Cryptography

## Prerequisites

### System Dependencies

**macOS:**
```bash
brew install rust cmake pkg-config sqlite
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get install build-essential cmake pkg-config libsqlite3-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**
- Install Rust from https://rustup.rs
- Install Visual Studio Build Tools with C++ support
- Install CMake

## Build Options

### 1. Classical Crypto Only (Default)

```bash
cargo build --release
```

Features:
- ed25519 signatures (64 bytes)
- Noise protocol for P2P
- Fast and lightweight

### 2. Post-Quantum Crypto

```bash
cargo build --release --features pqc
```

Features:
- Dilithium5 signatures (~4595 bytes)
- Kyber1024 KEM for P2P
- Quantum-resistant

### 3. Hybrid Mode (Both)

```bash
cargo build --release --features "classical-crypto,pqc"
```

Features:
- Support both classical and PQC
- Nodes negotiate best common crypto
- Maximum compatibility

### 4. Specific PQC Algorithms

```bash
# Only Dilithium (signatures)
cargo build --release --features pqc-dilithium

# Only Kyber (key exchange)
cargo build --release --features pqc-kyber

# Both PQC algorithms
cargo build --release --features "pqc-dilithium,pqc-kyber"
```

## Build Times

| Configuration | Clean Build | Incremental |
|---------------|-------------|-------------|
| Classical only | ~2 min | ~10 sec |
| PQC only | ~3 min | ~15 sec |
| Hybrid | ~4 min | ~20 sec |

**Note**: PQC adds ~1-2 minutes to clean build time due to additional dependencies.

## Binary Sizes

| Configuration | Debug | Release |
|---------------|-------|---------|
| Classical only | ~50 MB | ~10 MB |
| PQC only | ~80 MB | ~15 MB |
| Hybrid | ~100 MB | ~18 MB |

**Note**: PQC increases binary size by ~50% due to additional crypto libraries.

## Testing

### Run All Tests

```bash
# Classical crypto tests
cargo test --features classical-crypto

# PQC tests
cargo test --features pqc

# All tests
cargo test --all-features
```

### Run Specific Test Suites

```bash
# Core crypto tests
cargo test --package synapsenet-core --features pqc

# P2P Kyber tests
cargo test --package synapsenet-p2p --features pqc-kyber

# Integration tests
cargo test --test pqc_tests --features pqc
```

## Examples

### Run PQC Demo

```bash
cargo run --example pqc_demo --features pqc
```

### Run P2P with PQC

```bash
# Terminal 1
cargo run --example p2p_demo --features pqc

# Terminal 2
cargo run --example p2p_broadcast --features pqc
```

## Benchmarks

```bash
# Benchmark signatures
cargo bench --features pqc signature

# Benchmark key exchange
cargo bench --features pqc handshake

# All benchmarks
cargo bench --all-features
```

## Cross-Compilation

### Linux → Windows

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu --features pqc
```

### macOS → Linux

```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu --features pqc
```

## Docker Build

### Classical Crypto

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/syn /usr/local/bin/
CMD ["syn"]
```

### Post-Quantum Crypto

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --features pqc

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/syn /usr/local/bin/
CMD ["syn"]
```

Build:
```bash
docker build -t synapsenet:pqc --build-arg FEATURES=pqc .
```

## Troubleshooting

### Issue: "pqcrypto-dilithium not found"

**Solution**: Update Cargo.toml dependencies:
```bash
cargo update
cargo clean
cargo build --features pqc
```

### Issue: "linking with `cc` failed"

**Solution**: Install C compiler:
```bash
# macOS
xcode-select --install

# Linux
sudo apt-get install build-essential

# Windows
# Install Visual Studio Build Tools
```

### Issue: Build takes too long

**Solution**: Use incremental compilation:
```bash
export CARGO_INCREMENTAL=1
cargo build --features pqc
```

### Issue: Out of memory during build

**Solution**: Reduce parallel jobs:
```bash
cargo build --features pqc -j 2
```

## CI/CD Integration

### GitHub Actions

```yaml
name: Build with PQC

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build with PQC
        run: cargo build --release --features pqc
      - name: Test with PQC
        run: cargo test --features pqc
```

### GitLab CI

```yaml
build-pqc:
  image: rust:1.70
  script:
    - cargo build --release --features pqc
    - cargo test --features pqc
```

## Performance Optimization

### Release Profile

Add to `Cargo.toml`:
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

### Target CPU

```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release --features pqc
```

### Link-Time Optimization

```bash
cargo build --release --features pqc
# LTO is already enabled in release profile
```

## Dependency Management

### Update Dependencies

```bash
cargo update
```

### Check for Security Advisories

```bash
cargo install cargo-audit
cargo audit
```

### Check for Outdated Dependencies

```bash
cargo install cargo-outdated
cargo outdated
```

## Feature Flag Reference

| Flag | Description | Dependencies |
|------|-------------|--------------|
| `classical-crypto` | ed25519 + Noise | ed25519-dalek |
| `pqc-dilithium` | Dilithium signatures | pqcrypto-dilithium |
| `pqc-kyber` | Kyber KEM | pqcrypto-kyber |
| `pqc` | All PQC algorithms | Both above |

## Next Steps

1. **Build**: `cargo build --features pqc`
2. **Test**: `cargo test --features pqc`
3. **Run**: `cargo run --example pqc_demo --features pqc`
4. **Deploy**: See [INSTALL.md](INSTALL.md)

## Resources

- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [pqcrypto crates](https://github.com/rustpq/pqcrypto)
- [NIST PQC](https://csrc.nist.gov/projects/post-quantum-cryptography)

---

**Questions?** Open an issue on GitHub or contact Kepler3124@proton.me
