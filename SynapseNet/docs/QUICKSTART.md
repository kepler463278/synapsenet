# Quick Start Guide

Get SynapseNet running in 5 minutes.

## 1. Install Dependencies

### macOS
```bash
brew install rustup sqlite cmake pkg-config
rustup-init
rustup default stable
```

### Linux
```bash
sudo apt-get install build-essential pkg-config libsqlite3-dev cmake
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Windows
1. Install Rust from https://rustup.rs
2. Install Visual Studio Build Tools
3. Install SQLite and CMake

## 2. Build

```bash
git clone https://github.com/synapsenet/synapsenet.git
cd synapsenet
cargo build --release
```

## 3. Initialize Node

```bash
./target/release/syn init
```

This creates:
- `.synapsenet/node.key` - Your private key (keep safe!)
- `.synapsenet/node.pub` - Your public key
- `.synapsenet/synapsenet.db` - Local database

## 4. Add Knowledge

```bash
# Add text
./target/release/syn add "Rust is a systems programming language focused on safety and performance"

# Add file
./target/release/syn add README.md

# Add more
./target/release/syn add "Python is great for data science and machine learning"
./target/release/syn add "JavaScript runs in browsers and on servers with Node.js"
```

## 5. Query

```bash
./target/release/syn query "What is Rust?"
```

Output:
```
Found 1 results:

1. Similarity: 0.923
   ID: a1b2c3d4e5f6...
   Title: Rust is a systems programming language...
```

## 6. Try More Queries

```bash
./target/release/syn query "programming languages" --k 5
./target/release/syn query "data science"
./target/release/syn query "web development"
```

## 7. Check Status

```bash
./target/release/syn peers
```

## 8. DevNet (Multi-Node Testing)

### macOS/Linux
```bash
./scripts/devnet.sh start
```

### Windows
```powershell
.\scripts\devnet.ps1 start
```

This starts 3 local nodes. Try:

```bash
# Add to node 1
syn --data-dir .devnet/node1 add "Hello from node 1"

# Query from node 2
syn --data-dir .devnet/node2 query "Hello"
```

## Next Steps

- Read [ARCHITECTURE.md](ARCHITECTURE.md) to understand how it works
- Check [API.md](API.md) for detailed API reference
- See [ROADMAP.md](ROADMAP.md) for upcoming features
- Join the community and contribute!

## Troubleshooting

### "command not found: cargo"
Make sure Rust is installed and in your PATH:
```bash
source $HOME/.cargo/env
```

### "error: linker `cc` not found"
Install build tools:
```bash
# macOS
xcode-select --install

# Linux
sudo apt-get install build-essential
```

### "No grains in local memory"
Add some grains first with `syn add`.

## Tips

1. **Backup your keys**: Copy `.synapsenet/node.key` to a safe place
2. **Use absolute paths**: For data-dir if running from different locations
3. **Start small**: Add 10-20 grains to test, then scale up
4. **Check logs**: Use `RUST_LOG=info syn query "test"` for debug output

---

Happy knowledge sharing! 🧠✨
