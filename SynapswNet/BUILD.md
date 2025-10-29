# Build Instructions

## Prerequisites

Before building SynapseNet, you need to install Rust and system dependencies.

### Step 1: Install Rust

Visit https://rustup.rs and follow the instructions, or:

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable
```

**Windows:**
Download and run `rustup-init.exe` from https://rustup.rs

### Step 2: Install System Dependencies

**macOS:**
```bash
brew install sqlite cmake pkg-config
```

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libsqlite3-dev cmake
```

**Fedora/RHEL:**
```bash
sudo dnf install -y gcc gcc-c++ make pkg-config sqlite-devel cmake
```

**Windows:**
1. Install Visual Studio Build Tools with C++ support
2. Download SQLite DLL from https://www.sqlite.org/download.html
3. Install CMake from https://cmake.org/download/

### Step 3: Verify Installation

```bash
rustc --version
cargo --version
```

You should see output like:
```
rustc 1.75.0 (stable)
cargo 1.75.0
```

## Building

### Debug Build (Fast compilation, slower runtime)

```bash
cargo build
```

Binary location: `target/debug/syn`

### Release Build (Optimized for performance)

```bash
cargo build --release
```

Binary location: `target/release/syn`

**Note:** Release builds take longer to compile but run much faster.

## Testing

### Run all tests

```bash
cargo test
```

### Run specific test

```bash
cargo test test_grain_creation
```

### Run with output

```bash
cargo test -- --nocapture
```

## Code Quality

### Format code

```bash
cargo fmt
```

### Lint code

```bash
cargo clippy
```

### Check without building

```bash
cargo check
```

## Platform-Specific Notes

### macOS (M2/ARM64)

Everything should work out of the box with Homebrew dependencies.

### Linux

If you encounter linking errors, make sure you have:
```bash
sudo apt-get install build-essential pkg-config libsqlite3-dev
```

### Windows

Common issues:
1. **"link.exe not found"**: Install Visual Studio Build Tools
2. **SQLite errors**: Make sure SQLite DLL is in PATH
3. **CMake errors**: Install CMake and add to PATH

## Features

SynapseNet supports optional features:

### Default (HNSW index)

```bash
cargo build --release
```

### With FAISS (optional, requires C++ libs)

```bash
cargo build --release --features faiss
```

### With GPU support (future)

```bash
cargo build --release --features gpu-ort
```

## Cross-Compilation

### Build for different target

```bash
# List available targets
rustup target list

# Add target
rustup target add x86_64-unknown-linux-gnu

# Build for target
cargo build --release --target x86_64-unknown-linux-gnu
```

## Troubleshooting

### "error: failed to run custom build command for `rusqlite`"

Install SQLite development libraries:
- macOS: `brew install sqlite`
- Linux: `sudo apt-get install libsqlite3-dev`
- Windows: Download and install SQLite

### "error: linker `cc` not found"

Install C compiler:
- macOS: `xcode-select --install`
- Linux: `sudo apt-get install build-essential`
- Windows: Install Visual Studio Build Tools

### "error: could not find `Cargo.toml`"

Make sure you're in the project root directory.

### Slow compilation

Use `cargo build` (debug) for faster iteration during development.
Use `cargo build --release` only when you need performance.

## Development Workflow

```bash
# 1. Make changes to code
vim crates/core/src/grain.rs

# 2. Check for errors (fast)
cargo check

# 3. Run tests
cargo test

# 4. Format code
cargo fmt

# 5. Lint
cargo clippy

# 6. Build release
cargo build --release
```

## Clean Build

Remove all build artifacts:

```bash
cargo clean
```

## Documentation

Generate and open documentation:

```bash
cargo doc --open
```

## Benchmarks

Run performance benchmarks:

```bash
./scripts/bench.sh  # macOS/Linux
.\scripts\bench.ps1  # Windows
```

## Next Steps

After successful build:
1. Read [QUICKSTART.md](docs/QUICKSTART.md) for usage
2. Try the examples in [README.md](README.md)
3. Check [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines

## Getting Help

- Open an issue on GitHub
- Check existing issues for solutions
- Read the documentation in `docs/`

---

Happy building! 🦀
