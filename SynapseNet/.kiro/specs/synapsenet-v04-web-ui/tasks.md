# Implementation Plan - SynapseNet v0.4

This implementation plan breaks down the v0.4 design into discrete, actionable coding tasks. Each task builds incrementally on previous work and references specific requirements from the requirements document.

## Task List

- [x] 1. Set up Tauri project structure and core integration
  - Create new `crates/tauri-app` workspace member with Tauri 2.0 dependencies
  - Configure Tauri build system for Mac, Windows, and Linux targets
  - Set up TypeScript + React frontend with Vite bundler
  - Implement basic AppState structure with Arc-wrapped shared state
  - Create IPC bridge foundation with error handling
  - _Requirements: 1.1, 1.2, 6.1, 6.2_

- [x] 2. Implement core Tauri command handlers
- [x] 2.1 Create add_grain command handler
  - Implement `add_grain` Tauri command that accepts text and tags
  - Integrate with existing embedding system and storage
  - Return grain ID and timing metrics
  - _Requirements: 1.3, 1.7_

- [x] 2.2 Create search_grains command handler
  - Implement `search_grains` command with query text and k parameter
  - Use existing HNSW index for semantic search
  - Return results with similarity scores and metadata
  - _Requirements: 1.4, 1.5_

- [x] 2.3 Create stats and network info commands
  - Implement `get_stats` command returning grain count, peers, storage usage
  - Implement `get_network_peers` command returning connected peer list
  - Implement `get_grain_details` command for individual grain retrieval
  - _Requirements: 1.1, 8.1_

- [x] 3. Build React frontend UI components
- [x] 3.1 Create AddGrainView component
  - Build text input area with expandable textarea
  - Add tag input with comma-separated parsing
  - Implement submit button with loading state
  - Show success/error notifications using toast library
  - _Requirements: 1.3, 1.7_

- [x] 3.2 Create SearchView component
  - Build semantic search input with debouncing
  - Display results list with similarity scores
  - Create result preview cards showing title, tags, timestamp
  - Add filter controls for tags and date ranges
  - _Requirements: 1.4, 1.5_

- [x] 3.3 Create GraphView component
  - Integrate D3.js or Cytoscape.js for graph visualization
  - Render nodes as grains and edges as semantic connections
  - Implement zoom, pan, and node click interactions
  - Add cluster visualization by topic similarity
  - _Requirements: 1.6_

- [x] 3.4 Create StatsView dashboard
  - Display total grains count, network peers, storage usage
  - Show PoE rewards earned (placeholder for now)
  - Create recent activity timeline
  - _Requirements: 1.1, 8.1_

- [x] 4. Implement multi-model AI system
- [x] 4.1 Create ModelInfo and ModelSize enums
  - Define ModelSize enum (Small, Medium, Large)
  - Create ModelInfo struct with name, dimensions, file size, RAM requirements
  - Add GPU support flag to ModelInfo
  - _Requirements: 2.1, 2.2_

- [x] 4.2 Implement MultiModelManager
  - Create MultiModelManager struct with HashMap of loaded models
  - Implement load_model method that initializes ONNX models
  - Add embed_with_model method for explicit model selection
  - Implement embed_auto method that uses active model
  - Add list_loaded_models and get_best_model_for_hardware methods
  - _Requirements: 2.1, 2.3, 2.4_

- [x] 4.3 Update GrainMeta with model information
  - Add embedding_model field (String) to GrainMeta
  - Add embedding_dimensions field (usize) to GrainMeta
  - Update Grain creation to populate model metadata
  - Implement storage migration for existing grains
  - _Requirements: 2.5_

- [x] 4.4 Create model configuration system
  - Define ModelConfig struct in config module
  - Add AiConfig with models list and default model
  - Implement model auto-detection based on hardware
  - Create model download/management utilities
  - _Requirements: 2.4_



- [x] 5. Implement global P2P mesh networking
- [x] 5.1 Integrate Kademlia DHT into network behaviour
  - Add Kademlia behaviour to SynapseNetBehaviour struct
  - Configure DHT with MemoryStore and appropriate k-value
  - Implement DHT initialization and bootstrap logic
  - _Requirements: 3.1, 3.2_

- [x] 5.2 Create DhtDiscovery module
  - Implement DhtDiscovery struct wrapping Kademlia behaviour
  - Add bootstrap method that connects to bootstrap peers
  - Implement announce_topic for publishing node interests
  - Create find_peers_for_topic method using DHT queries
  - Add get_closest_peers method for proximity-based discovery
  - _Requirements: 3.2, 3.6_

- [x] 5.3 Implement NAT traversal system
  - Add AutoNAT behaviour to detect NAT type
  - Integrate Circuit Relay v2 client for relay connections
  - Create NatTraversal struct with autonat and relay client
  - Implement detect_nat_type method
  - Add establish_connection method with fallback to relay
  - _Requirements: 3.2, 3.3_

- [x] 5.4 Create peer clustering algorithm
  - Define PeerCluster struct with topic, peers, and timestamp
  - Implement add_peer and remove_inactive_peers methods
  - Create clustering algorithm based on topic similarity
  - Add get_best_peers method that prioritizes cluster members
  - Integrate clustering into peer selection logic
  - _Requirements: 3.5_

- [x] 5.5 Set up bootstrap node infrastructure
  - Create hardcoded list of bootstrap peer addresses
  - Implement DNS seed fallback mechanism
  - Add bootstrap node health monitoring
  - _Requirements: 3.6_

- [x] 6. Implement batch processing pipeline
- [x] 6.1 Create document parser interfaces
  - Define SupportedFormat enum (PlainText, Markdown, Pdf, Json, Csv)
  - Create DocumentParser trait with parse and supported_extensions methods
  - Implement PlainTextParser for .txt files
  - Implement MarkdownParser for .md files
  - Implement PdfParser using pdf-extract crate
  - Implement JsonParser for .json files
  - _Requirements: 4.2_

- [x] 6.2 Implement BatchProcessor core
  - Create BatchProcessor struct with embedding manager, store, index, signing key
  - Define BatchConfig with batch_size, parallel_workers, use_gpu, model_name
  - Implement file scanner that recursively finds files in directory
  - Add format detection logic based on file extensions
  - _Requirements: 4.1, 4.2_

- [x] 6.3 Build batch embedding pipeline
  - Implement batch embedding that processes N chunks at once
  - Add GPU queue for parallel embedding generation
  - Create grain creation loop with signing
  - Implement bulk insert into storage and index
  - _Requirements: 4.1, 4.3, 4.6_

- [x] 6.4 Add progress tracking and reporting
  - Define BatchProgress struct with file/chunk counters and timing
  - Create BatchResult struct with success/error counts
  - Implement progress channel using mpsc for real-time updates
  - Add estimated time remaining calculation
  - _Requirements: 4.5_

- [x] 6.5 Create batch import Tauri commands
  - Implement batch_import command that accepts paths and config
  - Add progress streaming to frontend via Tauri events
  - Create batch_import UI in frontend with progress bar
  - _Requirements: 4.1, 4.5_

- [x] 7. Implement PoE v2 economic model
- [x] 7.1 Create PoE v2 core structures
  - Define PoEScore struct with novelty, coherence, reuse, total fields
  - Create PoEv2Engine struct with store, index, reuse_tracker
  - Define RewardWeights struct with configurable weight values
  - _Requirements: 5.1_

- [x] 7.2 Implement novelty score calculation
  - Create calculate_novelty function that searches K nearest neighbors
  - Compute average similarity to neighbors
  - Return novelty as 1.0 - avg_similarity
  - Handle edge case of first grain (novelty = 1.0)
  - _Requirements: 5.2_

- [x] 7.3 Implement coherence score calculation
  - Create calculate_coherence function that finds related grains
  - Extract topics from related grains using tags and metadata
  - Calculate topic diversity score
  - Compute coherence as connections × diversity
  - _Requirements: 5.3_

- [x] 7.4 Implement reuse tracking system
  - Create ReuseTracker struct with access_log HashMap
  - Define AccessEvent struct with timestamp, peer_id, access_type
  - Implement record_access method for logging grain access
  - Create calculate_reuse_score using log-scale frequency and diversity
  - Add access tracking to search and retrieval operations
  - _Requirements: 5.4_

- [x] 7.5 Implement final reward calculation
  - Create calculate_ngt_reward function with weighted score combination
  - Apply reward weights (0.4 novelty, 0.3 coherence, 0.3 reuse)
  - Calculate final NGT reward (1 + weighted_score × 10)
  - Implement anti-gaming measures (novelty threshold, rate limiting)
  - Add reward decay over time for reuse scores
  - _Requirements: 5.1, 5.5, 5.6, 5.7_

- [x] 7.6 Update GrainMeta with PoE scores
  - Add poe_score field (Option<PoEScore>) to GrainMeta
  - Add access_count field (u64) to GrainMeta
  - Update storage schema with PoE-related fields
  - Implement PoE score caching and updates
  - _Requirements: 5.1_

- [x] 8. Update storage schema and migrations
- [x] 8.1 Create grain_access tracking table
  - Add grain_access table with grain_id, peer_id, access_type, timestamp
  - Create indexes on grain_id and timestamp columns
  - Implement insert and query methods for access tracking
  - _Requirements: 5.4_

- [x] 8.2 Create embedding_models metadata table
  - Add embedding_models table with name, dimensions, file_size_mb, loaded_at
  - Implement model registration on load
  - Add query methods for model information
  - _Requirements: 2.1, 2.4_

- [x] 8.3 Create peer_clusters table
  - Add peer_clusters table with topic, peer_id, similarity, last_seen
  - Implement cluster persistence and retrieval
  - Add cleanup for stale cluster entries
  - _Requirements: 3.5_

- [x] 8.4 Implement v0.3 to v0.4 migration
  - Create migration script that reads v0.3 grains
  - Add default values for new metadata fields (model, dimensions)
  - Update schema version in database
  - Rebuild HNSW index with new metadata
  - _Requirements: 10.1, 10.3_

- [x] 9. Enhance configuration system
- [x] 9.1 Create comprehensive Config structure
  - Define Config struct with network, ai, economy, ui sections
  - Create NetworkConfig with DHT, relay, and peer settings
  - Define AiConfig with model configurations
  - Add EconomyConfig with PoE weights and thresholds
  - Create UiConfig with theme and view preferences
  - _Requirements: 2.4, 3.1, 5.1_

- [x] 9.2 Implement configuration loading and validation
  - Add TOML configuration file parsing
  - Implement default configuration generation
  - Add configuration validation with helpful error messages
  - Create configuration migration for v0.3 configs
  - _Requirements: 10.3_

- [x] 9.3 Add configuration UI in Tauri app
  - Create settings view in frontend
  - Implement configuration editing interface
  - Add save/reset configuration commands
  - Show configuration validation errors
  - _Requirements: 9.3_

- [x] 10. Implement error handling and recovery
- [x] 10.1 Define comprehensive error types
  - Create SynapseNetError enum with all error categories
  - Define NetworkError with connection, DHT, NAT errors
  - Create EmbeddingError with model and inference errors
  - Add BatchError for batch processing failures
  - _Requirements: 8.2, 8.3, 8.4_

- [x] 10.2 Implement error recovery strategies
  - Add retry logic with exponential backoff for network errors
  - Implement GPU fallback to CPU on errors
  - Add model fallback to smaller models on OOM
  - Create error logging and user notifications
  - _Requirements: 8.2, 8.3_

- [x] 10.3 Add error handling to UI
  - Implement graceful degradation for missing features
  - Create user-friendly error messages
  - Add retry buttons for transient errors
  - Show detailed error logs in debug mode
  - _Requirements: 8.5_



- [x] 11. Implement REST API v2 endpoints
- [x] 11.1 Create v2 API routes
  - Add `/v2/models` endpoint for listing available models
  - Create `/v2/batch/import` endpoint for batch operations
  - Add `/v2/poe/scores` endpoint for PoE score queries
  - Implement `/v2/network/peers` with cluster information
  - _Requirements: 10.2_

- [x] 11.2 Maintain v0.3 API compatibility
  - Ensure all v0.3 endpoints continue to work
  - Add deprecation warnings to old endpoints
  - Document migration path from v1 to v2 API
  - _Requirements: 10.2_

- [x] 12. Add comprehensive logging and monitoring
- [x] 12.1 Enhance logging throughout codebase
  - Add structured logging with tracing crate
  - Implement log levels (debug, info, warn, error)
  - Add performance metrics logging
  - Create log rotation and management
  - _Requirements: 8.1, 8.2_

- [x] 12.2 Integrate Prometheus metrics
  - Use existing metrics module from v0.3
  - Add metrics for batch processing throughput
  - Track P2P network statistics (peers, messages)
  - Monitor PoE calculation performance
  - _Requirements: 8.1_

- [x] 12.3 Create monitoring dashboard in UI
  - Display real-time metrics in StatsView
  - Show network health indicators
  - Add performance graphs (embedding speed, search latency)
  - _Requirements: 8.1_

- [x] 13. Build cross-platform installers
- [x] 13.1 Configure Tauri bundler
  - Set up macOS .dmg installer with code signing
  - Configure Windows .msi installer with proper permissions
  - Create Linux .deb and .AppImage packages
  - Add application icons and metadata
  - _Requirements: 6.1, 6.4_

- [x] 13.2 Implement auto-update system
  - Integrate Tauri updater for automatic updates
  - Create update server or use GitHub releases
  - Add update notification in UI
  - Implement update download and installation
  - _Requirements: 6.1_

- [x] 14. Create user documentation
- [x] 14.1 Write user guide
  - Create getting started guide with installation steps
  - Document core features (add, search, batch import)
  - Add troubleshooting section for common issues
  - Include screenshots and examples
  - _Requirements: 9.1, 9.2, 9.4_

- [x] 14.2 Create in-app tutorial
  - Implement first-run tutorial in UI
  - Add interactive walkthrough of main features
  - Create example knowledge grains for demonstration
  - Add contextual help tooltips
  - _Requirements: 9.1, 9.2, 9.3_

- [x] 14.3 Write API documentation
  - Document all Tauri commands with examples
  - Create REST API v2 reference
  - Add code examples in multiple languages
  - Document configuration options
  - _Requirements: 9.4_

- [x] 15. Implement comprehensive testing
- [x] 15.1 Create unit tests for new components
  - Write tests for MultiModelManager
  - Test PoE v2 score calculations
  - Add tests for batch processing pipeline
  - Test DHT and NAT traversal logic
  - _Requirements: 2.1, 3.1, 4.1, 5.1_

- [x] 15.2 Add integration tests
  - Test end-to-end add grain → search workflow
  - Create multi-node P2P communication tests
  - Test batch import with various file formats
  - Verify PoE reward calculation across network
  - _Requirements: 1.1, 3.6, 4.1, 5.1_

- [x] 15.3 Implement UI tests
  - Add React component tests with Testing Library
  - Create E2E tests with Playwright
  - Test IPC command interactions
  - Add visual regression tests
  - _Requirements: 1.1, 8.1_

- [x] 15.4 Create performance benchmarks
  - Benchmark embedding speed (CPU vs GPU)
  - Test search latency at different scales (1K, 10K, 100K grains)
  - Measure batch processing throughput
  - Profile memory usage under load
  - _Requirements: 8.2, 8.3, 8.4, 8.5_

- [x] 16. Optimize performance
- [x] 16.1 Optimize embedding pipeline
  - Implement batch embedding optimization for GPU
  - Add embedding caching for repeated text
  - Optimize tokenization performance
  - _Requirements: 4.6, 8.2_

- [x] 16.2 Optimize search performance
  - Tune HNSW index parameters (M, ef_construction)
  - Implement result caching for common queries
  - Add query result pagination
  - _Requirements: 8.2, 8.3_

- [x] 16.3 Optimize network performance
  - Implement message batching for P2P
  - Add compression for large messages
  - Optimize DHT query routing
  - _Requirements: 3.6, 8.1_

- [x] 16.4 Optimize storage performance
  - Implement write batching for bulk inserts
  - Add database connection pooling
  - Optimize Parquet file organization
  - _Requirements: 8.3_

- [x] 17. Security hardening
- [x] 17.1 Implement peer authentication
  - Add PQC-based peer identity verification
  - Implement challenge-response authentication
  - Add peer reputation tracking
  - _Requirements: 7.3_

- [x] 17.2 Add rate limiting and DoS protection
  - Implement per-peer rate limiting
  - Add connection limits
  - Create request throttling for expensive operations
  - _Requirements: 5.6_

- [x] 17.3 Implement data validation
  - Add input validation for all user inputs
  - Validate grain signatures on receipt
  - Check message integrity in P2P layer
  - _Requirements: 7.3_

- [x] 18. Prepare for release
- [x] 18.1 Create release builds
  - Build optimized release binaries for all platforms
  - Test installers on clean systems
  - Verify code signing and notarization (macOS)
  - _Requirements: 6.1, 6.4_

- [x] 18.2 Write release notes
  - Document all new features in v0.4
  - List breaking changes and migration steps
  - Include known issues and workarounds
  - Add upgrade instructions from v0.3
  - _Requirements: 10.1, 10.2, 10.3, 10.4_

- [x] 18.3 Create release announcement
  - Write blog post announcing v0.4
  - Create demo video showing new features
  - Prepare social media posts
  - Update project website
  - _Requirements: 1.1, 2.1, 3.1, 4.1, 5.1_

---

## Implementation Notes

### Task Dependencies

- Tasks 1-3 (Tauri setup and UI) can be done in parallel with tasks 4-7 (backend features)
- Task 8 (storage migrations) depends on tasks 4, 5, 7
- Task 9 (configuration) should be done early to support other features
- Tasks 15-17 (testing, optimization, security) should be ongoing throughout development
- Task 18 (release) is the final phase after all features are complete

### Testing Strategy

- Write tests alongside implementation (not as separate phase)
- Focus on core functionality first, edge cases later
- Use integration tests to verify feature interactions
- Performance testing should validate against requirements

### Optional Tasks

Tasks marked with * are optional and can be deferred to v0.4.1 if time is limited. Core functionality should be prioritized.

### Estimated Timeline

- Weeks 1-2: Tasks 1-6 (Tauri, UI, Multi-model, P2P, Batch)
- Week 3: Tasks 7-10 (PoE v2, Storage, Config, Errors)
- Week 4: Tasks 11-14 (API, Logging, Installers, Docs)
- Week 5: Tasks 15-17 (Testing, Optimization, Security)
- Week 6: Task 18 (Release preparation)

Total: ~6 weeks for complete v0.4 implementation
