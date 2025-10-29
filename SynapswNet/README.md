# SynapseNet v0.2

**Decentralized semantic memory network with Proof of Emergence**

SynapseNet is a peer-to-peer network where humans and local AI exchange semantic knowledge grains. Intelligence belongs to the community. No center. No owner.

> **🚀 New here?** Start with [START_HERE.md](START_HERE.md) for a quick setup guide!

## What is it?

- **Grain**: Vector embedding + metadata + signature + links
- **Value**: Contribution to emergence (Proof of Emergence)
- **Risk queries**: Answered with consequences and transparency, not instructions
- **Data**: Stays local; only semantics and proofs are shared
- **No premine**: No privileged keys. Emission = contribution.

## Quick Start

### Installation

**macOS (M2)**
```bash
brew install rustup sqlite cmake pkg-config
rustup default stable
cargo build --release
```

**Linux (Ubuntu)**
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

### Usage

```bash
# Initialize local node
./target/release/syn init

# Add knowledge
./target/release/syn add "Rust is a systems programming language"
./target/release/syn add path/to/document.txt

# Query semantic memory
./target/release/syn query "What is Rust?"

# Show peers
./target/release/syn peers

# Export to Parquet (NEW in v0.2)
./target/release/syn export -o export_dir

# Import from Parquet (NEW in v0.2)
./target/release/syn import -i export_dir

# Generate configuration file (NEW in v0.2)
./target/release/syn config -o config.toml

# Show node statistics (NEW in v0.2)
./target/release/syn stats
```

## What's New in v0.2

### 🚀 ONNX Embeddings Infrastructure
- Model management system with automatic download support
- all-MiniLM-L6-v2 model integration (384-dim embeddings)
- Performance monitoring with timing metrics
- Configurable embedding parameters

### 🌐 P2P Networking (libp2p)
- Full P2P swarm with mDNS peer discovery
- Grain broadcasting with signature verification
- Distributed KNN queries across peers
- Peer reputation system with automatic bad peer disconnection
- Rate limiting (100 grains/min per peer)
- GossipSub topics: grains.put, grains.ack, query.knn, query.resp

### 📦 Parquet Export/Import
- Export grains to Parquet format with Snappy compression
- Batch processing (10,000 grains per file)
- Import with signature verification
- Efficient columnar storage for large datasets

### ⚙️ Configuration Management
- TOML-based configuration system
- Configurable P2P, AI, and storage parameters
- `syn config` command to generate default config
- Validation of all configuration values

See [CHANGELOG.md](CHANGELOG.md) for detailed changes.

```

## Architecture

```
Grain → Graph → P2P → PoE
  ↓       ↓       ↓      ↓
Vector  Links  Gossip  NGT
```

**Components:**
- **Core**: Grain, Link, Graph, PoE
- **Storage**: SQLite + HNSW index
- **P2P**: libp2p (GossipSub)
- **AI**: ONNX embeddings (CPU baseline)
- **Economy**: NGT credits
- **Governance**: Policy engine (OK/AnalysisOnly/Curated)

## Proof of Emergence (PoE)

```
NGT(g) = α * N(g) + β * C(g) + γ * log(1 + R(g))
```

Where:
- **N(g)** = Novelty (1 - max_cos_sim with existing grains)
- **C(g)** = Coherence (avg similarity to relevant clusters)
- **R(g)** = Reuse count (how often grain appears in top-k results)

Default weights: α=0.5, β=0.3, γ=0.2

**Anti-spam**: If N(g) < τ and C(g) < τ → no reward

## Safe Responses

**Policy classes:**
- **OK**: Normal response
- **AnalysisOnly**: Consequences only, no step-by-step harm instructions
- **Curated**: Queue for human review

## Platform Support

| Component | macOS (ARM64) | Linux (x86_64/ARM64) | Windows (x64) |
|-----------|---------------|----------------------|---------------|
| Rust core | ✅ | ✅ | ✅ |
| SQLite | ✅ | ✅ | ✅ |
| HNSW index | ✅ | ✅ | ✅ |
| ONNX CPU | ✅ | ✅ | ✅ |

## Roadmap

- [x] Core primitives (Grain, Link, Graph)
- [x] SQLite storage
- [x] HNSW vector index
- [x] CLI (init/add/query)
- [ ] ONNX embeddings
- [ ] P2P networking (libp2p)
- [ ] PoE calculation
- [ ] Policy engine
- [ ] DevNet scripts
- [ ] E2E tests
- [ ] GPU acceleration (Metal/CUDA/DirectML)
- [ ] Desktop UI (Tauri)

## Contributing

```bash
# Run tests
cargo test

# Run linter
cargo clippy

# Format code
cargo fmt
```

## License

MIT OR Apache-2.0

## Security

Report vulnerabilities to: security@synapsenet.org

---

*"Intelligence belongs to society. The center does not exist. The owner does not exist."* — GENESIS.txt
