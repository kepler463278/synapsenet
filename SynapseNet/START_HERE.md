# 🚀 Start Here - SynapseNet Quick Setup

**New to SynapseNet? Follow these steps to get started in 10 minutes.**

## ⚡ Quick Path

### 1. Install Rust (if not installed)

**Check if you have Rust:**
```bash
rustc --version
```

**If not, install it:**

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**
- Download from https://rustup.rs
- Run installer
- Restart terminal

### 2. Install System Dependencies

**macOS:**
```bash
brew install sqlite cmake pkg-config
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get install build-essential pkg-config libsqlite3-dev cmake
```

**Windows:**
- Install Visual Studio Build Tools (C++)
- Install SQLite and CMake

### 3. Build SynapseNet

```bash
cargo build --release
```

⏱️ First build takes 5-10 minutes. Grab a coffee! ☕

### 4. Initialize & Use

```bash
# Initialize node
./target/release/syn init

# Add knowledge
./target/release/syn add "Rust is a systems programming language"

# Query
./target/release/syn query "What is Rust?"
```

## 📚 What to Read Next

Choose your path:

### 🏃 I want to use it now
→ [QUICKSTART.md](docs/QUICKSTART.md)

### 🔧 I want to understand how it works
→ [ARCHITECTURE.md](docs/ARCHITECTURE.md)

### 💻 I want to develop/contribute
→ [CONTRIBUTING.md](CONTRIBUTING.md)

### 🐳 I want to use Docker
→ [DOCKER.md](docs/DOCKER.md)

### ❓ I have questions
→ [FAQ.md](docs/FAQ.md)

### 🗺️ I want to see the roadmap
→ [ROADMAP.md](docs/ROADMAP.md)

## 🆘 Need Help?

**Installation issues?**
→ [SETUP.md](SETUP.md) - Detailed setup guide

**Build errors?**
→ [BUILD.md](BUILD.md) - Build troubleshooting

**General questions?**
→ [FAQ.md](docs/FAQ.md) - Frequently asked questions

**Still stuck?**
→ Open an issue on GitHub

## 🎯 Quick Commands Reference

```bash
# Initialize
syn init

# Add text
syn add "Your knowledge here"

# Add file
syn add document.txt

# Query (top 5 results)
syn query "Your question"

# Query (top 10 results)
syn query "Your question" --k 10

# Show peers
syn peers

# Export data
syn export --output snapshots/
```

## 🧪 Try Examples

```bash
# Basic usage
cargo run --example basic_usage

# Policy engine demo
cargo run --example policy_demo

# Proof of Emergence demo
cargo run --example poe_demo
```

## 🌐 Multi-Node Testing

```bash
# Start 3 local nodes
./scripts/devnet.sh start  # macOS/Linux
.\scripts\devnet.ps1 start  # Windows

# Use different nodes
syn --data-dir .devnet/node1 add "Hello from node 1"
syn --data-dir .devnet/node2 query "Hello"
```

## 📖 Core Concepts

**Grain** = Knowledge unit (vector + metadata + signature)

**PoE** = Proof of Emergence (reward for valuable contributions)

**NGT** = Neural Graph Token (earned through PoE)

**Policy** = Safe response system (OK/AnalysisOnly/Curated)

## 🔐 Security First

- **Backup your keys**: `.synapsenet/node.key` is your identity
- **Keep it safe**: Never share your private key
- **Report issues**: security@synapsenet.org

## 🤝 Join the Community

- **GitHub**: Star, watch, contribute
- **Issues**: Report bugs, request features
- **Discussions**: Ask questions, share ideas

## 📜 Philosophy

> "Intelligence belongs to society. The center does not exist. The owner does not exist."
> 
> — GENESIS.txt

SynapseNet is:
- ✅ Decentralized (no central server)
- ✅ Open source (MIT/Apache-2.0)
- ✅ Privacy-first (local-first design)
- ✅ Community-owned (no premine, no privileged keys)
- ✅ Safe by design (consequence-based responses)

---

**Ready to start?** Pick a guide above and dive in! 🌊

**Questions?** Check [FAQ.md](docs/FAQ.md) or open an issue.

**Want to contribute?** Read [CONTRIBUTING.md](CONTRIBUTING.md).

---

*Built with 🦀 Rust • Powered by 🧠 Emergence • Owned by 🌍 Everyone*
