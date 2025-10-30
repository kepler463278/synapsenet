# Implementation Plan - SynapseNet v0.2

This document breaks down the v0.2 implementation into discrete, actionable coding tasks. Each task builds incrementally on previous work.

## Phase 1: ONNX Embeddings (Foundation)

### - [ ] 1. Set up ONNX dependencies and model management
- Add `ort`, `tokenizers`, `reqwest` to workspace dependencies in root `Cargo.toml`
- Add dependencies to `crates/ai/Cargo.toml`
- Create `crates/ai/src/model_manager.rs` with ModelManager struct
- Implement `ensure_model()` to download all-MiniLM-L6-v2 from HuggingFace
- Implement `verify_checksum()` for model integrity verification
- _Requirements: 1.1, 2.1, 2.2, 2.4_

### - [ ] 2. Implement ONNX embedding generation
- [ ] 2.1 Create `crates/ai/src/onnx_embed.rs` with OnnxEmbedding struct
  - Implement `new()` to load ONNX model and tokenizer
  - Implement `embed()` for single text embedding
  - Implement `embed_batch()` for batch processing
  - Implement `dim()` to return 384
  - _Requirements: 1.2, 1.3, 3.2_

- [ ] 2.2 Add error handling for ONNX operations
  - Create `OnnxError` enum in `crates/ai/src/lib.rs`
  - Handle model loading failures with clear error messages
  - Handle tokenization errors
  - _Requirements: 1.5, 2.5_

- [ ] 2.3 Integrate ONNX embeddings into CLI
  - Modify `crates/cli/src/main.rs` `add_grain()` to use OnnxEmbedding
  - Modify `query_grains()` to use OnnxEmbedding for query vector
  - Replace dummy_embedding() calls with real embeddings
  - Add progress indicator for embedding generation
  - _Requirements: 1.2, 1.3, 3.4_

- [ ]* 2.4 Write tests for ONNX embedding
  - Test model loading in `crates/ai/src/onnx_embed.rs`
  - Test embedding generation for sample texts
  - Test batch processing
  - Test error handling for missing model
  - _Requirements: 1.1, 1.2, 1.3_

### - [ ] 3. Add embedding performance monitoring
- Add timing metrics to embedding generation
- Log embedding time at DEBUG level
- Display warning if embedding takes > 2 seconds
- _Requirements: 3.1, 3.4_

## Phase 2: P2P Network (Core Networking)

### - [ ] 4. Implement P2P swarm initialization
- [ ] 4.1 Update `crates/p2p/src/swarm.rs` with full implementation
  - Implement `SynapseSwarm::new()` with libp2p setup
  - Add Noise protocol for encryption
  - Add mDNS for peer discovery
  - Add Identify protocol for peer info
  - Configure GossipSub with proper parameters
  - _Requirements: 4.1, 4.2, 4.3, 8.5_

- [ ] 4.2 Implement swarm event loop
  - Implement `run()` method to handle swarm events
  - Handle peer connected/disconnected events
  - Handle mDNS discovery events
  - Log P2P events at INFO level
  - _Requirements: 4.4, 4.5, 12.3_

- [ ]* 4.3 Write tests for swarm initialization
  - Test swarm creation
  - Test configuration parsing
  - Mock peer discovery
  - _Requirements: 4.1, 4.2_

### - [ ] 5. Implement grain broadcasting
- [ ] 5.1 Add grain broadcast functionality
  - Implement `broadcast_grain()` in SynapseSwarm
  - Serialize grain to NetworkMessage::GrainPut
  - Publish to `grains.put` GossipSub topic
  - Track sent grains to avoid duplicates
  - _Requirements: 5.1, 5.5_

- [ ] 5.2 Implement grain reception handler
  - Subscribe to `grains.put` topic on startup
  - Deserialize received NetworkMessage
  - Verify grain signature before storing
  - Store valid grains in SQLite and HNSW index
  - Publish acknowledgment to `grains.ack` topic
  - _Requirements: 5.2, 5.3, 5.4, 6.1, 6.2, 6.3_

- [ ] 5.3 Add duplicate detection and rate limiting
  - Check if grain already exists before storing
  - Implement rate limiter (100 grains/min per peer)
  - Reject grains exceeding rate limit
  - _Requirements: 5.5, 6.4, 6.5_

- [ ]* 5.4 Write tests for grain broadcasting
  - Test grain serialization/deserialization
  - Test signature verification
  - Test duplicate detection
  - Test rate limiting
  - _Requirements: 5.1, 5.3, 5.4, 6.5_

### - [ ] 6. Implement distributed queries
- [ ] 6.1 Add query broadcasting
  - Implement `query_peers()` in SynapseSwarm
  - Serialize query vector to NetworkMessage::QueryKnn
  - Publish to `query.knn` topic
  - Wait up to 2 seconds for responses
  - _Requirements: 7.2, 7.3_

- [ ] 6.2 Implement query response handler
  - Subscribe to `query.knn` topic
  - Perform local KNN search for received queries
  - Publish results to `query.resp` topic
  - Include source node in response
  - _Requirements: 7.1, 7.5_

- [ ] 6.3 Merge local and peer results
  - Collect responses from peers
  - Merge with local results by similarity score
  - Sort by similarity descending
  - Return top k results
  - _Requirements: 7.4_

- [ ]* 6.4 Write tests for distributed queries
  - Test query serialization
  - Test result merging
  - Test timeout handling
  - _Requirements: 7.2, 7.3, 7.4_

### - [ ] 7. Add peer reputation and security
- Create `PeerInfo` struct in `crates/p2p/src/peer.rs`
- Track reputation score for each peer
- Decrease reputation on invalid grain
- Disconnect peers with reputation < -10
- _Requirements: 8.1, 8.2, 8.3, 8.4_

## Phase 3: Parquet Export/Import

### - [ ] 8. Implement Parquet export
- [ ] 8.1 Create Parquet exporter
  - Create `crates/storage/src/parquet_io.rs`
  - Add `parquet` and `arrow` to `crates/storage/Cargo.toml`
  - Implement `ParquetExporter` struct
  - Implement `export()` to write grains to Parquet files
  - Use Snappy compression
  - Batch 10,000 grains per file
  - _Requirements: 9.1, 9.2, 9.3, 9.4, 9.5_

- [ ] 8.2 Add export CLI command
  - Add `Export` command to CLI in `crates/cli/src/main.rs`
  - Implement `export_grains()` function
  - Display progress (grains exported / total)
  - Show export statistics (files created, bytes written)
  - _Requirements: 9.1, 9.5_

- [ ]* 8.3 Write tests for Parquet export
  - Test export of 100 grains
  - Test export of 10,000+ grains (multiple files)
  - Test compression
  - Verify Parquet file structure
  - _Requirements: 9.1, 9.2, 9.3, 9.4_

### - [ ] 9. Implement Parquet import
- [ ] 9.1 Create Parquet importer
  - Implement `ParquetImporter` struct in `parquet_io.rs`
  - Implement `import()` to read grains from Parquet files
  - Verify grain signatures during import
  - Skip grains with invalid signatures
  - Rebuild HNSW index after import
  - _Requirements: 10.1, 10.2, 10.3, 10.4_

- [ ] 9.2 Add import CLI command
  - Add `Import` command to CLI
  - Implement `import_grains()` function
  - Display progress (grains imported / total)
  - Show import statistics (imported, skipped, invalid)
  - _Requirements: 10.1, 10.5_

- [ ]* 9.3 Write tests for Parquet import
  - Test import of exported grains (round-trip)
  - Test signature verification
  - Test handling of corrupted files
  - Test index rebuild
  - _Requirements: 10.1, 10.2, 10.3, 10.4_

## Phase 4: Configuration and Integration

### - [ ] 10. Implement configuration management
- [ ] 10.1 Create configuration system
  - Create `crates/core/src/config.rs`
  - Define `Config` struct with all settings
  - Implement `load()` to read from `config.toml`
  - Implement `save()` to write config
  - Implement `validate()` for config validation
  - _Requirements: 11.1, 11.2, 11.3, 11.4, 11.5_

- [ ] 10.2 Integrate config into CLI
  - Load config on startup in `main.rs`
  - Use config values for P2P, embedding, storage
  - Create default config if missing
  - Display error for invalid config
  - _Requirements: 11.2, 11.3, 11.4_

- [ ]* 10.3 Write tests for configuration
  - Test config loading
  - Test default values
  - Test validation
  - Test invalid config handling
  - _Requirements: 11.1, 11.2, 11.5_

### - [ ] 11. Add monitoring and metrics
- [ ] 11.1 Implement metrics collection
  - Create `NodeMetrics` struct in `crates/core/src/metrics.rs`
  - Track grains total, local, remote
  - Track peers connected
  - Track embedding generation time
  - Track query time
  - _Requirements: 12.1, 12.2, 12.4_

- [ ] 11.2 Add stats CLI command
  - Add `Stats` command to CLI
  - Implement `show_stats()` function
  - Display all metrics in readable format
  - _Requirements: 12.2_

- [ ] 11.3 Enhance peers command
  - Update `show_peers()` to display detailed peer info
  - Show grains received/sent per peer
  - Show peer reputation
  - _Requirements: 12.1_

### - [ ] 12. End-to-end integration and testing
- [ ] 12.1 Create multi-node E2E test
  - Create `tests/e2e_p2p.rs`
  - Start 3 nodes with P2P enabled
  - Add grain on node1
  - Verify grain appears on node2 and node3
  - Query from node2, verify results include node1 grains
  - _Requirements: 5.1, 6.1, 7.1_

- [ ] 12.2 Create embedding E2E test
  - Add grain with real text
  - Query with semantically similar text
  - Verify results are relevant (not just hash-based)
  - _Requirements: 1.2, 1.3_

- [ ] 12.3 Create export/import E2E test
  - Export 1000 grains
  - Create new node
  - Import grains
  - Verify all grains present and searchable
  - _Requirements: 9.1, 10.1_

- [ ]* 12.4 Performance benchmarks
  - Benchmark embedding generation (100 texts)
  - Benchmark P2P grain propagation
  - Benchmark Parquet export (100k grains)
  - Verify all meet performance requirements
  - _Requirements: 3.1, 9.1_

### - [ ] 13. Update documentation
- Update README.md with v0.2 features
- Update QUICKSTART.md with ONNX setup
- Create P2P_GUIDE.md for network configuration
- Update API.md with new commands
- Update CHANGELOG.md with v0.2 changes
- _Requirements: All_

### - [ ] 14. Prepare release
- Run all tests and verify they pass
- Run clippy and fix warnings
- Run cargo fmt
- Update version to 0.2.0 in all Cargo.toml files
- Create git tag v0.2.0
- Build release binaries for macOS/Linux/Windows
- Write release notes

## Task Dependencies

```
1 → 2 → 3 (ONNX embeddings)
        ↓
4 → 5 → 6 → 7 (P2P network)
        ↓
8 → 9 (Parquet export/import)
        ↓
10 → 11 (Config and metrics)
        ↓
12 → 13 → 14 (Integration, docs, release)
```

## Estimated Timeline

- **Phase 1 (ONNX)**: 2 weeks
- **Phase 2 (P2P)**: 2 weeks
- **Phase 3 (Parquet)**: 1 week
- **Phase 4 (Integration)**: 1 week
- **Total**: 6 weeks

## Notes

- Tasks marked with `*` are optional tests but recommended
- Each task should be completed and tested before moving to the next
- P2P tasks require running multiple nodes for testing
- ONNX tasks require downloading ~90MB model on first run
- Performance benchmarks should be run on representative hardware

## Success Criteria

All tasks complete when:
- [ ] All non-optional tasks are done
- [ ] All tests pass
- [ ] Performance requirements met
- [ ] Documentation updated
- [ ] Release artifacts created
