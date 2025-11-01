# Design Document - SynapseNet v0.4

## Overview

SynapseNet v0.4 transforms the project from a developer-focused CLI tool into a mass-market application through five major architectural additions:

1. **Tauri-based Web UI** - Native desktop application with modern web frontend
2. **Multi-Model AI System** - Support for multiple embedding models with automatic selection
3. **Global P2P Mesh** - DHT-based discovery and NAT traversal for worldwide connectivity
4. **Batch Processing Pipeline** - High-throughput knowledge import and embedding
5. **Enhanced PoE v2** - Three-component reward system (novelty, coherence, reuse)

This design builds on the existing v0.3 architecture (PQC crypto, GPU acceleration, REST API, Parquet storage) and maintains backward compatibility.

## Architecture

### High-Level System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Tauri Web UI                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │   Add View   │  │ Search View  │  │  Graph View  │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
│         TypeScript/React Frontend (Renderer)               │
└─────────────────────────────────────────────────────────────┘
                            │ IPC
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   Tauri Backend (Rust)                      │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              Command Handlers                         │  │
│  │  • add_grain  • search  • get_stats  • batch_import  │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                  Core SynapseNet Engine                     │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │   AI     │  │ Storage  │  │   P2P    │  │ Economy  │  │
│  │ (Multi)  │  │ (Parquet)│  │  (DHT)   │  │ (PoE v2) │  │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Technology Stack

**Frontend:**
- Tauri 2.0 (native app framework)
- TypeScript + React (UI framework)
- TailwindCSS (styling)
- D3.js or Cytoscape.js (graph visualization)

**Backend:**
- Rust (existing crates + new Tauri integration)
- Existing: core, p2p, storage, ai, economy
- New: tauri-app crate

**Infrastructure:**
- libp2p with Kademlia DHT (peer discovery)
- ONNX Runtime (multi-model support)
- SQLite + Parquet (storage)
- PQC crypto (existing)



## Components and Interfaces

### 1. Tauri Application Layer

#### 1.1 Frontend Components

**AddGrainView**
- Text input area (expandable)
- Tag input (comma-separated)
- Submit button
- Real-time character count
- Success/error notifications

**SearchView**
- Semantic search input
- Results list with similarity scores
- Result preview cards
- Filter by tags/date
- Sort by relevance/date

**GraphView**
- Interactive knowledge graph
- Node = grain, Edge = semantic similarity
- Zoom/pan controls
- Node click → detail view
- Cluster visualization by topic

**StatsView**
- Total grains count
- Network peers count
- Storage usage
- PoE rewards earned
- Recent activity timeline

#### 1.2 Tauri Commands (IPC Bridge)

```rust
// crates/tauri-app/src/commands.rs

#[tauri::command]
async fn add_grain(
    text: String,
    tags: Vec<String>,
    state: State<'_, AppState>
) -> Result<AddGrainResponse, String>

#[tauri::command]
async fn search_grains(
    query: String,
    k: usize,
    state: State<'_, AppState>
) -> Result<Vec<SearchResult>, String>

#[tauri::command]
async fn get_grain_details(
    grain_id: String,
    state: State<'_, AppState>
) -> Result<GrainDetails, String>

#[tauri::command]
async fn get_stats(
    state: State<'_, AppState>
) -> Result<NodeStats, String>

#[tauri::command]
async fn batch_import(
    paths: Vec<String>,
    state: State<'_, AppState>
) -> Result<BatchImportProgress, String>

#[tauri::command]
async fn get_network_peers(
    state: State<'_, AppState>
) -> Result<Vec<PeerInfo>, String>
```

#### 1.3 Application State

```rust
pub struct AppState {
    pub store: Arc<RwLock<Store>>,
    pub embedding_manager: Arc<MultiModelManager>,
    pub p2p_swarm: Arc<RwLock<Swarm<SynapseNetBehaviour>>>,
    pub poe_engine: Arc<RwLock<PoEEngine>>,
    pub signing_key: Arc<UnifiedSigningKey>,
    pub config: Arc<RwLock<Config>>,
}
```



### 2. Multi-Model AI System

#### 2.1 Model Categories

```rust
pub enum ModelSize {
    Small,   // MiniLM (33MB, 384-dim) - phones/low-end
    Medium,  // BERT/E5 (120MB, 768-dim) - laptops
    Large,   // Nomic/Mistral (500MB+, 1024-dim) - servers/GPU
}

pub struct ModelInfo {
    pub name: String,
    pub size: ModelSize,
    pub dimensions: usize,
    pub file_size_mb: usize,
    pub min_ram_mb: usize,
    pub supports_gpu: bool,
}
```

#### 2.2 Multi-Model Manager

```rust
// crates/ai/src/multi_model.rs

pub struct MultiModelManager {
    models: HashMap<String, Arc<OnnxEmbedding>>,
    active_model: RwLock<String>,
    gpu_provider: GpuProvider,
}

impl MultiModelManager {
    pub async fn load_model(&mut self, model_info: &ModelInfo) -> Result<()>
    
    pub async fn embed_with_model(
        &self,
        text: &str,
        model_name: &str
    ) -> Result<Vec<f32>>
    
    pub async fn embed_auto(&self, text: &str) -> Result<Vec<f32>>
    
    pub fn list_loaded_models(&self) -> Vec<ModelInfo>
    
    pub fn get_best_model_for_hardware(&self) -> String
}
```

#### 2.3 Vector Space Alignment

Different models produce different dimensional embeddings. We handle this through:

**Option A: Dimension Padding/Truncation**
- Pad smaller vectors with zeros
- Truncate larger vectors
- Simple but loses information

**Option B: Learned Projection (v0.4.1+)**
- Train small neural network to project between spaces
- Maintains semantic relationships
- Requires training data

**v0.4 Implementation: Option A with metadata tagging**
- Store model name + dimensions in GrainMeta
- Search only within same model space initially
- Cross-model search in future release

```rust
pub struct GrainMeta {
    // ... existing fields ...
    pub embedding_model: String,      // NEW
    pub embedding_dimensions: usize,  // NEW
}
```



### 3. Global P2P Mesh Networking

#### 3.1 Enhanced P2P Architecture

```rust
// crates/p2p/src/behaviour.rs

use libp2p::{
    gossipsub, identify, kad, mdns, noise, tcp, yamux,
    swarm::NetworkBehaviour,
};

#[derive(NetworkBehaviour)]
pub struct SynapseNetBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,  // NEW
    pub identify: identify::Behaviour,
}
```

#### 3.2 DHT-Based Discovery

**Kademlia DHT Integration:**
- Each node has a PeerID derived from public key
- DHT stores peer routing information
- Nodes announce themselves to DHT
- Queries find closest peers by topic/interest

```rust
// crates/p2p/src/dht.rs

pub struct DhtDiscovery {
    kademlia: kad::Behaviour<kad::store::MemoryStore>,
}

impl DhtDiscovery {
    pub async fn bootstrap(&mut self, bootstrap_peers: Vec<Multiaddr>) -> Result<()>
    
    pub async fn announce_topic(&mut self, topic: &str) -> Result<()>
    
    pub async fn find_peers_for_topic(&mut self, topic: &str) -> Result<Vec<PeerId>>
    
    pub async fn get_closest_peers(&mut self, key: &[u8]) -> Result<Vec<PeerId>>
}
```

**Bootstrap Nodes:**
- Hardcoded list of stable bootstrap peers
- Community-run infrastructure
- Fallback to DNS seeds

#### 3.3 NAT Traversal

**Hole-Punching Strategy:**
1. Use libp2p's AutoNAT to detect NAT type
2. Attempt direct connection first
3. If behind NAT, use relay protocol (Circuit Relay v2)
4. STUN-like coordination through relay

```rust
// crates/p2p/src/nat.rs

pub struct NatTraversal {
    autonat: autonat::Behaviour,
    relay_client: relay::client::Behaviour,
}

impl NatTraversal {
    pub async fn detect_nat_type(&mut self) -> NatType
    
    pub async fn establish_connection(
        &mut self,
        peer_id: PeerId
    ) -> Result<Connection>
}
```

#### 3.4 Peer Clustering

Nodes automatically cluster by shared interests:

```rust
pub struct PeerCluster {
    pub topic: String,
    pub peers: HashSet<PeerId>,
    pub last_updated: SystemTime,
}

impl PeerCluster {
    pub fn add_peer(&mut self, peer_id: PeerId)
    
    pub fn remove_inactive_peers(&mut self, timeout: Duration)
    
    pub fn get_best_peers(&self, count: usize) -> Vec<PeerId>
}
```

**Clustering Algorithm:**
- Analyze grain tags/topics from each peer
- Calculate topic similarity using cosine distance
- Group peers with >0.7 similarity
- Prioritize connections within cluster



### 4. Batch Processing Pipeline

#### 4.1 Batch Import Architecture

```rust
// crates/ai/src/batch.rs

pub struct BatchProcessor {
    embedding_manager: Arc<MultiModelManager>,
    store: Arc<RwLock<Store>>,
    index: Arc<RwLock<HnswIndex>>,
    signing_key: Arc<UnifiedSigningKey>,
    gpu_provider: GpuProvider,
}

pub struct BatchConfig {
    pub batch_size: usize,           // Process N items at once
    pub parallel_workers: usize,     // Number of parallel threads
    pub use_gpu: bool,
    pub model_name: String,
}

impl BatchProcessor {
    pub async fn import_directory(
        &self,
        path: &Path,
        config: BatchConfig,
        progress_tx: mpsc::Sender<BatchProgress>
    ) -> Result<BatchResult>
    
    pub async fn import_files(
        &self,
        files: Vec<PathBuf>,
        config: BatchConfig,
        progress_tx: mpsc::Sender<BatchProgress>
    ) -> Result<BatchResult>
}
```

#### 4.2 File Format Support

```rust
pub enum SupportedFormat {
    PlainText,
    Markdown,
    Pdf,
    Json,
    Csv,
}

pub trait DocumentParser: Send + Sync {
    fn parse(&self, content: &[u8]) -> Result<Vec<String>>;
    fn supported_extensions(&self) -> Vec<&'static str>;
}

// Parsers
pub struct PlainTextParser;
pub struct MarkdownParser;
pub struct PdfParser;  // Using pdf-extract crate
pub struct JsonParser;
```

#### 4.3 Batch Processing Pipeline

```
Input Files
    │
    ▼
┌─────────────────┐
│  File Scanner   │  Recursively find files
└─────────────────┘
    │
    ▼
┌─────────────────┐
│  Format Detect  │  Detect file type
└─────────────────┘
    │
    ▼
┌─────────────────┐
│  Parse Content  │  Extract text chunks
└─────────────────┘
    │
    ▼
┌─────────────────┐
│  Batch Embed    │  GPU-accelerated embedding
│  (GPU/CPU)      │  Process N chunks at once
└─────────────────┘
    │
    ▼
┌─────────────────┐
│  Create Grains  │  Sign + create grain objects
└─────────────────┘
    │
    ▼
┌─────────────────┐
│  Store + Index  │  Persist to storage + HNSW
└─────────────────┘
    │
    ▼
┌─────────────────┐
│  P2P Broadcast  │  Optional: share to network
└─────────────────┘
```

#### 4.4 Progress Tracking

```rust
pub struct BatchProgress {
    pub total_files: usize,
    pub processed_files: usize,
    pub total_chunks: usize,
    pub processed_chunks: usize,
    pub current_file: String,
    pub elapsed_seconds: u64,
    pub estimated_remaining_seconds: u64,
}

pub struct BatchResult {
    pub success_count: usize,
    pub error_count: usize,
    pub total_grains_created: usize,
    pub total_time_seconds: u64,
    pub errors: Vec<BatchError>,
}
```



### 5. Enhanced PoE v2 Economic Model

#### 5.1 Three-Component Reward System

```rust
// crates/economy/src/poe_v2.rs

pub struct PoEv2Engine {
    store: Arc<RwLock<Store>>,
    index: Arc<RwLock<HnswIndex>>,
    reuse_tracker: Arc<RwLock<ReuseTracker>>,
}

pub struct PoEScore {
    pub novelty: f32,      // 0.0 - 1.0
    pub coherence: f32,    // 0.0 - 1.0
    pub reuse: f32,        // 0.0 - 1.0
    pub total: f32,        // Weighted sum
}

impl PoEv2Engine {
    pub fn calculate_score(&self, grain: &Grain) -> Result<PoEScore>
}
```

#### 5.2 Novelty Score Calculation

**Algorithm:**
1. Embed new grain
2. Search for K nearest neighbors (K=10)
3. Calculate average similarity to neighbors
4. Novelty = 1.0 - avg_similarity

```rust
pub fn calculate_novelty(
    grain_embedding: &[f32],
    index: &HnswIndex
) -> Result<f32> {
    let neighbors = index.search(grain_embedding, 10)?;
    
    if neighbors.is_empty() {
        return Ok(1.0); // First grain = maximum novelty
    }
    
    let avg_similarity = neighbors.iter()
        .map(|n| n.similarity)
        .sum::<f32>() / neighbors.len() as f32;
    
    Ok(1.0 - avg_similarity)
}
```

**Novelty Thresholds:**
- 0.9 - 1.0: Highly novel (rare)
- 0.7 - 0.9: Novel
- 0.5 - 0.7: Somewhat novel
- 0.3 - 0.5: Common
- 0.0 - 0.3: Duplicate/spam

#### 5.3 Coherence Score Calculation

**Algorithm:**
1. Find related grains (similarity > 0.6)
2. Analyze connection patterns
3. Reward grains that bridge topics

```rust
pub fn calculate_coherence(
    grain: &Grain,
    store: &Store,
    index: &HnswIndex
) -> Result<f32> {
    let related = index.search(&grain.vec, 20)?
        .into_iter()
        .filter(|r| r.similarity > 0.6)
        .collect::<Vec<_>>();
    
    if related.len() < 2 {
        return Ok(0.0); // No connections
    }
    
    // Calculate topic diversity
    let topics = extract_topics(&related, store)?;
    let diversity = calculate_topic_diversity(&topics);
    
    // Coherence = connections × diversity
    let connection_score = (related.len() as f32 / 20.0).min(1.0);
    Ok(connection_score * diversity)
}
```

**Coherence Factors:**
- Number of connections (more = better)
- Topic diversity (bridges = better)
- Connection strength (stronger = better)

#### 5.4 Reuse Score Tracking

```rust
pub struct ReuseTracker {
    access_log: HashMap<GrainId, Vec<AccessEvent>>,
}

pub struct AccessEvent {
    pub timestamp: SystemTime,
    pub peer_id: PeerId,
    pub access_type: AccessType,
}

pub enum AccessType {
    Search,      // Found in search results
    Retrieve,    // Explicitly retrieved
    Reference,   // Referenced by another grain
}

impl ReuseTracker {
    pub fn record_access(&mut self, grain_id: GrainId, event: AccessEvent)
    
    pub fn calculate_reuse_score(&self, grain_id: &GrainId) -> f32 {
        let events = self.access_log.get(grain_id);
        
        if events.is_none() {
            return 0.0;
        }
        
        let events = events.unwrap();
        let unique_peers = events.iter()
            .map(|e| e.peer_id)
            .collect::<HashSet<_>>()
            .len();
        
        // Score based on unique users and access frequency
        let frequency_score = (events.len() as f32).log10() / 3.0; // Log scale
        let diversity_score = (unique_peers as f32).log10() / 2.0;
        
        (frequency_score + diversity_score).min(1.0)
    }
}
```

#### 5.5 Final Reward Calculation

```rust
pub struct RewardWeights {
    pub novelty: f32,     // Default: 0.4
    pub coherence: f32,   // Default: 0.3
    pub reuse: f32,       // Default: 0.3
}

pub fn calculate_ngt_reward(
    score: &PoEScore,
    weights: &RewardWeights
) -> f32 {
    let weighted_score = 
        score.novelty * weights.novelty +
        score.coherence * weights.coherence +
        score.reuse * weights.reuse;
    
    // Base reward: 1 NGT
    // Bonus: up to 10 NGT for exceptional contributions
    1.0 + (weighted_score * 10.0)
}
```

**Anti-Gaming Measures:**
- Penalize duplicate content (novelty < 0.3)
- Rate limit submissions per peer
- Require minimum coherence for rewards
- Decay reuse score over time



## Data Models

### Enhanced Grain Structure

```rust
// crates/core/src/grain.rs

pub struct Grain {
    pub id: GrainId,              // blake3 hash
    pub vec: Vec<f32>,            // Embedding vector
    pub meta: GrainMeta,
    pub sig: Vec<u8>,             // Signature
}

pub struct GrainMeta {
    pub author_pk: Vec<u8>,
    pub crypto_backend: CryptoBackend,
    pub ts_unix_ms: i64,
    pub tags: Vec<String>,
    pub mime: String,
    pub lang: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    
    // NEW in v0.4
    pub embedding_model: String,      // e.g. "all-MiniLM-L6-v2"
    pub embedding_dimensions: usize,  // e.g. 384
    pub poe_score: Option<PoEScore>,  // Cached PoE score
    pub access_count: u64,            // Local access counter
}
```

### Configuration Model

```rust
// crates/core/src/config.rs

pub struct Config {
    pub data_dir: PathBuf,
    pub network: NetworkConfig,
    pub ai: AiConfig,
    pub economy: EconomyConfig,
    pub ui: UiConfig,
}

pub struct NetworkConfig {
    pub listen_addresses: Vec<Multiaddr>,
    pub bootstrap_peers: Vec<Multiaddr>,
    pub enable_mdns: bool,
    pub enable_dht: bool,
    pub enable_relay: bool,
    pub max_peers: usize,
}

pub struct AiConfig {
    pub models: Vec<ModelConfig>,
    pub default_model: String,
    pub gpu_provider: Option<String>,
    pub batch_size: usize,
}

pub struct ModelConfig {
    pub name: String,
    pub path: PathBuf,
    pub size: ModelSize,
    pub auto_load: bool,
}

pub struct EconomyConfig {
    pub enable_poe: bool,
    pub reward_weights: RewardWeights,
    pub min_novelty_threshold: f32,
}

pub struct UiConfig {
    pub theme: String,
    pub default_view: String,
    pub enable_graph: bool,
}
```

### Storage Schema Updates

**SQLite Tables:**

```sql
-- Existing grains table
CREATE TABLE grains (
    id BLOB PRIMARY KEY,
    vec BLOB NOT NULL,
    meta_json TEXT NOT NULL,
    sig BLOB NOT NULL,
    created_at INTEGER NOT NULL
);

-- NEW: Access tracking
CREATE TABLE grain_access (
    grain_id BLOB NOT NULL,
    peer_id TEXT NOT NULL,
    access_type TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    FOREIGN KEY (grain_id) REFERENCES grains(id)
);

CREATE INDEX idx_grain_access_grain ON grain_access(grain_id);
CREATE INDEX idx_grain_access_time ON grain_access(timestamp);

-- NEW: Model metadata
CREATE TABLE embedding_models (
    name TEXT PRIMARY KEY,
    dimensions INTEGER NOT NULL,
    file_size_mb INTEGER NOT NULL,
    loaded_at INTEGER
);

-- NEW: Peer clusters
CREATE TABLE peer_clusters (
    topic TEXT NOT NULL,
    peer_id TEXT NOT NULL,
    similarity REAL NOT NULL,
    last_seen INTEGER NOT NULL,
    PRIMARY KEY (topic, peer_id)
);
```



## Error Handling

### Error Types

```rust
// crates/core/src/error.rs

#[derive(Debug, thiserror::Error)]
pub enum SynapseNetError {
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),
    
    #[error("AI/Embedding error: {0}")]
    Embedding(#[from] EmbeddingError),
    
    #[error("Crypto error: {0}")]
    Crypto(#[from] CryptoError),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Batch processing error: {0}")]
    Batch(String),
    
    #[error("UI error: {0}")]
    Ui(String),
}

#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
    #[error("Failed to connect to peer: {0}")]
    ConnectionFailed(String),
    
    #[error("DHT lookup failed: {0}")]
    DhtLookupFailed(String),
    
    #[error("NAT traversal failed")]
    NatTraversalFailed,
    
    #[error("Peer timeout")]
    PeerTimeout,
}

#[derive(Debug, thiserror::Error)]
pub enum EmbeddingError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    
    #[error("Model load failed: {0}")]
    ModelLoadFailed(String),
    
    #[error("Inference failed: {0}")]
    InferenceFailed(String),
    
    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
}
```

### Error Recovery Strategies

**Network Errors:**
- Retry with exponential backoff
- Fall back to relay if direct connection fails
- Cache failed peers to avoid repeated attempts
- Notify user of connectivity issues

**Embedding Errors:**
- Fall back to CPU if GPU fails
- Fall back to smaller model if OOM
- Skip corrupted files in batch processing
- Log errors but continue processing

**Storage Errors:**
- Implement write-ahead logging
- Periodic integrity checks
- Automatic backup before migrations
- Clear error messages for disk full

**UI Errors:**
- Graceful degradation (disable features)
- User-friendly error messages
- Retry buttons for transient errors
- Detailed logs for debugging



## Testing Strategy

### Unit Tests

**Core Components:**
- Grain creation and validation
- Signature verification (classical + PQC)
- PoE score calculation
- Vector similarity functions
- Configuration parsing

**AI Components:**
- Model loading and inference
- Batch processing pipeline
- Multi-model manager
- GPU provider detection

**Storage Components:**
- SQLite operations
- Parquet read/write
- HNSW index operations
- Migration scripts

**Network Components:**
- DHT operations
- NAT detection
- Peer clustering
- Message serialization

### Integration Tests

**End-to-End Workflows:**
1. Add grain → Store → Index → Search
2. Batch import → Process → Verify
3. Network discovery → Connect → Sync
4. PoE calculation → Reward distribution

**Multi-Node Tests:**
- 2-node P2P communication
- DHT peer discovery
- Grain synchronization
- Cluster formation

### UI Tests

**Tauri Frontend:**
- Component rendering tests (React Testing Library)
- User interaction flows (Playwright)
- IPC command tests
- State management tests

**Visual Regression:**
- Screenshot comparison
- Graph rendering validation
- Responsive layout tests

### Performance Tests

**Benchmarks:**
- Embedding speed (CPU vs GPU)
- Search latency (1K, 10K, 100K grains)
- Batch processing throughput
- Network message overhead
- Memory usage under load

**Load Tests:**
- Concurrent user simulation
- Large batch imports
- High-frequency searches
- Network stress testing

### Security Tests

**Crypto Validation:**
- Signature verification
- Key generation randomness
- PQC implementation correctness

**Network Security:**
- Peer authentication
- Message integrity
- DoS resistance
- Sybil attack mitigation



## Implementation Phases

### Phase 1: Tauri Foundation (Week 1)

**Goals:**
- Set up Tauri project structure
- Implement basic UI views
- Connect to existing backend via IPC
- Basic add/search functionality

**Deliverables:**
- `crates/tauri-app` crate
- React frontend with 3 main views
- IPC command handlers
- Desktop app builds for Mac/Windows/Linux

### Phase 2: Multi-Model AI (Week 1-2)

**Goals:**
- Implement MultiModelManager
- Support 3 model sizes
- Model auto-selection
- Update storage schema

**Deliverables:**
- `crates/ai/src/multi_model.rs`
- Model configuration system
- Migration script for existing grains
- Model download/management UI

### Phase 3: Global P2P Mesh (Week 2)

**Goals:**
- Integrate Kademlia DHT
- Implement NAT traversal
- Peer clustering algorithm
- Bootstrap node setup

**Deliverables:**
- `crates/p2p/src/dht.rs`
- `crates/p2p/src/nat.rs`
- Updated network behaviour
- Bootstrap node infrastructure

### Phase 4: Batch Processing (Week 2)

**Goals:**
- File format parsers
- Batch pipeline implementation
- Progress tracking
- GPU optimization

**Deliverables:**
- `crates/ai/src/batch.rs`
- Document parsers (txt, md, pdf, json)
- CLI batch commands
- UI batch import interface

### Phase 5: PoE v2 (Week 3)

**Goals:**
- Implement 3-component scoring
- Reuse tracking system
- Reward calculation
- Anti-gaming measures

**Deliverables:**
- `crates/economy/src/poe_v2.rs`
- `crates/economy/src/reuse_tracker.rs`
- Updated reward distribution
- PoE dashboard in UI

### Phase 6: Integration & Polish (Week 3)

**Goals:**
- End-to-end testing
- Performance optimization
- Documentation
- Release preparation

**Deliverables:**
- Comprehensive test suite
- User documentation
- Developer documentation
- Release builds



## Architecture Diagrams

### Tauri IPC Flow

```
┌─────────────────────────────────────────────────────────┐
│                    Frontend (React)                     │
│                                                         │
│  User Action (e.g., "Add Grain")                       │
│         │                                               │
│         ▼                                               │
│  invoke('add_grain', { text, tags })                   │
└─────────────────────────────────────────────────────────┘
                        │
                        │ IPC (JSON)
                        ▼
┌─────────────────────────────────────────────────────────┐
│                 Tauri Backend (Rust)                    │
│                                                         │
│  #[tauri::command]                                      │
│  async fn add_grain(...)                               │
│         │                                               │
│         ▼                                               │
│  AppState → MultiModelManager → embed()                │
│         │                                               │
│         ▼                                               │
│  Create Grain → Sign → Store → Index                   │
│         │                                               │
│         ▼                                               │
│  Return Result<AddGrainResponse>                       │
└─────────────────────────────────────────────────────────┘
                        │
                        │ IPC (JSON)
                        ▼
┌─────────────────────────────────────────────────────────┐
│                    Frontend (React)                     │
│                                                         │
│  Update UI with result                                 │
│  Show success notification                             │
└─────────────────────────────────────────────────────────┘
```

### P2P Network Topology

```
                    ┌──────────────┐
                    │  Bootstrap   │
                    │    Nodes     │
                    └──────────────┘
                           │
                           │ Initial connection
                           ▼
        ┌──────────────────────────────────────┐
        │         Kademlia DHT                 │
        │  (Distributed peer routing table)    │
        └──────────────────────────────────────┘
                 │         │         │
        ┌────────┘         │         └────────┐
        ▼                  ▼                  ▼
   ┌────────┐         ┌────────┐         ┌────────┐
   │ Node A │◄───────►│ Node B │◄───────►│ Node C │
   │ (Mac)  │         │(Windows)│         │(Linux) │
   └────────┘         └────────┘         └────────┘
        │                  │                  │
        │                  │                  │
        └──────────────────┼──────────────────┘
                           │
                    Gossipsub Topics
                  (grain sync, queries)
```

### Batch Processing Flow

```
User selects folder
        │
        ▼
┌───────────────────┐
│  Scan Directory   │
│  Find all files   │
└───────────────────┘
        │
        ▼
┌───────────────────┐
│  Detect Formats   │
│  .txt .md .pdf    │
└───────────────────┘
        │
        ▼
┌───────────────────┐     ┌──────────────┐
│  Parse Content    │────►│  Text Chunks │
│  Extract text     │     │  (in memory) │
└───────────────────┘     └──────────────┘
        │                         │
        ▼                         ▼
┌───────────────────┐     ┌──────────────┐
│  Batch Embedding  │◄────│  GPU Queue   │
│  Process N at once│     │  (parallel)  │
└───────────────────┘     └──────────────┘
        │
        ▼
┌───────────────────┐
│  Create Grains    │
│  Sign each one    │
└───────────────────┘
        │
        ▼
┌───────────────────┐
│  Bulk Insert      │
│  Store + Index    │
└───────────────────┘
        │
        ▼
┌───────────────────┐
│  Progress Update  │
│  Notify UI        │
└───────────────────┘
```

### PoE v2 Calculation Flow

```
New Grain Added
        │
        ▼
┌─────────────────────────────────────────┐
│         Calculate Novelty               │
│  • Search K nearest neighbors           │
│  • Compute avg similarity               │
│  • Novelty = 1 - avg_similarity         │
└─────────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────────┐
│        Calculate Coherence              │
│  • Find related grains (sim > 0.6)      │
│  • Analyze topic diversity              │
│  • Score = connections × diversity      │
└─────────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────────┐
│         Calculate Reuse                 │
│  • Query access log                     │
│  • Count unique peers                   │
│  • Score = log(frequency + diversity)   │
└─────────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────────┐
│         Weighted Combination            │
│  total = 0.4×novelty + 0.3×coherence    │
│          + 0.3×reuse                    │
└─────────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────────┐
│         NGT Reward                      │
│  reward = 1 + (total × 10)              │
│  (1-11 NGT range)                       │
└─────────────────────────────────────────┘
```

## Security Considerations

### Threat Model

**Network Attacks:**
- Sybil attacks (fake peers)
- Eclipse attacks (isolate node)
- DoS (flood with requests)
- Man-in-the-middle

**Economic Attacks:**
- Reward gaming (spam low-quality grains)
- Collusion (fake reuse counts)
- Front-running (copy valuable grains)

**Data Attacks:**
- Privacy leaks (metadata analysis)
- Content poisoning (malicious grains)
- Storage exhaustion

### Mitigations

**Network Security:**
- PQC encryption for all messages
- Peer reputation system
- Rate limiting per peer
- Connection limits
- DHT security extensions

**Economic Security:**
- Novelty threshold (min 0.3)
- Reuse decay over time
- Peer diversity requirements
- Stake-based participation (future)

**Data Security:**
- Local-first architecture
- Explicit sharing consent
- Encrypted storage option
- Metadata minimization

## Performance Targets

### Latency

- UI launch: < 2 seconds
- Add grain: < 500ms (CPU), < 200ms (GPU)
- Search (10K grains): < 500ms
- Batch import: > 100 docs/min (GPU)

### Throughput

- Concurrent users: 1000+ per node
- Network messages: 10K/sec
- Storage writes: 1000 grains/sec

### Resource Usage

- Memory: < 1GB (medium model)
- Disk: ~10MB per 1000 grains
- Network: < 1Mbps idle, < 10Mbps active
- CPU: < 20% idle, < 80% active

## Backward Compatibility

### v0.3 → v0.4 Migration

**Storage Migration:**
```rust
pub fn migrate_v03_to_v04(data_dir: &Path) -> Result<()> {
    // 1. Read v0.3 grains
    // 2. Add new metadata fields (model, dimensions)
    // 3. Set default values for existing grains
    // 4. Update schema version
    // 5. Rebuild HNSW index
}
```

**Network Compatibility:**
- v0.4 nodes can communicate with v0.3 nodes
- Graceful degradation for missing features
- Protocol version negotiation
- Deprecation timeline (6 months)

**API Compatibility:**
- All v0.3 REST endpoints maintained
- New endpoints added with `/v2/` prefix
- CLI commands backward compatible
- Configuration auto-migration

## Open Questions

1. **Model Distribution:** How to distribute large embedding models? (CDN, P2P, IPFS?)
2. **Cross-Model Search:** Should we implement learned projections in v0.4 or v0.5?
3. **Bootstrap Infrastructure:** Who runs bootstrap nodes? (Foundation, community, both?)
4. **Relay Nodes:** Incentivize relay operation? (PoE rewards for relays?)
5. **Mobile Support:** Tauri mobile in v0.5? (iOS/Android)
6. **Token Economics:** When to launch NGT token? (Testnet first?)

## Success Metrics

### Adoption Metrics

- Downloads: 10K in first month
- Active nodes: 1K daily
- Grains created: 100K in first month
- Network size: 500+ peers

### Technical Metrics

- Uptime: > 99%
- Search accuracy: > 90%
- User satisfaction: > 4.5/5
- Bug reports: < 10 critical/month

### Community Metrics

- GitHub stars: 1K+
- Contributors: 20+
- Discord members: 500+
- Documentation completeness: 100%

---

**Design Status:** Complete and ready for implementation planning.
