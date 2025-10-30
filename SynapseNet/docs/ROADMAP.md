# SynapseNet Roadmap

## v0.1.0-alpha (Current) - MVP Foundation

**Goal**: Local-first semantic memory with basic CLI

### Core ✅
- [x] Grain structure (vector + metadata + signature)
- [x] Link structure (semantic connections)
- [x] Graph (local DAG)
- [x] PoE formula (novelty + coherence + reuse)

### Storage ✅
- [x] SQLite schema (grains, links, credits, peers)
- [x] HNSW vector index
- [x] Store implementation

### CLI ✅
- [x] `syn init` - Initialize node
- [x] `syn add` - Add grains
- [x] `syn query` - Query memory
- [x] `syn peers` - Show peers (stub)
- [x] `syn export` - Export (stub)

### Governance ✅
- [x] Policy engine (OK/AnalysisOnly/Curated)
- [x] Consequence analyzer
- [x] Curator queue (stub)

### Economy ✅
- [x] NGT ledger
- [x] PoE calculation
- [x] Reputation system (basic)

### Infrastructure ✅
- [x] Cross-platform build (macOS/Linux/Windows)
- [x] Tests (unit + e2e)
- [x] CI/CD (GitHub Actions)
- [x] Documentation

### P2P 🚧
- [x] libp2p integration (basic)
- [ ] GossipSub topics
- [ ] Peer discovery (mDNS)
- [ ] Message handling

### AI 🚧
- [x] Embedding interface
- [ ] ONNX runtime integration
- [ ] all-MiniLM-L6-v2 model

---

## v0.2.0 - Network & Embeddings

**Goal**: Real P2P networking + production embeddings

### P2P
- [ ] Full GossipSub implementation
- [ ] Grain synchronization
- [ ] KNN query distribution
- [ ] Peer reputation scoring
- [ ] Bootstrap nodes

### AI
- [ ] ONNX embeddings (CPU)
- [ ] Model download/caching
- [ ] Batch processing
- [ ] Embedding cache

### DevNet
- [ ] Multi-node local testing
- [ ] Automated test scenarios
- [ ] Performance benchmarks
- [ ] Network simulation

### Storage
- [ ] Parquet export (full)
- [ ] Snapshot/restore
- [ ] Database migration
- [ ] Compression

---

## v0.3.0 - Performance & Scale

**Goal**: GPU acceleration + large-scale testing

### GPU Acceleration
- [ ] Metal backend (macOS)
- [ ] CUDA backend (Linux)
- [ ] DirectML backend (Windows)
- [ ] Benchmark suite

### Vector Index
- [ ] FAISS integration (optional)
- [ ] Index persistence
- [ ] Incremental updates
- [ ] Distributed index

### Economy
- [ ] NGT token contract (optional)
- [ ] Staking mechanism
- [ ] Curator rewards
- [ ] Governance voting

### Monitoring
- [ ] Metrics collection
- [ ] Prometheus exporter
- [ ] Grafana dashboards
- [ ] Alerting

---

## v0.4.0 - API & Integration

**Goal**: REST API + SDK for developers

### API
- [ ] REST server
- [ ] JSON-RPC
- [ ] WebSocket subscriptions
- [ ] CORS support

### SDK
- [ ] JavaScript/TypeScript
- [ ] Python
- [ ] Go
- [ ] Documentation

### Integration
- [ ] Browser extension
- [ ] VS Code extension
- [ ] Obsidian plugin
- [ ] Notion integration

---

## v0.5.0 - UI & UX

**Goal**: Desktop app + visualization

### Desktop App (Tauri)
- [ ] Graph visualization
- [ ] Grain browser
- [ ] Query interface
- [ ] Settings panel

### Features
- [ ] Real-time updates
- [ ] Search history
- [ ] Bookmarks
- [ ] Export/import

### Mobile (Future)
- [ ] iOS app
- [ ] Android app
- [ ] Sync protocol

---

## v1.0.0 - Production Ready

**Goal**: Stable, secure, scalable

### Security
- [ ] Security audit
- [ ] Penetration testing
- [ ] Bug bounty program
- [ ] Formal verification (crypto)

### Performance
- [ ] 100K+ grains per node
- [ ] 1000+ node network
- [ ] < 100ms query latency
- [ ] < 10ms add latency

### Documentation
- [ ] Complete API docs
- [ ] Video tutorials
- [ ] Use case examples
- [ ] Best practices guide

### Community
- [ ] Governance framework
- [ ] Curator onboarding
- [ ] Developer grants
- [ ] Ambassador program

---

## Future (v2.0+)

### Advanced Features
- [ ] Multi-modal embeddings (text + image + audio)
- [ ] Federated learning
- [ ] Zero-knowledge proofs
- [ ] Cross-chain bridges

### Research
- [ ] Byzantine fault tolerance
- [ ] Sybil resistance
- [ ] Incentive mechanism design
- [ ] Emergence metrics

### Ecosystem
- [ ] Marketplace for models
- [ ] Curator DAO
- [ ] Grant program
- [ ] Research partnerships

---

## Release Schedule

- **v0.1.0-alpha**: Q1 2025 ✅
- **v0.2.0**: Q2 2025
- **v0.3.0**: Q3 2025
- **v0.4.0**: Q4 2025
- **v0.5.0**: Q1 2026
- **v1.0.0**: Q2 2026

---

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for how to help with these goals.

## Feedback

Open an issue or discussion on GitHub to suggest features or changes to the roadmap.

---

*"Intelligence belongs to society. The center does not exist."*
