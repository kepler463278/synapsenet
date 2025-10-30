# Requirements Document - SynapseNet v0.2

## Introduction

SynapseNet v0.2 introduces three critical features that transform the system from a local-only MVP to a functional distributed semantic memory network:

1. **ONNX Embeddings** - Replace dummy hash-based vectors with real semantic embeddings
2. **P2P Network** - Enable grain exchange between nodes using libp2p
3. **Parquet Export** - Provide snapshot/backup functionality for knowledge bases

This release focuses on making SynapseNet production-ready for small-scale deployments and testing.

## Glossary

- **System**: SynapseNet node software
- **User**: Person running a SynapseNet node
- **Grain**: Atomic unit of semantic knowledge with vector embedding
- **Embedding**: Dense vector representation of text (384-dimensional float32)
- **ONNX**: Open Neural Network Exchange format for ML models
- **P2P**: Peer-to-peer network communication
- **GossipSub**: libp2p publish-subscribe protocol
- **Parquet**: Columnar storage format for data export
- **Node**: Single instance of SynapseNet software
- **Peer**: Another node in the P2P network

## Requirements

### Requirement 1: ONNX Embedding Integration

**User Story:** As a user, I want my grains to have real semantic embeddings so that search results are actually relevant to my queries.

#### Acceptance Criteria

1. WHEN the System initializes, THE System SHALL download the all-MiniLM-L6-v2 ONNX model if not present locally
2. WHEN a user adds text via `syn add`, THE System SHALL generate a 384-dimensional embedding using the ONNX model
3. WHEN a user queries via `syn query`, THE System SHALL generate a query embedding using the ONNX model
4. THE System SHALL cache the ONNX model in the data directory to avoid re-downloading
5. WHEN the ONNX model fails to load, THE System SHALL provide a clear error message with troubleshooting steps

### Requirement 2: ONNX Model Management

**User Story:** As a user, I want the embedding model to be managed automatically so that I don't need to manually download or configure it.

#### Acceptance Criteria

1. THE System SHALL store ONNX models in `{data_dir}/models/` directory
2. WHEN the model file is missing, THE System SHALL download it from HuggingFace
3. WHEN the model file is corrupted, THE System SHALL re-download it
4. THE System SHALL verify model file integrity using SHA256 checksum
5. THE System SHALL support CPU-only inference by default

### Requirement 3: Embedding Performance

**User Story:** As a user, I want embedding generation to be fast enough for interactive use so that adding grains doesn't feel slow.

#### Acceptance Criteria

1. THE System SHALL generate embeddings for text under 512 tokens in less than 500ms on CPU
2. THE System SHALL batch multiple embedding requests when possible
3. THE System SHALL provide progress feedback for large batch operations
4. WHEN embedding generation takes longer than 2 seconds, THE System SHALL display a progress indicator

### Requirement 4: P2P Network Initialization

**User Story:** As a user, I want my node to automatically discover and connect to peers so that I can participate in the network without manual configuration.

#### Acceptance Criteria

1. WHEN the System starts with P2P enabled, THE System SHALL initialize a libp2p swarm
2. THE System SHALL listen on a configurable TCP port (default: 9000)
3. THE System SHALL use mDNS to discover peers on the local network
4. THE System SHALL support manual peer addition via multiaddr
5. WHEN no peers are found after 30 seconds, THE System SHALL log a warning

### Requirement 5: Grain Broadcasting

**User Story:** As a user, I want my grains to be shared with peers automatically so that the network builds collective knowledge.

#### Acceptance Criteria

1. WHEN a user adds a grain via `syn add`, THE System SHALL broadcast the grain to all connected peers via GossipSub
2. THE System SHALL subscribe to the `grains.put` topic on startup
3. WHEN a grain is received from a peer, THE System SHALL verify the signature before storing
4. WHEN a grain signature is invalid, THE System SHALL reject the grain and log a warning
5. THE System SHALL not broadcast grains that already exist locally

### Requirement 6: Grain Synchronization

**User Story:** As a user, I want my node to receive grains from peers so that I benefit from the collective knowledge of the network.

#### Acceptance Criteria

1. WHEN a grain is received via GossipSub, THE System SHALL store it in the local database
2. WHEN a grain is received via GossipSub, THE System SHALL add it to the HNSW index
3. THE System SHALL acknowledge received grains by publishing to `grains.ack` topic
4. WHEN a duplicate grain is received, THE System SHALL ignore it
5. THE System SHALL rate-limit grain ingestion to prevent spam (max 100 grains/minute per peer)

### Requirement 7: Distributed Query

**User Story:** As a user, I want to query not just my local grains but also grains from connected peers so that I can access the full network knowledge.

#### Acceptance Criteria

1. WHEN a user queries via `syn query`, THE System SHALL first search local grains
2. WHEN local results are insufficient (less than k results), THE System SHALL broadcast a query to peers via `query.knn` topic
3. THE System SHALL wait up to 2 seconds for peer responses
4. THE System SHALL merge local and peer results by similarity score
5. THE System SHALL display the source node for each result

### Requirement 8: P2P Security

**User Story:** As a user, I want the P2P network to be secure so that malicious nodes cannot corrupt my knowledge base.

#### Acceptance Criteria

1. THE System SHALL verify ed25519 signatures on all received grains
2. THE System SHALL reject grains with invalid signatures
3. THE System SHALL maintain a reputation score for each peer
4. WHEN a peer sends invalid grains repeatedly, THE System SHALL disconnect from that peer
5. THE System SHALL use Noise protocol for encrypted P2P connections

### Requirement 9: Parquet Export

**User Story:** As a user, I want to export my grains to Parquet format so that I can backup my knowledge base and analyze it with external tools.

#### Acceptance Criteria

1. WHEN a user runs `syn export`, THE System SHALL export all grains to Parquet files
2. THE System SHALL create one Parquet file per 10,000 grains
3. THE System SHALL include all grain fields: id, vec, meta, sig
4. THE System SHALL compress Parquet files using Snappy compression
5. THE System SHALL write Parquet files to the specified output directory

### Requirement 10: Parquet Import

**User Story:** As a user, I want to import grains from Parquet files so that I can restore backups or migrate data between nodes.

#### Acceptance Criteria

1. WHEN a user runs `syn import`, THE System SHALL read Parquet files from the specified directory
2. THE System SHALL verify grain signatures before importing
3. WHEN a grain signature is invalid, THE System SHALL skip that grain and log a warning
4. THE System SHALL rebuild the HNSW index after import
5. THE System SHALL display import progress (grains imported / total)

### Requirement 11: Configuration Management

**User Story:** As a user, I want to configure P2P and embedding settings so that I can customize the node behavior.

#### Acceptance Criteria

1. THE System SHALL read configuration from `{data_dir}/config.toml`
2. THE System SHALL support configuration for: P2P port, bootstrap peers, model path, embedding batch size
3. WHEN config file is missing, THE System SHALL use default values
4. WHEN config file has invalid syntax, THE System SHALL display a clear error message
5. THE System SHALL validate configuration values on startup

### Requirement 12: Monitoring and Metrics

**User Story:** As a user, I want to see network statistics so that I can monitor my node's health and connectivity.

#### Acceptance Criteria

1. WHEN a user runs `syn peers`, THE System SHALL display: peer count, connected peers, grains received, grains sent
2. WHEN a user runs `syn stats`, THE System SHALL display: total grains, index size, database size, model loaded
3. THE System SHALL log P2P events (peer connected, peer disconnected, grain received)
4. THE System SHALL track embedding generation time and log performance metrics
5. THE System SHALL expose metrics in a structured format for monitoring tools

## Non-Functional Requirements

### Performance
- Embedding generation: < 500ms for 512 tokens on CPU
- P2P message latency: < 100ms on local network
- Parquet export: > 1000 grains/second
- Query response time: < 2 seconds including P2P

### Scalability
- Support 100+ connected peers
- Handle 100,000+ grains per node
- Process 100 grains/minute from network

### Reliability
- Graceful degradation when P2P unavailable
- Automatic reconnection to peers
- Data integrity verification (signatures, checksums)

### Security
- All grains cryptographically signed
- Encrypted P2P connections (Noise protocol)
- Signature verification on all received data
- Peer reputation tracking

### Usability
- Zero-configuration P2P discovery (mDNS)
- Automatic model download
- Clear error messages
- Progress indicators for long operations

## Dependencies

- ONNX Runtime (ort crate)
- all-MiniLM-L6-v2 model from HuggingFace
- libp2p with GossipSub
- Apache Parquet (parquet crate)
- tokio async runtime

## Constraints

- CPU-only inference (GPU support in v0.3)
- Local network P2P only (no NAT traversal yet)
- Single ONNX model (no model switching)
- No Byzantine fault tolerance (honest majority assumed)

## Success Criteria

v0.2 is successful when:
1. Users can add grains with real semantic embeddings
2. Nodes can discover and connect to peers automatically
3. Grains are shared across the network
4. Queries return relevant results from local and peer grains
5. Users can export/import their knowledge base
6. All tests pass
7. Documentation is updated

## Out of Scope (Future Versions)

- GPU acceleration (v0.3)
- NAT traversal (v0.3)
- REST API (v0.3)
- Multiple embedding models (v0.4)
- Byzantine fault tolerance (v1.0)
- Mobile apps (v2.0+)
