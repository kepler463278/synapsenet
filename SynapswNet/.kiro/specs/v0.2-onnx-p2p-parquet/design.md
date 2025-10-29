# Design Document - SynapseNet v0.2

## Overview

SynapseNet v0.2 transforms the system from a local-only prototype to a functional distributed semantic memory network. This document describes the architecture, components, and implementation strategy for three major features: ONNX embeddings, P2P networking, and Parquet export.

## Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         CLI Layer                            │
│              syn init | add | query | peers                  │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                      Application Layer                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │   Embedding  │  │   P2P Swarm  │  │   Parquet    │     │
│  │   Service    │  │   Manager    │  │   Exporter   │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                        Core Layer                            │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │  Grain   │  │   Link   │  │  Graph   │  │   PoE    │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                      Storage Layer                           │
│  ┌──────────────┐      ┌──────────────┐                    │
│  │   SQLite     │      │  HNSW Index  │                    │
│  └──────────────┘      └──────────────┘                    │
└─────────────────────────────────────────────────────────────┘
```

### Component Interaction Flow

```
User Command (syn add "text")
        ↓
    CLI Parser
        ↓
Embedding Service → ONNX Model → 384-dim vector
        ↓
   Grain Creation → Sign with ed25519
        ↓
   Store in SQLite + HNSW Index
        ↓
P2P Broadcast → GossipSub (grains.put)
        ↓
   Peers receive → Verify → Store
```

## Components and Interfaces

### 1. ONNX Embedding Service

**Location:** `crates/ai/src/onnx_embed.rs`

**Purpose:** Generate semantic embeddings using ONNX Runtime

**Interface:**
```rust
pub struct OnnxEmbedding {
    session: Session,
    tokenizer: Tokenizer,
    model_path: PathBuf,
}

impl OnnxEmbedding {
    /// Load ONNX model from path or download if missing
    pub fn new(model_path: PathBuf) -> Result<Self>;
    
    /// Generate embedding for single text
    pub fn embed(&self, text: &str) -> Result<Vec<f32>>;
    
    /// Generate embeddings for batch of texts
    pub fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>>;
    
    /// Get embedding dimension (384 for all-MiniLM-L6-v2)
    pub fn dim(&self) -> usize;
}
```

**Dependencies:**
- `ort` crate for ONNX Runtime
- `tokenizers` crate for text tokenization
- `reqwest` for model download

**Model Management:**
```rust
pub struct ModelManager {
    models_dir: PathBuf,
}

impl ModelManager {
    /// Download model from HuggingFace if not present
    pub async fn ensure_model(&self, model_name: &str) -> Result<PathBuf>;
    
    /// Verify model file integrity
    pub fn verify_checksum(&self, path: &PathBuf, expected: &str) -> Result<bool>;
    
    /// Get model path
    pub fn model_path(&self, model_name: &str) -> PathBuf;
}
```

**Model Details:**
- Model: `sentence-transformers/all-MiniLM-L6-v2`
- Format: ONNX (optimized for CPU)
- Dimension: 384
- Max tokens: 512
- Download URL: HuggingFace model hub

### 2. P2P Network Layer

**Location:** `crates/p2p/src/`

**Purpose:** Enable distributed grain exchange using libp2p

**Swarm Manager:**
```rust
pub struct SynapseSwarm {
    swarm: Swarm<SynapseBehaviour>,
    local_peer_id: PeerId,
    connected_peers: HashMap<PeerId, PeerInfo>,
}

impl SynapseSwarm {
    /// Create new swarm with mDNS discovery
    pub async fn new(config: P2pConfig) -> Result<Self>;
    
    /// Start swarm event loop
    pub async fn run(&mut self) -> Result<()>;
    
    /// Broadcast grain to all peers
    pub fn broadcast_grain(&mut self, grain: &Grain) -> Result<()>;
    
    /// Query peers for KNN results
    pub async fn query_peers(&mut self, query_vec: &[f32], k: usize) -> Result<Vec<QueryResult>>;
    
    /// Get connected peer count
    pub fn peer_count(&self) -> usize;
}
```

**Network Behaviour:**
```rust
#[derive(NetworkBehaviour)]
pub struct SynapseBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
    identify: identify::Behaviour,
}
```

**Message Types:**
```rust
#[derive(Serialize, Deserialize)]
pub enum NetworkMessage {
    GrainPut {
        grain: Grain,
        links: Vec<Link>,
    },
    GrainAck {
        grain_id: [u8; 32],
        peer_id: String,
    },
    QueryKnn {
        query_id: String,
        vector: Vec<f32>,
        k: usize,
    },
    QueryResponse {
        query_id: String,
        results: Vec<QueryResult>,
    },
}
```

**Topics:**
- `grains.put` - Broadcast new grains
- `grains.ack` - Acknowledge receipt
- `query.knn` - Distributed KNN queries
- `query.resp` - Query responses

**Security:**
- Noise protocol for encryption
- Signature verification on all grains
- Peer reputation tracking
- Rate limiting (100 grains/min per peer)

### 3. Parquet Export/Import

**Location:** `crates/storage/src/parquet_io.rs`

**Purpose:** Backup and restore knowledge base

**Exporter:**
```rust
pub struct ParquetExporter {
    output_dir: PathBuf,
    batch_size: usize, // 10,000 grains per file
}

impl ParquetExporter {
    pub fn new(output_dir: PathBuf) -> Self;
    
    /// Export all grains to Parquet files
    pub fn export(&self, store: &Store) -> Result<ExportStats>;
    
    /// Export specific grains
    pub fn export_grains(&self, grains: &[Grain]) -> Result<PathBuf>;
}

pub struct ExportStats {
    pub total_grains: usize,
    pub files_created: usize,
    pub total_bytes: u64,
    pub duration: Duration,
}
```

**Importer:**
```rust
pub struct ParquetImporter {
    input_dir: PathBuf,
}

impl ParquetImporter {
    pub fn new(input_dir: PathBuf) -> Self;
    
    /// Import grains from Parquet files
    pub fn import(&self, store: &mut Store) -> Result<ImportStats>;
    
    /// Verify grain signatures during import
    pub fn import_with_verification(&self, store: &mut Store) -> Result<ImportStats>;
}

pub struct ImportStats {
    pub total_grains: usize,
    pub imported: usize,
    pub skipped: usize,
    pub invalid_signatures: usize,
    pub duration: Duration,
}
```

**Schema:**
```rust
// Parquet schema for grains
struct GrainRecord {
    id: [u8; 32],
    vec: Vec<f32>,
    author_pk: [u8; 32],
    ts_unix_ms: i64,
    tags: Vec<String>,
    mime: String,
    lang: String,
    title: Option<String>,
    summary: Option<String>,
    sig: Vec<u8>,
}
```

**Compression:** Snappy (fast, good compression ratio)

### 4. Configuration Management

**Location:** `crates/core/src/config.rs`

**Config File:** `{data_dir}/config.toml`

**Structure:**
```toml
[node]
data_dir = ".synapsenet"

[embedding]
model = "all-MiniLM-L6-v2"
model_path = "models/"
batch_size = 32
device = "cpu"

[p2p]
enabled = true
port = 9000
bootstrap_peers = [
    "/ip4/1.2.3.4/tcp/9000/p2p/12D3KooW..."
]
mdns_enabled = true

[storage]
max_grains = 1000000
index_ef_construction = 200
index_ef_search = 200

[export]
batch_size = 10000
compression = "snappy"
```

**Config Struct:**
```rust
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub node: NodeConfig,
    pub embedding: EmbeddingConfig,
    pub p2p: P2pConfig,
    pub storage: StorageConfig,
    pub export: ExportConfig,
}

impl Config {
    /// Load from file or use defaults
    pub fn load(path: &Path) -> Result<Self>;
    
    /// Save to file
    pub fn save(&self, path: &Path) -> Result<()>;
    
    /// Validate configuration
    pub fn validate(&self) -> Result<()>;
}
```

## Data Models

### Enhanced Grain (no changes to structure)

Grain structure remains the same, but:
- `vec` now contains real 384-dim embeddings (not dummy)
- Embeddings generated by ONNX model
- All other fields unchanged

### P2P Message Envelope

```rust
pub struct MessageEnvelope {
    pub message_id: String,
    pub sender: PeerId,
    pub timestamp: i64,
    pub payload: NetworkMessage,
    pub signature: Vec<u8>,
}
```

### Peer Information

```rust
pub struct PeerInfo {
    pub peer_id: PeerId,
    pub addresses: Vec<Multiaddr>,
    pub connected_at: i64,
    pub grains_received: u64,
    pub grains_sent: u64,
    pub reputation: f64,
    pub last_seen: i64,
}
```

## Error Handling

### Error Types

```rust
#[derive(Error, Debug)]
pub enum SynapseError {
    #[error("ONNX model not found: {0}")]
    ModelNotFound(String),
    
    #[error("Embedding generation failed: {0}")]
    EmbeddingError(String),
    
    #[error("P2P network error: {0}")]
    NetworkError(String),
    
    #[error("Invalid grain signature")]
    InvalidSignature,
    
    #[error("Parquet export failed: {0}")]
    ExportError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}
```

### Error Recovery

- **Model download failure**: Retry 3 times, then fallback to dummy embeddings with warning
- **P2P connection failure**: Continue in local-only mode
- **Signature verification failure**: Reject grain, log warning, decrease peer reputation
- **Parquet export failure**: Partial export, save what succeeded

## Testing Strategy

### Unit Tests

1. **ONNX Embedding**
   - Test model loading
   - Test embedding generation
   - Test batch processing
   - Test error handling (missing model)

2. **P2P Network**
   - Test swarm initialization
   - Test message serialization
   - Test signature verification
   - Test peer discovery (mocked)

3. **Parquet Export**
   - Test export/import round-trip
   - Test compression
   - Test large datasets (100k grains)
   - Test corrupted file handling

### Integration Tests

1. **End-to-End Embedding**
   - Add grain with ONNX → Query → Verify relevance

2. **Multi-Node P2P**
   - Start 3 nodes → Add grain on node1 → Verify on node2/node3

3. **Export/Import**
   - Export 1000 grains → Import to new node → Verify integrity

### Performance Tests

1. **Embedding Speed**
   - Measure time for 100 embeddings
   - Target: < 500ms per embedding

2. **P2P Throughput**
   - Measure grains/second across network
   - Target: 100 grains/minute

3. **Parquet Export**
   - Measure export speed for 100k grains
   - Target: > 1000 grains/second

## Deployment Considerations

### Model Distribution

- Model files (~90MB) not included in binary
- Downloaded on first run
- Cached in `{data_dir}/models/`
- Option to pre-download for offline use

### P2P Bootstrap

- mDNS for local network discovery
- Optional bootstrap peers for public network
- Fallback to local-only mode if P2P fails

### Resource Usage

- ONNX model: ~200MB RAM
- P2P connections: ~1MB per peer
- HNSW index: ~4 bytes per dimension per grain
- Total for 100k grains: ~500MB RAM

## Migration from v0.1

### Database Migration

No schema changes required. Existing grains will:
1. Keep dummy embeddings initially
2. Be re-embedded on first query (lazy migration)
3. Or re-embedded in batch via `syn reindex` command

### Configuration

New `config.toml` file created on first run with defaults.

### Backward Compatibility

v0.2 nodes can read v0.1 databases but not vice versa (due to P2P metadata).

## Security Considerations

### Threat Model

1. **Malicious Peer**: Sends invalid grains
   - Mitigation: Signature verification, reputation tracking

2. **Sybil Attack**: Many fake peers
   - Mitigation: Rate limiting, reputation system

3. **Model Poisoning**: Corrupted ONNX model
   - Mitigation: Checksum verification

4. **Data Exfiltration**: Peer collects all grains
   - Mitigation: None (public network by design)

### Security Measures

- All grains signed with ed25519
- P2P connections encrypted with Noise
- Model checksum verification
- Peer reputation tracking
- Rate limiting on grain ingestion

## Performance Optimizations

### Embedding

- Batch processing for multiple texts
- Model caching in memory
- Tokenizer reuse

### P2P

- Message batching
- Connection pooling
- Lazy peer discovery

### Storage

- HNSW index tuning (ef_construction, ef_search)
- SQLite WAL mode
- Prepared statements

## Monitoring and Observability

### Metrics

```rust
pub struct NodeMetrics {
    pub grains_total: u64,
    pub grains_local: u64,
    pub grains_remote: u64,
    pub peers_connected: usize,
    pub embeddings_generated: u64,
    pub embedding_avg_time_ms: f64,
    pub queries_total: u64,
    pub queries_avg_time_ms: f64,
}
```

### Logging

- Structured logging with `tracing`
- Log levels: ERROR, WARN, INFO, DEBUG, TRACE
- P2P events logged at INFO
- Performance metrics at DEBUG

## Documentation Updates

### User Documentation

- Update README with ONNX setup
- Add P2P configuration guide
- Add export/import tutorial
- Update FAQ with v0.2 questions

### Developer Documentation

- API documentation for new modules
- Architecture diagrams
- Integration guide for embeddings
- P2P protocol specification

## Rollout Plan

### Phase 1: ONNX Embeddings (Week 1-2)
1. Implement OnnxEmbedding struct
2. Add model download logic
3. Integrate with CLI
4. Test and benchmark

### Phase 2: P2P Network (Week 3-4)
1. Implement SynapseSwarm
2. Add GossipSub handlers
3. Implement grain broadcast
4. Test multi-node scenarios

### Phase 3: Parquet Export (Week 5)
1. Implement ParquetExporter
2. Implement ParquetImporter
3. Add CLI commands
4. Test large datasets

### Phase 4: Integration & Testing (Week 6)
1. End-to-end testing
2. Performance optimization
3. Documentation
4. Release v0.2.0

## Success Metrics

v0.2 is successful when:
- [ ] ONNX embeddings generate in < 500ms
- [ ] 3 nodes can discover each other via mDNS
- [ ] Grains propagate across network in < 5 seconds
- [ ] Query results are semantically relevant
- [ ] Export/import 100k grains in < 2 minutes
- [ ] All tests pass
- [ ] Documentation complete
- [ ] Zero critical bugs

## Open Questions

1. Should we support multiple ONNX models? (Decision: No, v0.4)
2. How to handle NAT traversal? (Decision: v0.3 with relay)
3. Should export be incremental? (Decision: Full export for v0.2)
4. GPU support priority? (Decision: v0.3)

## References

- ONNX Runtime: https://onnxruntime.ai/
- all-MiniLM-L6-v2: https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2
- libp2p: https://libp2p.io/
- Apache Parquet: https://parquet.apache.org/
- GossipSub spec: https://github.com/libp2p/specs/blob/master/pubsub/gossipsub/
