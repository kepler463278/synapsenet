# SynapseNet Project Structure

## Overview

```
synapsenet/
├── crates/              # Rust workspace crates
│   ├── core/           # Core primitives (Grain, Link, Graph, PoE)
│   ├── storage/        # SQLite + HNSW index
│   ├── p2p/            # libp2p networking
│   ├── ai/             # Embeddings & consequence analysis
│   ├── economy/        # NGT ledger & reputation
│   ├── governance/     # Policy engine & curation
│   ├── api/            # REST/RPC (future)
│   └── cli/            # Command-line interface (syn)
├── docs/               # Documentation
├── examples/           # Usage examples
├── scripts/            # DevNet & benchmarks
├── tests/              # E2E tests
└── [config files]      # Cargo, Docker, CI/CD, etc.
```

## Core Crates

### `crates/core/` - Core Primitives

**Purpose:** Fundamental data structures and algorithms

**Files:**
- `grain.rs` - Grain structure (vector + metadata + signature)
- `link.rs` - Semantic links between grains
- `graph.rs` - Local knowledge graph (DAG)
- `poe.rs` - Proof of Emergence calculation

**Key Types:**
- `Grain` - Atomic knowledge unit
- `Link` - Semantic connection
- `Graph` - Local graph structure
- `ProofOfEmergence` - Reward calculator
- `Credit` - NGT allocation record

### `crates/storage/` - Storage Layer

**Purpose:** Persistence and indexing

**Files:**
- `store.rs` - SQLite database operations
- `index_hnsw.rs` - HNSW vector index
- `index_faiss.rs` - FAISS index (future)

**Key Types:**
- `Store` - SQLite wrapper
- `HnswIndex` - KNN search index
- `SearchResult` - Query result

### `crates/p2p/` - P2P Networking

**Purpose:** Decentralized communication

**Files:**
- `swarm.rs` - libp2p swarm management
- `topics.rs` - GossipSub topics & messages

**Key Types:**
- `SynapseSwarm` - P2P network manager
- `GossipMessage` - Network message types
- `Topic` - Message topics

### `crates/ai/` - AI Components

**Purpose:** Embeddings and analysis

**Files:**
- `embed.rs` - Embedding model interface
- `consequence.rs` - Consequence analyzer

**Key Types:**
- `EmbeddingModel` - Trait for embeddings
- `ConsequenceAnalyzer` - Risk analysis

### `crates/economy/` - Economic System

**Purpose:** Rewards and reputation

**Files:**
- `ngt.rs` - NGT ledger
- `reputation.rs` - Reputation system

**Key Types:**
- `NgtLedger` - Token accounting
- `ReputationSystem` - Peer scoring

### `crates/governance/` - Governance

**Purpose:** Policy and curation

**Files:**
- `policy.rs` - Policy engine
- `curator.rs` - Curation queue

**Key Types:**
- `PolicyEngine` - Query classifier
- `PolicyClass` - OK/AnalysisOnly/Curated
- `CuratorQueue` - Review queue

### `crates/api/` - API Layer (Future)

**Purpose:** External interfaces

**Files:**
- `rpc.rs` - JSON-RPC server
- `rest.rs` - REST API

### `crates/cli/` - Command Line

**Purpose:** User interface

**Files:**
- `main.rs` - CLI implementation

**Commands:**
- `syn init` - Initialize node
- `syn add` - Add grain
- `syn query` - Search
- `syn peers` - Show peers
- `syn export` - Export data

## Documentation

### `docs/`

- `QUICKSTART.md` - 5-minute tutorial
- `API.md` - API reference
- `ARCHITECTURE.md` - System design
- `ROADMAP.md` - Future plans
- `DOCKER.md` - Docker deployment
- `FAQ.md` - Common questions

## Examples

### `examples/`

- `basic_usage.rs` - Core functionality demo
- `policy_demo.rs` - Policy engine demo
- `poe_demo.rs` - Proof of Emergence demo

Run with: `cargo run --example basic_usage`

## Scripts

### `scripts/`

- `devnet.sh` - Start local cluster (macOS/Linux)
- `devnet.ps1` - Start local cluster (Windows)
- `bench.sh` - Run benchmarks

## Tests

### `tests/`

- `e2e.rs` - End-to-end tests

Run with: `cargo test`

## Configuration Files

### Root Level

- `Cargo.toml` - Workspace configuration
- `rust-toolchain.toml` - Rust version
- `rustfmt.toml` - Code formatting
- `clippy.toml` - Linter config
- `.gitignore` - Git ignore rules
- `.editorconfig` - Editor settings

### Docker

- `Dockerfile` - Container image
- `docker-compose.yml` - Multi-node setup
- `.dockerignore` - Docker ignore rules

### CI/CD

- `.github/workflows/ci.yml` - GitHub Actions
- `.github/ISSUE_TEMPLATE/` - Issue templates
- `.github/pull_request_template.md` - PR template

### Build

- `Makefile` - Build shortcuts
- `.cargo/config.toml` - Cargo settings

## Documentation Files

### Getting Started

- `START_HERE.md` - Quick start guide
- `SETUP.md` - Detailed setup
- `INSTALL.md` - Installation guide
- `BUILD.md` - Build instructions

### Project Info

- `README.md` - Main readme (English)
- `README.ru.md` - Russian readme
- `GENESIS.txt` - Founding principles
- `PROJECT_STATUS.md` - Current status
- `CHANGELOG.md` - Version history
- `ROADMAP.md` - Future plans (in docs/)

### Contributing

- `CONTRIBUTING.md` - Contribution guide
- `SECURITY.md` - Security policy

### Legal

- `LICENSE-MIT` - MIT license
- `LICENSE-APACHE` - Apache 2.0 license

## Data Directories (Created at Runtime)

### `.synapsenet/` (Default)

- `node.key` - Private key (ed25519)
- `node.pub` - Public key
- `synapsenet.db` - SQLite database

### `.devnet/` (DevNet)

- `node1/`, `node2/`, `node3/` - Test nodes

## Key Dependencies

### Core
- `ed25519-dalek` - Signatures
- `blake3` - Hashing
- `serde` - Serialization

### Storage
- `rusqlite` - SQLite
- `hnsw_rs` - Vector index

### P2P
- `libp2p` - Networking
- `tokio` - Async runtime

### CLI
- `clap` - Argument parsing
- `tracing` - Logging

## Build Artifacts (Ignored)

- `target/` - Cargo build output
- `Cargo.lock` - Dependency lock file
- `*.db` - Database files
- `*.log` - Log files

## Development Workflow

1. **Edit code** in `crates/*/src/`
2. **Run tests**: `cargo test`
3. **Format**: `cargo fmt`
4. **Lint**: `cargo clippy`
5. **Build**: `cargo build --release`
6. **Run**: `./target/release/syn`

## Adding New Features

### New Core Type

1. Add to `crates/core/src/`
2. Export in `crates/core/src/lib.rs`
3. Add tests
4. Update docs

### New CLI Command

1. Add to `crates/cli/src/main.rs`
2. Add to `Commands` enum
3. Implement handler
4. Update help text

### New Example

1. Create `examples/my_example.rs`
2. Add to `Cargo.toml` `[[example]]` section
3. Add to `examples/README.md`

## File Naming Conventions

- **Rust files**: `snake_case.rs`
- **Docs**: `UPPERCASE.md` or `Title_Case.md`
- **Scripts**: `lowercase.sh` or `lowercase.ps1`
- **Config**: `lowercase.toml` or `.lowercase`

## Code Organization

- **Public API**: Well-documented, stable
- **Internal**: May change, use at own risk
- **Tests**: In same file or `tests/` directory
- **Examples**: In `examples/` directory

---

For more details, see individual crate READMEs and documentation.
