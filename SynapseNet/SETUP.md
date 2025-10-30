# Complete Setup Guide for SynapseNet

This guide will walk you through setting up SynapseNet from scratch on macOS, Linux, or Windows.

## Prerequisites Check

Before starting, check if you have Rust installed:

```bash
rustc --version
cargo --version
```

If you see version numbers, skip to [Building SynapseNet](#building-synapsenet).

## Step 1: Install Rust

### macOS / Linux

Open Terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts. When done:

```bash
source $HOME/.cargo/env
rustup default stable
```

### Windows

1. Download `rustup-init.exe` from https://rustup.rs
2. Run the installer
3. Follow the prompts
4. Restart your terminal

Verify installation:

```bash
rustc --version
cargo --version
```

## Step 2: Install System Dependencies

### macOS

```bash
# Install Homebrew if not installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install sqlite cmake pkg-config
```

### Ubuntu / Debian

```bash
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libsqlite3-dev cmake git
```

### Fedora / RHEL

```bash
sudo dnf install -y gcc gcc-c++ make pkg-config sqlite-devel cmake git
```

### Windows

1. **Visual Studio Build Tools**
   - Download from https://visualstudio.microsoft.com/downloads/
   - Select "Desktop development with C++"
   - Install

2. **SQLite**
   - Download from https://www.sqlite.org/download.html
   - Extract to `C:\sqlite`
   - Add `C:\sqlite` to PATH

3. **CMake**
   - Download from https://cmake.org/download/
   - Install and add to PATH

4. **Git**
   - Download from https://git-scm.com/download/win
   - Install

## Step 3: Clone Repository

```bash
git clone https://github.com/synapsenet/synapsenet.git
cd synapsenet
```

Or download ZIP from GitHub and extract.

## Step 4: Build SynapseNet

### Debug Build (Fast compilation, for development)

```bash
cargo build
```

Binary: `target/debug/syn`

### Release Build (Optimized, for production)

```bash
cargo build --release
```

Binary: `target/release/syn`

**Note:** First build takes 5-10 minutes. Subsequent builds are faster.

## Step 5: Verify Installation

```bash
# Run tests
cargo test

# Check binary
./target/release/syn --help
```

You should see:

```
SynapseNet CLI - Decentralized semantic memory

Usage: syn [OPTIONS] <COMMAND>

Commands:
  init    Initialize local node
  add     Add text or file as grain
  query   Query semantic memory
  peers   Show peers and P2P status
  export  Export grains to Parquet
  help    Print this message or the help of the given subcommand(s)
```

## Step 6: Initialize Your Node

```bash
./target/release/syn init
```

This creates:
- `.synapsenet/node.key` - Your private key (KEEP SAFE!)
- `.synapsenet/node.pub` - Your public key
- `.synapsenet/synapsenet.db` - Local database

**IMPORTANT:** Backup `node.key` to a safe location!

## Step 7: Add Your First Grain

```bash
./target/release/syn add "Rust is a systems programming language focused on safety and performance"
```

## Step 8: Query

```bash
./target/release/syn query "What is Rust?"
```

## Step 9: (Optional) Install Globally

### macOS / Linux

```bash
cargo install --path crates/cli
```

Now you can use `syn` from anywhere:

```bash
syn add "Hello world"
syn query "Hello"
```

### Windows

```powershell
cargo install --path crates/cli
```

Make sure `%USERPROFILE%\.cargo\bin` is in your PATH.

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

### "failed to run custom build command for `rusqlite`"

Install SQLite development libraries:
- macOS: `brew install sqlite`
- Linux: `sudo apt-get install libsqlite3-dev`
- Windows: Download SQLite DLL

### Build is very slow

This is normal for the first build. Use `cargo build` (debug) for faster iteration.

### "No grains in local memory"

Add grains first with `syn add "text"`.

## Next Steps

1. **Read the docs**
   - [QUICKSTART.md](docs/QUICKSTART.md) - Quick tutorial
   - [API.md](docs/API.md) - API reference
   - [ARCHITECTURE.md](docs/ARCHITECTURE.md) - How it works

2. **Try examples**
   ```bash
   cargo run --example basic_usage
   cargo run --example policy_demo
   cargo run --example poe_demo
   ```

3. **Run DevNet**
   ```bash
   ./scripts/devnet.sh start  # macOS/Linux
   .\scripts\devnet.ps1 start  # Windows
   ```

4. **Contribute**
   - Read [CONTRIBUTING.md](CONTRIBUTING.md)
   - Check [ROADMAP.md](docs/ROADMAP.md)
   - Open issues or PRs

## Getting Help

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: Questions and ideas
- **Email**: hello@synapsenet.org

## Security

- **Backup your keys**: Copy `.synapsenet/node.key` to safe storage
- **Report vulnerabilities**: security@synapsenet.org
- **Read**: [SECURITY.md](SECURITY.md)

---

**Congratulations! You're now running SynapseNet.** 🎉

*"Intelligence belongs to society. The center does not exist."* — GENESIS.txt
