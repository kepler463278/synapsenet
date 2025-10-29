# 👋 Welcome to SynapseNet!

**Decentralized semantic memory network with Proof of Emergence**

> *"Intelligence belongs to society. The center does not exist. The owner does not exist."*

## 🎯 What is This?

SynapseNet is a peer-to-peer network where humans and local AI exchange knowledge grains. Think of it as:
- 🧠 **Semantic memory** - Store and search knowledge by meaning, not keywords
- 🔐 **Cryptographically signed** - Every grain is verifiable
- 🌐 **Decentralized** - No central server, no single point of failure
- 💎 **Proof of Emergence** - Earn rewards for valuable contributions
- 🛡️ **Safe by design** - Policy engine for harmful queries

## 🚀 Quick Start (5 Minutes)

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Install Dependencies
```bash
# macOS
brew install sqlite cmake pkg-config

# Linux
sudo apt-get install build-essential pkg-config libsqlite3-dev cmake
```

### 3. Build & Run
```bash
cargo build --release
./target/release/syn init
./target/release/syn add "Rust is a systems programming language"
./target/release/syn query "What is Rust?"
```

## 📖 Where to Go Next?

### 🏃 I want to use it NOW
→ **[START_HERE.md](START_HERE.md)** - Quick setup guide

### 🤔 I want to understand it first
→ **[ARCHITECTURE.md](docs/ARCHITECTURE.md)** - How it works

### 💻 I want to develop/contribute
→ **[CONTRIBUTING.md](CONTRIBUTING.md)** - Development guide

### ❓ I have questions
→ **[FAQ.md](docs/FAQ.md)** - Frequently asked questions

### 🗺️ I want to see the roadmap
→ **[ROADMAP.md](docs/ROADMAP.md)** - Future plans

## 📁 Project Structure

```
synapsenet/
├── crates/              # Rust workspace
│   ├── core/           # Grain, Link, Graph, PoE
│   ├── storage/        # SQLite + HNSW index
│   ├── p2p/            # libp2p networking
│   ├── ai/             # Embeddings & analysis
│   ├── economy/        # NGT ledger & reputation
│   ├── governance/     # Policy engine
│   └── cli/            # Command-line interface
├── docs/               # Documentation
├── examples/           # Usage examples
├── scripts/            # DevNet & benchmarks
└── tests/              # E2E tests
```

## ✨ Key Features

### ✅ Implemented (v0.1.0-alpha)
- Core primitives (Grain, Link, Graph)
- SQLite storage + HNSW vector index
- CLI commands (init/add/query/peers/export)
- Proof of Emergence calculation
- Policy engine (safe responses)
- NGT ledger & reputation
- Cross-platform (macOS/Linux/Windows)
- Tests, examples, documentation

### 🚧 In Progress
- ONNX embeddings (v0.2)
- P2P networking (v0.2)
- REST API (v0.3)
- GPU acceleration (v0.3)
- Desktop UI (v0.5)

## 🎓 Core Concepts

**Grain** = Knowledge unit (vector + metadata + signature)
```rust
Grain {
    id: blake3(vec || meta || author_pk),
    vec: Vec<f32>,           // Embedding
    meta: GrainMeta,         // Author, timestamp, tags
    sig: Signature,          // ed25519
}
```

**PoE** = Proof of Emergence (reward formula)
```
NGT(g) = α * N(g) + β * C(g) + γ * log(1 + R(g))
```
- N(g) = Novelty
- C(g) = Coherence
- R(g) = Reuse count

**Policy** = Safe response system
- OK: Normal response
- AnalysisOnly: Consequences only, no instructions
- Curated: Human review required

## 🛠️ Quick Commands

```bash
# Initialize
syn init

# Add knowledge
syn add "Your text here"
syn add document.txt

# Query
syn query "Your question" --k 10

# Show peers
syn peers

# Export
syn export --output snapshots/
```

## 📚 Documentation Index

### Getting Started
- **[START_HERE.md](START_HERE.md)** - Quick start (10 min)
- **[SETUP.md](SETUP.md)** - Detailed setup
- **[QUICKSTART.md](docs/QUICKSTART.md)** - Tutorial

### Understanding
- **[ARCHITECTURE.md](docs/ARCHITECTURE.md)** - System design
- **[API.md](docs/API.md)** - API reference
- **[FAQ.md](docs/FAQ.md)** - Common questions

### Development
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - How to contribute
- **[BUILD.md](BUILD.md)** - Build instructions
- **[NEXT_STEPS.md](NEXT_STEPS.md)** - Development priorities

### Deployment
- **[DOCKER.md](docs/DOCKER.md)** - Docker setup
- **[INSTALL.md](INSTALL.md)** - Installation guide

### Project Info
- **[GENESIS.txt](GENESIS.txt)** - Founding principles
- **[PROJECT_STATUS.md](PROJECT_STATUS.md)** - Current status
- **[ROADMAP.md](docs/ROADMAP.md)** - Future plans
- **[CHANGELOG.md](CHANGELOG.md)** - Version history
- **[SECURITY.md](SECURITY.md)** - Security policy

## 🧪 Try Examples

```bash
# Basic usage (grains, storage, search)
cargo run --example basic_usage

# Policy engine (safe responses)
cargo run --example policy_demo

# Proof of Emergence (rewards)
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

## 🔐 Security

- **Backup your keys**: `.synapsenet/node.key` is your identity
- **Keep it safe**: Never share your private key
- **Report issues**: security@synapsenet.org

## 🤝 Community

- **GitHub**: Star, watch, contribute
- **Issues**: Report bugs, request features
- **Discussions**: Ask questions, share ideas
- **Email**: hello@synapsenet.org

## 📜 License

MIT OR Apache-2.0 - Use freely, commercially or otherwise

## 🎯 Status

**Current:** v0.1.0-alpha (January 2025)
- ✅ Core functionality working
- ⚠️ Local-only mode (P2P coming in v0.2)
- ⚠️ Dummy embeddings (ONNX coming in v0.2)
- 🚧 Not production-ready yet

**Next:** v0.2.0 (Q2 2025)
- ONNX embeddings
- Full P2P networking
- Parquet export

## 🎉 What You Get

- **8 Rust crates** - Modular architecture
- **3000+ lines** - Production-quality code
- **Complete docs** - Everything explained
- **Tests & examples** - Learn by doing
- **CI/CD** - GitHub Actions
- **Docker** - Easy deployment
- **Cross-platform** - macOS/Linux/Windows

## 🚀 Ready to Start?

1. **New user?** → [START_HERE.md](START_HERE.md)
2. **Developer?** → [CONTRIBUTING.md](CONTRIBUTING.md)
3. **Curious?** → [ARCHITECTURE.md](docs/ARCHITECTURE.md)
4. **Questions?** → [FAQ.md](docs/FAQ.md)

---

## 💡 Philosophy

SynapseNet is built on these principles:

1. **Decentralization** - No central authority
2. **Privacy** - Local-first, you own your data
3. **Emergence** - Value comes from collective intelligence
4. **Safety** - Consequences, not instructions for harm
5. **Openness** - Open source, open community

See [GENESIS.txt](GENESIS.txt) for the full vision.

---

## 🌟 Quick Links

| What | Where |
|------|-------|
| Quick Start | [START_HERE.md](START_HERE.md) |
| Tutorial | [QUICKSTART.md](docs/QUICKSTART.md) |
| Architecture | [ARCHITECTURE.md](docs/ARCHITECTURE.md) |
| API Docs | [API.md](docs/API.md) |
| FAQ | [FAQ.md](docs/FAQ.md) |
| Roadmap | [ROADMAP.md](docs/ROADMAP.md) |
| Contributing | [CONTRIBUTING.md](CONTRIBUTING.md) |
| Security | [SECURITY.md](SECURITY.md) |

---

**Let's build the future of decentralized knowledge together!** 🌍🧠✨

*Built with 🦀 Rust • Powered by 🧠 Emergence • Owned by 🌍 Everyone*
