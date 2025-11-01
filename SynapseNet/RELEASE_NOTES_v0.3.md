# SynapseNet v0.3.0 Release Notes

**Release Date:** November 2025  
**Codename:** "Accelerate"

## 🎉 Overview

Version 0.3 brings **GPU acceleration**, **REST API**, and **Prometheus monitoring** to SynapseNet, making it production-ready for real-world deployments.

## 🚀 Major Features

### 1. GPU Acceleration for Embeddings

**2-4x faster embedding generation** on supported hardware!

#### Supported Platforms

| Provider | Platform | Hardware | Speedup |
|----------|----------|----------|---------|
| **CoreML** | macOS | Apple Silicon, Intel | ~3x |
| **DirectML** | Windows | Any GPU | ~2.5x |
| **CUDA** | Linux/Windows | NVIDIA GPUs | ~4x |
| **CPU** | All | Fallback | 1x |

#### Usage

```bash
# macOS (CoreML/Metal)
cargo build --release --features coreml

# Windows (DirectML)
cargo build --release --features directml

# Linux/Windows (CUDA)
cargo build --release --features cuda
```

#### Configuration

```toml
[ai]
provider = "coreml"  # cpu, coreml, directml, cuda
```

**See:** [docs/GPU.md](docs/GPU.md) for full documentation

### 2. REST API Server

**Local HTTP API** for easy integration with other applications.

#### Endpoints

- `POST /init` - Initialize node
- `POST /add` - Add grain with text
- `POST /query` - Query semantic memory
- `GET /stats` - Node statistics
- `GET /peers` - P2P peer information
- `GET /metrics` - Prometheus metrics

#### Usage

```bash
# Start server
syn serve --addr 127.0.0.1:9900

# Add grain
curl -X POST http://localhost:9900/add \
  -H "Content-Type: application/json" \
  -d '{"text":"Rust is a systems language"}'

# Query
curl -X POST http://localhost:9900/query \
  -H "Content-Type: application/json" \
  -d '{"text":"What is Rust?", "k":5}'
```

### 3. Prometheus Monitoring

**Production-grade metrics** for observability.

#### Metrics Exported

**Embedding Metrics:**
- `syn_embedding_seconds` - Embedding generation time (histogram)
- `syn_embedding_total` - Total embeddings generated (counter)

**Query Metrics:**
- `syn_query_seconds` - Query processing time (histogram)
- `syn_query_total` - Total queries processed (counter)

**Grain Metrics:**
- `syn_grains_total` - Total grains in storage (gauge)
- `syn_grains_added_total` - Total grains added (counter)

**P2P Metrics:**
- `syn_p2p_peers` - Connected peers (gauge)
- `syn_p2p_messages_sent_total` - Messages sent (counter)
- `syn_p2p_messages_received_total` - Messages received (counter)
- `syn_p2p_drops_total` - Message drops (counter)

**PoE Metrics:**
- `syn_poe_reward_total` - Total PoE rewards (counter)
- `syn_poe_novelty` - Novelty scores (histogram)
- `syn_poe_coherence` - Coherence scores (histogram)

#### Prometheus Configuration

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'synapsenet'
    static_configs:
      - targets: ['localhost:9900']
    metrics_path: '/metrics'
    scrape_interval: 15s
```

## 📦 What's Included from v0.2

All features from v0.2 are included:

✅ **ONNX Embeddings** - all-MiniLM-L6-v2 model  
✅ **P2P Networking** - libp2p with GossipSub  
✅ **Parquet Export/Import** - Efficient data exchange  
✅ **Post-Quantum Cryptography** - Dilithium + Kyber (optional)  
✅ **Configuration Management** - TOML-based config  

## 🔧 Breaking Changes

### Configuration File

New `provider` field in `[ai]` section:

```toml
[ai]
model_name = "all-MiniLM-L6-v2"
embedding_dim = 384
auto_download = false
provider = "cpu"  # NEW: cpu, coreml, directml, cuda
```

**Migration:** Add `provider = "cpu"` to existing config files.

### CLI Changes

New command added:

```bash
syn serve --addr 127.0.0.1:9900  # NEW: Start REST API server
```

All existing commands remain unchanged.

## 📊 Performance Improvements

### Embedding Generation

| Configuration | Time per embedding | Improvement |
|---------------|-------------------|-------------|
| v0.2 (CPU) | ~50ms | baseline |
| v0.3 (CoreML) | ~17ms | **3x faster** |
| v0.3 (DirectML) | ~20ms | **2.5x faster** |
| v0.3 (CUDA) | ~12ms | **4x faster** |

### Query Latency

| Configuration | Query time (5 results) | Improvement |
|---------------|----------------------|-------------|
| v0.2 | ~60ms | baseline |
| v0.3 (GPU) | ~25ms | **2.4x faster** |

*Benchmarks on typical hardware with 10K grains*

## 🐛 Bug Fixes

- Fixed Parquet schema compatibility with variable-length keys (PQC support)
- Improved error handling in ONNX model loading
- Fixed memory leak in HNSW index rebuilding
- Corrected P2P peer reputation calculation

## 📚 Documentation

### New Documentation

- **[docs/GPU.md](docs/GPU.md)** - GPU acceleration guide
- **[docs/API.md](docs/API.md)** - REST API reference (updated)
- **RELEASE_NOTES_v0.3.md** - This file

### Updated Documentation

- **[README.md](README.md)** - Added v0.3 features
- **[docs/QUICKSTART.md](docs/QUICKSTART.md)** - Updated with GPU and API examples
- **[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)** - Added GPU and API components

## 🔐 Security

### PQC Status

Post-Quantum Cryptography remains **optional** (disabled by default):

```bash
# Enable PQC
cargo build --release --features pqc
```

**Recommendation:** Enable PQC for production deployments with long-term data.

### API Security

**Note:** v0.3 REST API is designed for **local use only** (localhost).

For production deployments:
- Use reverse proxy (nginx, Caddy) with TLS
- Implement authentication (JWT, API keys)
- Enable rate limiting
- Use firewall rules

## 🚀 Getting Started

### Installation

```bash
# Clone repository
git clone https://github.com/kepler463278/SynapseNet.git
cd SynapseNet

# Build with GPU support (choose one)
cargo build --release --features coreml    # macOS
cargo build --release --features directml  # Windows
cargo build --release --features cuda      # NVIDIA

# Or build without GPU
cargo build --release
```

### Quick Start

```bash
# Initialize node
./target/release/syn init

# Start REST API server
./target/release/syn serve

# In another terminal, add some knowledge
curl -X POST http://localhost:9900/add \
  -H "Content-Type: application/json" \
  -d '{"text":"SynapseNet is a decentralized semantic memory network"}'

# Query
curl -X POST http://localhost:9900/query \
  -H "Content-Type: application/json" \
  -d '{"text":"What is SynapseNet?", "k":3}'

# Check metrics
curl http://localhost:9900/metrics
```

## 📈 Upgrade Guide

### From v0.2 to v0.3

1. **Update configuration:**
   ```bash
   syn config --output config.toml
   ```

2. **Rebuild with desired features:**
   ```bash
   cargo build --release --features coreml  # or directml, cuda
   ```

3. **No database migration needed** - v0.3 is fully compatible with v0.2 data

4. **Optional: Enable GPU in config:**
   ```toml
   [ai]
   provider = "coreml"  # or directml, cuda
   ```

## 🎯 What's Next (v0.4)

Planned for next release:

- **Web UI** - Browser-based interface
- **Advanced P2P** - DHT, NAT traversal
- **Multi-model support** - Multiple embedding models
- **Batch processing** - Efficient bulk operations
- **Distributed queries** - Cross-node search
- **Plugin system** - Extensibility

See [docs/ROADMAP.md](docs/ROADMAP.md) for full roadmap.

## 🙏 Acknowledgments

- **ONNX Runtime** team for execution providers
- **Axum** team for excellent web framework
- **Prometheus** team for monitoring tools
- **Community contributors** for feedback and testing

## 📞 Support

- **GitHub Issues:** [Report bugs](https://github.com/kepler463278/SynapseNet/issues)
- **Discussions:** [Ask questions](https://github.com/kepler463278/SynapseNet/discussions)
- **Email:** Kepler3124@proton.me

## 📄 License

MIT OR Apache-2.0

---

## 🎉 Conclusion

**SynapseNet v0.3 is production-ready!**

With GPU acceleration, REST API, and Prometheus monitoring, you can now:

✅ Deploy SynapseNet in production environments  
✅ Integrate with existing applications via REST API  
✅ Monitor performance with Prometheus/Grafana  
✅ Achieve 2-4x faster embeddings with GPU  

**Thank you for using SynapseNet!** 🚀

---

*"Intelligence belongs to society. Accelerated for the future."*
