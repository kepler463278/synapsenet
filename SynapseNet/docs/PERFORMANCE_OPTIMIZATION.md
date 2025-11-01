# SynapseNet v0.4 Performance Optimization Guide

This guide provides tips and techniques for optimizing SynapseNet performance.

---

## Quick Wins

### 1. Enable GPU Acceleration

**Impact:** 3-5x faster embeddings

```toml
[ai]
provider = "coreml"  # macOS
# provider = "directml"  # Windows
# provider = "cuda"  # Linux with NVIDIA
```

### 2. Use Batch Processing

**Impact:** 4x faster than sequential

```bash
# Instead of adding one by one
syn add "text1"
syn add "text2"

# Use batch import
syn batch-import grains.json
```

### 3. Optimize HNSW Index

**Impact:** Faster search, more memory

```toml
[storage]
hnsw_m = 32              # More connections = faster search
hnsw_ef_construction = 400  # Higher quality index
```

---

## Embedding Performance

### Model Selection

**Small Models (Fast):**
- all-MiniLM-L6-v2 (384D) - ~45ms per embedding
- Use for: Real-time applications, large batches

**Medium Models (Balanced):**
- all-mpnet-base-v2 (768D) - ~120ms per embedding
- Use for: Quality-critical applications

**Large Models (Slow):**
- Custom models (1024D+) - ~300ms+ per embedding
- Use for: Maximum quality, offline processing

### Batch Size Tuning

```toml
[ai]
batch_size = 32  # Optimal for most GPUs
# Increase for powerful GPUs: 64, 128
# Decrease for CPU: 8, 16
```

**Guidelines:**
- CPU: 8-16 items
- GPU (integrated): 16-32 items
- GPU (dedicated): 32-128 items

---

## Search Performance

### HNSW Parameters

**Fast Search (Less Accurate):**
```toml
[storage]
hnsw_m = 8
hnsw_ef_construction = 100
```

**Balanced:**
```toml
[storage]
hnsw_m = 16
hnsw_ef_construction = 200
```

**High Quality (Slower):**
```toml
[storage]
hnsw_m = 32
hnsw_ef_construction = 400
```

### Search Optimization

```rust
// Limit results
let results = search("query", k=10);  // Fast

// vs
let results = search("query", k=100);  // Slower
```

---

## Memory Optimization

### Reduce Memory Usage

**1. Disable Multi-Model:**
```toml
[ai]
multi_model_enabled = false
```
**Savings:** ~500MB per additional model

**2. Limit Max Peers:**
```toml
[network]
max_peers = 20  # Default: 50
```
**Savings:** ~10MB per peer

**3. Reduce Index Size:**
```toml
[storage]
hnsw_max_elements = 100000  # Default: 1000000
```
**Savings:** ~100MB

### Memory Monitoring

```bash
# Check memory usage
ps aux | grep synapsenet

# Monitor in UI
# Stats → System → Memory Usage
```

---

## Startup Time Optimization

### Reduce Startup Time

**1. Disable Auto-Load Models:**
```toml
[[ai.additional_models]]
auto_load = false  # Load on demand
```

**2. Reduce Index Rebuild:**
```toml
[storage]
rebuild_index_on_start = false
```

**3. Lazy P2P Initialization:**
```toml
[p2p]
enabled = false  # Enable when needed
```

**Expected Startup Times:**
- Minimal config: ~1-2 seconds
- Full config: ~5-10 seconds
- With large index: ~10-30 seconds

---

## Network Performance

### P2P Optimization

**1. Limit Connections:**
```toml
[network]
max_peers = 20  # Fewer peers = less overhead
```

**2. Optimize DHT:**
```toml
[network]
dht_k = 10  # Smaller k = faster lookups
```

**3. Disable Clustering (if not needed):**
```toml
[network]
clustering_enabled = false
```

### Bandwidth Optimization

- Use smaller embedding models
- Limit batch sizes for network operations
- Enable compression (future feature)

---

## Storage Performance

### Database Optimization

**1. Regular Cleanup:**
```bash
# Clean old access events
syn cleanup --older-than 90d
```

**2. Vacuum Database:**
```bash
sqlite3 ~/.synapsenet/synapsenet.db "VACUUM;"
```

**3. Optimize Indexes:**
```bash
sqlite3 ~/.synapsenet/synapsenet.db "ANALYZE;"
```

### Parquet Export/Import

**Fast Export:**
```bash
syn export --output ./backup --compression snappy
```

**Fast Import:**
```bash
syn import --input ./backup --batch-size 100
```

---

## Profiling

### CPU Profiling

```bash
# Install flamegraph
cargo install flamegraph

# Profile embedding
cargo flamegraph --bin syn -- add "test text"

# View flamegraph.svg
```

### Memory Profiling

```bash
# Install heaptrack (Linux)
heaptrack syn add "test text"

# Analyze
heaptrack_gui heaptrack.syn.*.gz
```

### Benchmarking

```bash
# Run benchmarks
cargo bench

# Specific benchmark
cargo bench --bench embedding_bench
```

---

## Performance Targets

### v0.4.0 Targets ✅

- **Embedding:** <50ms (GPU), <200ms (CPU)
- **Search:** <20ms for 10k grains
- **Batch (100 items):** <5 seconds
- **Startup:** <10 seconds
- **Memory:** <500MB baseline

### v0.5.0 Goals

- **Embedding:** <30ms (GPU), <150ms (CPU)
- **Search:** <10ms for 100k grains
- **Batch (1000 items):** <30 seconds
- **Startup:** <5 seconds
- **Memory:** <400MB baseline

---

## Monitoring Performance

### Real-Time Monitoring

1. Open **Monitoring View** in UI
2. Enable auto-refresh (5s interval)
3. Watch metrics:
   - Embedding avg time
   - Query avg time
   - Batch processing time
   - Memory usage

### Prometheus Metrics

```bash
# Scrape metrics
curl http://localhost:9900/metrics

# Key metrics:
# - syn_embedding_seconds
# - syn_query_seconds
# - syn_batch_seconds
# - syn_grains_total
```

---

## Troubleshooting Performance Issues

### Slow Embeddings

**Symptoms:** Adding grains takes >1 second

**Solutions:**
1. Enable GPU acceleration
2. Use smaller model
3. Close other applications
4. Check CPU usage
5. Increase batch size

### Slow Search

**Symptoms:** Search takes >1 second

**Solutions:**
1. Reduce k (number of results)
2. Optimize HNSW parameters
3. Rebuild index
4. Check index size
5. Reduce grain count (archive old grains)

### High Memory Usage

**Symptoms:** >1GB RAM usage

**Solutions:**
1. Disable multi-model
2. Reduce max_peers
3. Limit HNSW max_elements
4. Close unused features
5. Restart application

### Slow Startup

**Symptoms:** >30 seconds to start

**Solutions:**
1. Disable auto-load models
2. Skip index rebuild
3. Disable P2P on startup
4. Reduce grain count
5. Check disk speed

---

## Best Practices

### Development
- Profile before optimizing
- Measure impact of changes
- Use release builds for benchmarks
- Test on target hardware

### Production
- Monitor metrics regularly
- Set up alerts for anomalies
- Regular maintenance (cleanup, vacuum)
- Keep software updated

### Scaling
- Archive old grains
- Use multiple nodes
- Distribute load
- Optimize for your use case

---

## Performance Checklist

### Before Release
- [x] Benchmark core operations
- [x] Profile memory usage
- [x] Test on target platforms
- [x] Optimize hot paths
- [x] Document performance characteristics

### After Release
- [ ] Monitor real-world performance
- [ ] Collect user feedback
- [ ] Identify bottlenecks
- [ ] Plan optimizations for v0.4.1

---

## Resources

- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Tauri Performance](https://tauri.app/v1/guides/building/performance)
- [ONNX Runtime Performance](https://onnxruntime.ai/docs/performance/)

---

**Last Updated:** 2024-10-31  
**Version:** 0.4.0
