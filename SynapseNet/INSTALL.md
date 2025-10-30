# Installation Guide

## Prerequisites

### macOS (M2)

```bash
# Install Homebrew (if not installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install rustup sqlite cmake pkg-config

# Install Rust
rustup-init
rustup default stable
```

### Linux (Ubuntu/Debian)

```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libsqlite3-dev cmake

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable
```

### Linux (Fedora/RHEL)

```bash
# Install dependencies
sudo dnf install -y gcc gcc-c++ make pkg-config sqlite-devel cmake

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable
```

### Windows (x64)

1. **Install Rust**
   - Download from https://rustup.rs
   - Run `rustup-init.exe`
   - Follow the installer instructions

2. **Install Visual Studio Build Tools**
   - Download from https://visualstudio.microsoft.com/downloads/
   - Select "Desktop development with C++"
   - Install

3. **Install SQLite**
   - Download precompiled binaries from https://www.sqlite.org/download.html
   - Extract to `C:\sqlite`
   - Add to PATH

4. **Install CMake**
   - Download from https://cmake.org/download/
   - Install and add to PATH

## Build from Source

```bash
# Clone repository
git clone https://github.com/synapsenet/synapsenet.git
cd synapsenet

# Build
cargo build --release

# The binary will be at: target/release/syn
```

## Verify Installation

```bash
# Check Rust version
rustc --version

# Check cargo version
cargo --version

# Build and test
cargo test
```

## Quick Start

```bash
# Initialize node
./target/release/syn init

# Add knowledge
./target/release/syn add "Rust is a systems programming language"

# Query
./target/release/syn query "What is Rust?"
```

## Optional: Install Globally

### macOS/Linux

```bash
cargo install --path crates/cli
# Now you can use 'syn' from anywhere
```

### Windows

```powershell
cargo install --path crates/cli
# Add %USERPROFILE%\.cargo\bin to PATH if not already
```

## Troubleshooting

### macOS: "xcrun: error: invalid active developer path"

```bash
xcode-select --install
```

### Linux: "error: linker `cc` not found"

```bash
sudo apt-get install build-essential
```

### Windows: "link.exe not found"

Install Visual Studio Build Tools with C++ support.

### All platforms: "failed to run custom build command for `rusqlite`"

Make sure SQLite development libraries are installed:
- macOS: `brew install sqlite`
- Linux: `sudo apt-get install libsqlite3-dev`
- Windows: Download SQLite DLL and add to PATH

## Development Setup

```bash
# Install additional tools
rustup component add rustfmt clippy

# Format code
cargo fmt

# Lint
cargo clippy

# Run tests
cargo test

# Run benchmarks
./scripts/bench.sh  # macOS/Linux
.\scripts\bench.ps1  # Windows
```

## Next Steps

- Read [README.md](README.md) for usage examples
- Check [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines
- Review [SECURITY.md](SECURITY.md) for security best practices
