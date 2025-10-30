# SynapseNet Architecture

## Overview

SynapseNet is a decentralized semantic memory network where nodes exchange knowledge grains and earn rewards through Proof of Emergence (PoE).

## Core Components

### 1. Grain (Knowledge Unit)

```
Grain {
    id: blake3(vec || meta || author_pk)
    vec: Vec<f32>           // Embedding vector
    meta: GrainMeta         // Metadata
    sig: Signature          // ed25519 signature
}
```

**Properties:**
- Immutable once created
- Cryptographically signed
- Self-contained semantic unit

### 2. Link (Semantic Connection)

```
Link {
    from: grain_id
    to: grain_id
    weight: f32             // Strength (0..1)
    rationale: Option<String>
    sig: Signature
}
```

**Purpose:**
- Connect related grains
- Build semantic graph
- Enable traversal and discovery

### 3. Graph (Local Knowledge Base)

```
Graph {
    grains: HashMap<ID, Grain>
    links: HashMap<ID, Vec<Link>>
}
```

**Operations:**
- Add/retrieve grains
- Query by similarity
- Compute connectivity metrics

### 4. Storage Layer

**SQLite Schema:**
```sql
grains (id, vec, meta, sig, created_at)
links (from_id, to_id, weight, rationale, sig)
credits (grain_id, node_pk, ngt, reason, ts)
peers (peer_id, public_key, last_seen, reputation)
```

**Vector Index:**
- HNSW (default): Pure Rust, no dependencies
- FAISS (optional): Better performance, requires C++ libs

### 5. P2P Network (libp2p)

**Protocols:**
- **GossipSub**: Message propagation
- **mDNS**: Local peer discovery
- **Noise**: Encrypted transport
- **Yamux**: Stream multiplexing

**Topics:**
- `grains.put`: Publish new grains
- `grains.ack`: Acknowledge receipt
- `query.knn`: KNN search requests
- `query.resp`: Search responses

### 6. Proof of Emergence (PoE)

**Formula:**
```
NGT(g) = α * N(g) + β * C(g) + γ * log(1 + R(g))
```

**Components:**
- **N(g)**: Novelty = 1 - max_cos_sim(g, existing)
- **C(g)**: Coherence = avg_cos_sim(g, relevant_cluster)
- **R(g)**: Reuse count (decay over time)

**Anti-spam:**
- Threshold filters (τ_novelty, τ_coherence)
- Rate limiting
- Signature verification

### 7. Governance

**Policy Classes:**
- **OK**: Normal response
- **AnalysisOnly**: Consequences only, no instructions
- **Curated**: Human review required

**Implementation:**
```rust
PolicyEngine::classify(query) -> PolicyClass
ConsequenceAnalyzer::analyze(query) -> Response
```

### 8. Economy

**NGT Ledger:**
- Track node balances
- Award credits for contributions
- Compute total supply

**Reputation System:**
- Score peers based on behavior
- Trust threshold for interactions
- Decay over time

## Data Flow

### Adding a Grain

```
1. User: syn add "text"
2. Generate embedding (ONNX/dummy)
3. Create Grain with signature
4. Store in SQLite
5. Add to HNSW index
6. Calculate PoE metrics
7. Award NGT credits
8. (Future) Broadcast to P2P network
```

### Querying

```
1. User: syn query "question"
2. Generate query embedding
3. HNSW KNN search (local)
4. (Future) P2P query expansion
5. Merge results
6. Apply policy filter
7. Return formatted response
```

## Security Model

### Cryptography

- **Signatures**: ed25519-dalek (EdDSA)
- **Hashing**: blake3
- **Random**: OS-provided CSPRNG

### Threat Model

**Assumptions:**
- Honest majority of nodes
- No Byzantine fault tolerance (v0.1)
- Local-first security

**Mitigations:**
- Signature verification
- Rate limiting
- Reputation scoring
- Policy engine

## Performance

### Benchmarks (Target)

- **Add grain**: < 10ms
- **KNN search (1000 grains)**: < 50ms
- **P2P message**: < 100ms latency

### Scalability

- **Local**: 100K+ grains per node
- **Network**: 1000+ nodes (v1.0 target)
- **Index**: HNSW scales to millions

## Platform Support

| Feature | macOS | Linux | Windows |
|---------|-------|-------|---------|
| Core | ✅ | ✅ | ✅ |
| SQLite | ✅ | ✅ | ✅ |
| HNSW | ✅ | ✅ | ✅ |
| P2P | ✅ | ✅ | ✅ |
| ONNX CPU | ✅ | ✅ | ✅ |
| Metal | ✅ | ❌ | ❌ |
| CUDA | ❌ | ✅ | ❌ |
| DirectML | ❌ | ❌ | ✅ |

## Future Enhancements

### v0.2
- ONNX embeddings (all-MiniLM-L6-v2)
- Full P2P implementation
- DevNet testing

### v0.3
- GPU acceleration (Metal/CUDA/DirectML)
- FAISS index option
- REST API

### v1.0
- Desktop UI (Tauri)
- Mobile support
- Federation protocol

## References

- [GENESIS.txt](../GENESIS.txt) - Founding principles
- [SECURITY.md](../SECURITY.md) - Security practices
- [libp2p](https://libp2p.io/) - P2P networking
- [HNSW](https://arxiv.org/abs/1603.09320) - Vector index algorithm
