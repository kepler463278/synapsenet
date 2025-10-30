<div align="center">

# 🧠 SynapseNet

### Decentralized Semantic Memory Network with Proof of Emergence

**Intelligence belongs to the community. No center. No owner.**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/kepler463278/synapsenet/ci.yml?branch=main)](https://github.com/kepler463278/synapsenet/actions)

[Quick Start](#-quick-start) • [Documentation](#-documentation) • [Architecture](#-architecture) • [Roadmap](#-roadmap) • [Contributing](#-contributing)

</div>

---

## 🌟 What is SynapseNet?

SynapseNet is a **peer-to-peer network** where humans and local AI exchange semantic knowledge grains. It's a decentralized system for collective intelligence that rewards genuine contribution and emergence of new knowledge.

### The Vision

Imagine a world where:
- **Knowledge is truly decentralized** - No single entity controls the network
- **Contributors are rewarded fairly** - Proof of Emergence measures real value
- **Privacy is preserved** - Your data stays local, only semantics are shared
- **AI serves humanity** - Local AI helps you contribute, not extract from you
- **Intelligence emerges collectively** - The network gets smarter as more people participate

### Core Principles

```
🌱 Grain: Vector embedding + metadata + signature + links
💎 Value: Contribution to emergence (Proof of Emergence)
🔒 Privacy: Data stays local; only semantics and proofs are shared
⚖️ Fairness: No premine. No privileged keys. Emission = contribution
🛡️ Safety: Risk queries answered with consequences, not instructions
```

---

## 🚀 Quick Start

### Installation

**macOS**
```bash
brew install rustup sqlite cmake pkg-config
rustup default stable
cargo build --release
```

**Linux (Ubuntu/Debian)**
```bash
sudo apt-get install build-essential pkg-config libsqlite3-dev cmake
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo build --release
```

**Windows (x64)**
```powershell
# Install Rust from https://rustup.rs
# Install Visual Studio Build Tools (C++)
# Install SQLite DLL and CMake
cargo build --release
```

### Your First Steps

```bash
# 1. Initialize your local node
./target/release/syn init

# 2. Add some knowledge
./target/release/syn add "Rust is a systems programming language focused on safety and performance"

# 3. Query your semantic memory
./target/release/syn query "What is Rust?"

# 4. Check your node statistics
./target/release/syn stats
```

**That's it!** You're now part of the decentralized knowledge network. 🎉

---

## ✨ What's New in v0.2

### 🧠 ONNX Embeddings Infrastructure
- **Automatic model management** with download support
- **all-MiniLM-L6-v2** integration (384-dimensional embeddings)
- **Performance monitoring** with timing metrics and warnings
- **Configurable parameters** via TOML configuration

### 🌐 P2P Networking (libp2p)
- **Full P2P swarm** with mDNS peer discovery
- **Grain broadcasting** with cryptographic signature verification
- **Distributed KNN queries** across the network
- **Peer reputation system** with automatic bad peer disconnection
- **Rate limiting** (100 grains/min per peer) to prevent spam
- **GossipSub topics**: `grains.put`, `grains.ack`, `query.knn`, `query.resp`

### 📦 Parquet Export/Import
- **Export grains** to Parquet format with Snappy compression
- **Batch processing** (10,000 grains per file) for efficiency
- **Import with verification** - all signatures checked
- **Columnar storage** for large-scale datasets

### ⚙️ Configuration Management
- **TOML-based config** for all system parameters
- **Validation** of configuration values
- **Easy generation** with `syn config` command
- **Environment overrides** for deployment flexibility

See [CHANGELOG.md](CHANGELOG.md) for complete release notes.

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     SynapseNet Node                      │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐           │
│  │  Grain   │──▶│  Graph   │──▶│   P2P    │           │
│  │ (Vector) │   │ (Links)  │   │ (Gossip) │           │
│  └──────────┘   └──────────┘   └──────────┘           │
│       │              │               │                  │
│       ▼              ▼               ▼                  │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐           │
│  │  ONNX    │   │  HNSW    │   │   PoE    │           │
│  │Embedding │   │  Index   │   │  (NGT)   │           │
│  └──────────┘   └──────────┘   └──────────┘           │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### Components

| Component | Description | Status |
|-----------|-------------|--------|
| **Core** | Grain, Link, Graph, PoE primitives | ✅ Complete |
| **Storage** | SQLite + HNSW vector index | ✅ Complete |
| **AI** | ONNX embeddings (CPU baseline) | ✅ Complete |
| **P2P** | libp2p with GossipSub | ✅ Complete |
| **Economy** | NGT token system | 🚧 In Progress |
| **Governance** | Policy engine (OK/AnalysisOnly/Curated) | 🚧 In Progress |

---

## 💎 Proof of Emergence (PoE)

SynapseNet rewards **genuine contribution** to collective knowledge through Proof of Emergence:

```
NGT(g) = α × Novelty(g) + β × Coherence(g) + γ × log(1 + Reuse(g))
```

### How It Works

- **Novelty (N)**: How unique is this grain? `1 - max_similarity_to_existing`
- **Coherence (C)**: How well does it fit with related knowledge? `avg_similarity_to_cluster`
- **Reuse (R)**: How useful is it to others? `count_of_appearances_in_queries`

**Default weights**: α=0.5, β=0.3, γ=0.2

### Anti-Spam Protection

If a grain has both low novelty AND low coherence, it receives **no reward**. This prevents:
- Duplicate content spam
- Random noise injection
- Low-quality contributions

---

## 🛡️ Safety & Governance

### Policy Classes

SynapseNet implements a **three-tier safety system**:

| Policy | Behavior | Use Case |
|--------|----------|----------|
| **OK** | Normal response | General knowledge queries |
| **AnalysisOnly** | Consequences only, no instructions | Risk assessment queries |
| **Curated** | Queue for human review | Sensitive or harmful content |

### Example: Risk Query Handling

**Query**: "How to make explosives?"

**Traditional AI**: Step-by-step instructions ❌

**SynapseNet**: 
```
⚠️ This query involves potential harm.

Consequences:
- Legal: Manufacturing explosives without license is illegal (10-20 years)
- Safety: High risk of injury or death
- Social: Potential harm to others

If you have legitimate needs (mining, demolition), consult licensed professionals.
```

**Transparency over censorship. Consequences over instructions.**

---

## 🌍 Platform Support

| Platform | Architecture | Status |
|----------|--------------|--------|
| **macOS** | ARM64 (M1/M2/M3) | ✅ Fully Supported |
| **macOS** | x86_64 (Intel) | ✅ Fully Supported |
| **Linux** | x86_64 | ✅ Fully Supported |
| **Linux** | ARM64 | ✅ Fully Supported |
| **Windows** | x64 | ✅ Fully Supported |

All platforms support:
- ✅ Rust core
- ✅ SQLite storage
- ✅ HNSW vector index
- ✅ ONNX CPU inference
- 🚧 GPU acceleration (coming soon)

---

## 📚 Documentation

- **[START_HERE.md](START_HERE.md)** - Quick setup guide for beginners
- **[ARCHITECTURE.md](docs/ARCHITECTURE.md)** - Deep dive into system design
- **[API.md](docs/API.md)** - API reference and examples
- **[QUICKSTART.md](docs/QUICKSTART.md)** - Detailed tutorial
- **[FAQ.md](docs/FAQ.md)** - Frequently asked questions
- **[ROADMAP.md](docs/ROADMAP.md)** - Future development plans

---

## 🗺️ Roadmap

### ✅ Completed (v0.1 - v0.2)
- [x] Core primitives (Grain, Link, Graph)
- [x] SQLite storage with HNSW index
- [x] CLI (init/add/query/export/import)
- [x] ONNX embeddings infrastructure
- [x] P2P networking (libp2p)
- [x] Parquet export/import
- [x] Configuration management

### 🚧 In Progress (v0.3)
- [ ] Complete PoE calculation
- [ ] NGT token system
- [ ] Policy engine implementation
- [ ] DevNet deployment scripts
- [ ] Comprehensive E2E tests

### 🔮 Future (v0.4+)
- [ ] GPU acceleration (Metal/CUDA/DirectML)
- [ ] Desktop UI (Tauri)
- [ ] Mobile apps (iOS/Android)
- [ ] Web interface
- [ ] Advanced ML models (multilingual, multimodal)
- [ ] IPFS integration for distributed storage
- [ ] Smart contracts for NGT on blockchain
- [ ] Federation between networks

---

## 🤝 Contributing

We welcome contributions from everyone! Here's how you can help:

### Development

```bash
# Clone the repository
git clone https://github.com/kepler463278/synapsenet.git
cd synapsenet

# Run tests
cargo test

# Run linter
cargo clippy

# Format code
cargo fmt

# Build release
cargo build --release
```

### Ways to Contribute

- 🐛 **Report bugs** - Open an issue with details
- 💡 **Suggest features** - Share your ideas
- 📝 **Improve docs** - Help others understand
- 🔧 **Submit PRs** - Fix bugs or add features
- 🌍 **Translate** - Help make SynapseNet global
- 🧪 **Test** - Try it out and share feedback

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

---

## 📜 License

SynapseNet is dual-licensed under:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

You may choose either license for your use.

---

## 🔒 Security

Security is critical for a decentralized network. We take it seriously.

**Found a vulnerability?** Please report it responsibly:

- **GitHub Security Advisories**: [Report Here](https://github.com/kepler463278/synapsenet/security/advisories/new)
- **Email**: Kepler3124@proton.me (for sensitive issues)

See [SECURITY.md](SECURITY.md) for our full security policy.

---

## 📬 Contact

- **Project Lead**: Kepler3124@proton.me
- **GitHub**: [@kepler463278](https://github.com/kepler463278)
- **Issues**: [GitHub Issues](https://github.com/kepler463278/synapsenet/issues)
- **Discussions**: [GitHub Discussions](https://github.com/kepler463278/synapsenet/discussions)

---

## 🌟 Why SynapseNet Matters

In a world where:
- **Big Tech controls AI** and uses it to extract value from users
- **Knowledge is centralized** in proprietary databases
- **Privacy is sacrificed** for convenience
- **Contributors aren't rewarded** for their knowledge

**SynapseNet offers an alternative:**

✨ **Decentralized** - No single point of control or failure  
✨ **Fair** - Contributors are rewarded for genuine value  
✨ **Private** - Your data stays on your device  
✨ **Open** - Fully open-source and transparent  
✨ **Emergent** - Intelligence grows collectively  

---

## 💭 Philosophy

> *"Intelligence belongs to society. The center does not exist. The owner does not exist."*
> 
> — GENESIS.txt

SynapseNet is not just technology—it's a vision for how knowledge and intelligence can be organized in a more equitable, decentralized way. 

We believe that:
- **Knowledge should be free** but **contributors should be rewarded**
- **Privacy is a right**, not a privilege
- **AI should serve humanity**, not corporations
- **Collective intelligence** is more powerful than any single AI
- **Transparency** builds trust better than censorship

Join us in building the future of decentralized knowledge.

---

<div align="center">

**⭐ Star this repo if you believe in decentralized intelligence ⭐**

Made with ❤️ by the SynapseNet community

</div>
