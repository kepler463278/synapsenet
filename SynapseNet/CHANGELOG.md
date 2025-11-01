# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Cross-platform installers
- Auto-update system
- Mobile app (Tauri Mobile)

## [0.4.0] - 2024-10-31

### Added

**Desktop Application** 🖥️
- Native Tauri 2.0 application for macOS, Windows, Linux
- Modern React + TypeScript UI with 7 views
- AddGrainView - Text input with tags
- SearchView - Semantic search interface
- GraphView - D3.js knowledge visualization
- StatsView - Node statistics
- SettingsView - Configuration UI
- MonitoringView - Real-time metrics dashboard
- ErrorBoundary - Graceful error handling
- Keyboard shortcuts for navigation

**Multi-Model Embedding System** 🤖
- Load multiple ONNX models simultaneously
- Dynamic model switching without restart
- Model registry with metadata tracking
- Automatic dimension detection
- Memory-efficient model management
- Support for all-MiniLM-L6-v2 (384D)
- Support for all-mpnet-base-v2 (768D)
- Custom ONNX model support

**Batch Processing** ⚡
- Parallel embedding generation (4x faster)
- Configurable batch sizes (10-1000 items)
- Progress tracking with metrics
- Partial success handling
- Error recovery per item
- REST API endpoint: `POST /v2/batch/import`

**PoE v2 Economic Model** 💰
- Three-component scoring system
- Novelty score (40% weight) - Uniqueness
- Coherence score (30% weight) - Connections
- Reuse score (30% weight) - Access frequency
- NGT reward calculation (1-11 tokens)
- Access tracking system
- Topic diversity rewards
- Anti-gaming measures
- Log-scale reuse scoring

**Enhanced Configuration** ⚙️
- NetworkConfig - DHT, relay, clustering settings
- EconomyConfig - PoE weights and thresholds
- UiConfig - Theme, views, preferences
- TOML validation with helpful errors
- Auto-migration from v0.3
- Settings UI in desktop app
- Configuration migration guide

**Comprehensive Error Handling** 🛡️
- SynapseNetError with typed variants
- NetworkError, EmbeddingError, StorageError, BatchError
- Retry with exponential backoff
- Circuit breaker pattern
- GPU → CPU fallback
- Model size fallback (large → medium → small)
- ErrorContext for debugging
- User-friendly error messages in UI

**REST API v2** 🌐
- `GET /v2/models` - List available models
- `GET /v2/models/:name` - Get model info
- `POST /v2/batch/import` - Batch grain import
- `GET /v2/poe/scores` - PoE score queries
- `GET /v2/poe/scores/:id` - Grain PoE score
- `GET /v2/network/peers` - Peers with clusters
- `GET /v2/network/clusters` - Cluster information
- Backward compatible with v1 API
- Migration guide provided

**Monitoring & Logging** 📊
- Structured logging (JSON/Pretty formats)
- Log levels: TRACE, DEBUG, INFO, WARN, ERROR
- Performance spans and timers
- Log rotation with configurable size
- 20+ new Prometheus metrics
- PoE v2 metrics (novelty/coherence/reuse)
- Batch processing metrics
- Multi-model metrics
- Network clustering metrics
- Real-time monitoring dashboard in UI

**Storage Schema Updates** 💾
- grain_access table for access tracking
- embedding_models table for model metadata
- peer_clusters table for topic clustering
- Automatic v0.3 → v0.4 migration
- CLI migration command: `syn migrate`
- Backward-compatible schema
- Access event cleanup

**Documentation** 📚
- Complete User Guide (200+ lines)
- Quick Start Guide (5 minutes)
- API Migration Guide (v1 → v2)
- Troubleshooting section
- Configuration examples
- Best practices
- Inline code documentation

### Changed
- Configuration file format (auto-migrates)
- Database schema v2 → v4 (auto-migrates)
- Improved error messages
- Better configuration validation
- Enhanced performance
- Optimized memory usage

### Deprecated
- `POST /add` - Use `POST /v2/batch/import` instead
- `GET /peers` - Use `GET /v2/network/peers` instead

### Fixed
- Memory leaks in embedding generation
- Race conditions in P2P networking
- Configuration validation errors
- Search result ranking issues

## [0.3.0] - 2025-11-XX

### Added

**GPU Acceleration** ⚡
- CoreML execution provider for macOS (Metal backend)
- DirectML execution provider for Windows (any GPU)
- CUDA execution provider for Linux/Windows (NVIDIA GPUs)
- Auto-detection of best available GPU provider
- 2-4x speedup for embedding generation
- GPU provider configuration in config.toml
- `gpu_providers.rs` module with unified interface
- GPU demo example: `cargo run --example gpu_demo`
- Feature flags: `coreml`, `directml`, `cuda`, `gpu`
- Documentation in `docs/GPU.md`

**REST API Server** 🌐
- Local HTTP API with Axum framework
- Endpoints: `/init`, `/add`, `/query`, `/stats`, `/peers`, `/metrics`
- JSON request/response format
- CORS support for web applications
- CLI command: `syn serve --addr 127.0.0.1:9900`
- API state management with Arc<RwLock>
- Automatic embedding generation on add
- Real-time query processing
- API examples in `API_EXAMPLES.md`

**Prometheus Monitoring** 📊
- Production-grade metrics exporter
- Embedding metrics: duration histogram, total counter
- Query metrics: duration histogram, total counter
- Grain metrics: total gauge, added counter
- P2P metrics: peers gauge, messages sent/received, drops
- PoE metrics: reward total, novelty/coherence histograms
- `/metrics` endpoint for Prometheus scraping
- Ready for Grafana dashboards
- Lazy-static metric registration

**Post-Quantum Cryptography (PQC) Support** 🔐 (from v0.2)
- Unified crypto abstraction layer supporting both classical and PQC
- **Dilithium5** signatures (NIST ML-DSA standard) for quantum-resistant grain/link signing
- **Kyber1024** KEM (NIST ML-KEM standard) for quantum-resistant P2P handshakes
- Feature flags: `classical-crypto` (default), `pqc-dilithium`, `pqc-kyber`, `pqc` (all PQC)
- `UnifiedSigningKey` and `UnifiedVerifyingKey` traits for crypto backend abstraction
- `KyberHandshake` protocol for P2P key exchange
- PQC demo example: `cargo run --example pqc_demo --features pqc`
- Comprehensive PQC documentation in `docs/PQC.md`
- Quick start guide in `PQC_QUICKSTART.md`
- Tests for both classical and PQC crypto backends

**Security Benefits:**
- Protection against future quantum computer attacks
- "Harvest now, decrypt later" attack mitigation
- NIST-standardized algorithms (2024)
- Backward compatible with classical crypto

**Performance:**
- Dilithium: ~4x slower signing, ~1.5x slower verification
- Kyber: ~1.5x slower key exchange
- Storage: ~70x larger signatures (mitigated by Parquet compression)

### Planned
- REST API
- GPU acceleration (Metal/CUDA/DirectML)
- Web UI
- Mobile apps
- Hybrid signatures (classical + PQC)
- Falcon signatures (smaller than Dilithium)

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
