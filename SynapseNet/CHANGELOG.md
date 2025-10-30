# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- REST API
- GPU acceleration (Metal/CUDA/DirectML)
- Web UI
- Mobile apps

## [0.2.0] - 2025-10-29

### Added

**ONNX Embeddings Infrastructure**
- Model management system with automatic download support
- all-MiniLM-L6-v2 model integration (384-dimensional embeddings)
- Performance monitoring with timing metrics and warnings
- Configurable embedding parameters via config.toml
- Hash-based fallback embeddings for development

**P2P Networking (libp2p)**
- Full P2P swarm implementation with libp2p 0.53
- mDNS peer discovery for local network
- Noise protocol encryption for secure communication
- Identify protocol for peer information exchange
- GossipSub pub/sub with 4 topics:
  - `grains.put` - Grain broadcasting
  - `grains.ack` - Acknowledgments
  - `query.knn` - Distributed queries
  - `query.resp` - Query responses
- Grain broadcasting with signature verification
- Distributed KNN queries across peers
- Peer reputation system (-10 threshold for disconnection)
- Rate limiting (100 grains/min per peer)
- Automatic bad peer disconnection
- Connection/disconnection event handling
- Peer statistics tracking (grains sent/received, reputation)
- **Grain storage integration** - Callback system for automatic storage (NEW!)
- Received grains automatically stored in database (NEW!)

**Parquet Export/Import**
- Export grains to Apache Parquet format
- Snappy compression for efficient storage
- Batch processing (10,000 grains per file)
- Import with signature verification
- Columnar storage for large datasets
- CLI commands: `syn export` and `syn import`
- Export/import statistics display

**Configuration Management**
- TOML-based configuration system
- `Config` struct with validation
- Configurable sections: node, p2p, ai, storage
- `syn config` command to generate default config
- config.toml.example template
- Load/save/validate configuration

**CLI Enhancements**
- New `export` command with progress indicators
- New `import` command with statistics
- New `config` command for configuration generation
- New `stats` command for node metrics and statistics
- Enhanced `peers` command with helpful information
- Progress bars with indicatif library
- Better error messages and user feedback

**Examples**
- `p2p_demo.rs` - Basic P2P swarm initialization
- `p2p_broadcast.rs` - Grain broadcasting demo
- `p2p_query.rs` - Distributed query demo
- `p2p_with_storage.rs` - P2P with automatic grain storage (NEW!)

### Changed
- Updated libp2p to 0.53 with tokio support
- Updated arrow/parquet to 53.0
- Improved CLI output with emojis and formatting
- Enhanced logging with structured tracing

### Fixed
- Arrow/Parquet compatibility with chrono
- ListArray nullable field handling
- Binary array serialization
- Database path consistency

### Security
- Grain signature verification on import
- Peer reputation tracking
- Rate limiting to prevent spam
- Automatic disconnection of malicious peers

## [0.1.0-alpha] - 2025-01-XX

### Added
- Core primitives: Grain, Link, Graph
- SQLite storage backend
- HNSW vector index for KNN search
- CLI commands: init, add, query, peers, export
- Proof of Emergence (PoE) calculation
- Policy engine (OK/AnalysisOnly/Curated)
- Consequence analyzer for risk queries
- NGT ledger and reputation system
- Cross-platform support (macOS/Linux/Windows)
- E2E tests
- DevNet scripts for local testing
- Comprehensive documentation

### Security
- ed25519 signatures for grains and links
- blake3 hashing for grain IDs
- Signature verification
- Anti-spam thresholds

### Documentation
- README with quick start
- GENESIS.txt with founding principles
- SECURITY.md with security policy
- CONTRIBUTING.md with development guidelines
- API documentation
- Architecture overview
- Roadmap

## [0.0.1] - 2025-01-XX (Initial Commit)

### Added
- Project structure
- Cargo workspace setup
- Basic crate scaffolding

---

## Release Notes

### v0.1.0-alpha

This is the first public alpha release of SynapseNet. It provides:

**Core Functionality:**
- Local semantic memory with vector search
- Cryptographically signed knowledge grains
- Proof of Emergence reward calculation
- Safe response policy engine

**Limitations:**
- P2P networking not fully implemented (local-only mode)
- Dummy embeddings (ONNX integration pending)
- No GPU acceleration yet
- No UI (CLI only)

**Known Issues:**
- Query results may not be optimal with dummy embeddings
- P2P peers command shows "Local mode"
- Export to Parquet not implemented

**Next Steps:**
See [ROADMAP.md](docs/ROADMAP.md) for planned features in v0.2.0.

---

## Upgrade Guide

### From 0.0.x to 0.1.0

This is the first release, no upgrade needed.

---

## Contributors

Thank you to all contributors who made this release possible!

- Initial implementation: EK
- Community feedback: [Your name here]

---

*"Intelligence belongs to society. The center does not exist."*
