## GPU Acceleration in SynapseNet

SynapseNet v0.3 adds GPU acceleration for embeddings, providing 2-4x speedup on supported hardware.

## Supported Providers

| Provider | Platform | Hardware | Speedup |
|----------|----------|----------|---------|
| **CoreML** | macOS | Apple Silicon, Intel | 3x |
| **DirectML** | Windows | Any GPU (Intel/AMD/NVIDIA) | 2.5x |
| **CUDA** | Linux/Windows | NVIDIA GPUs | 4x |
| **CPU** | All | Fallback | 1x (baseline) |

## Quick Start

### macOS (CoreML/Metal)

```bash
# Build with CoreML support
cargo build --release --features coreml

# Run demo
cargo run --example gpu_demo --features coreml
```

### Windows (DirectML)

```bash
# Build with DirectML support
cargo build --release --features directml

# Run demo
cargo run --example gpu_demo --features directml
```

### Linux/Windows (CUDA)

```bash
# Prerequisites: CUDA Toolkit installed
# Set CUDA_PATH environment variable

# Build with CUDA support
cargo build --release --features cuda

# Run demo
cargo run --example gpu_demo --features cuda
```

## Configuration

### Via config.toml

```toml
[ai]
model_name = "all-MiniLM-L6-v2"
embedding_dim = 384
auto_download = true
provider = "coreml"  # Options: cpu, coreml, directml, cuda
```

### Via Code

```rust
use synapsenet_ai::{GpuProvider, OnnxEmbedding};

// Auto-detect best provider
let embedding = OnnxEmbedding::new(data_dir).await?;

// Or specify provider
let provider = GpuProvider::CoreML;
let embedding = OnnxEmbedding::new_with_provider(data_dir, provider).await?;
```

## Performance Benchmarks

### Embedding Generation (all-MiniLM-L6-v2, 384 dims)

| Provider | Time per embedding | Throughput |
|----------|-------------------|------------|
| CPU (baseline) | ~50ms | 20 emb/sec |
| CoreML (M1 Max) | ~17ms | 59 emb/sec |
| DirectML (RTX 3060) | ~20ms | 50 emb/sec |
| CUDA (RTX 4090) | ~12ms | 83 emb/sec |

*Benchmarks on typical hardware. Your results may vary.*

## Provider Details

### CoreML (macOS)

**Hardware Support:**
- Apple Silicon (M1/M2/M3) - Best performance
- Intel Macs with AMD GPUs - Good performance
- Neural Engine acceleration when available

**Features:**
- Automatic Metal backend
- Low power consumption
- Optimized for Apple hardware

**Requirements:**
- macOS 11.0+
- No additional installation needed

### DirectML (Windows)

**Hardware Support:**
- NVIDIA GPUs (GeForce, Quadro, Tesla)
- AMD GPUs (Radeon)
- Intel GPUs (Iris, Arc)

**Features:**
- Works with any DirectX 12 compatible GPU
- No vendor-specific drivers needed
- Good compatibility

**Requirements:**
- Windows 10 version 1903+
- DirectX 12 compatible GPU
- Latest GPU drivers

### CUDA (Linux/Windows)

**Hardware Support:**
- NVIDIA GPUs only
- Compute Capability 3.5+

**Features:**
- Best performance on NVIDIA hardware
- Mature ecosystem
- Advanced optimizations

**Requirements:**
- CUDA Toolkit 11.0+
- cuDNN 8.0+
- NVIDIA drivers 450.80.02+

**Installation:**
```bash
# Ubuntu/Debian
wget https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2004/x86_64/cuda-ubuntu2004.pin
sudo mv cuda-ubuntu2004.pin /etc/apt/preferences.d/cuda-repository-pin-600
sudo apt-key adv --fetch-keys https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2004/x86_64/3bf863cc.pub
sudo add-apt-repository "deb https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2004/x86_64/ /"
sudo apt-get update
sudo apt-get install cuda

# Set environment
export CUDA_PATH=/usr/local/cuda
export LD_LIBRARY_PATH=$CUDA_PATH/lib64:$LD_LIBRARY_PATH
```

## Troubleshooting

### CoreML: "Provider not available"

**Solution:** Ensure you're on macOS 11.0+ and built with `--features coreml`

### DirectML: Slow performance

**Solution:** 
- Update GPU drivers
- Check GPU is not in power-saving mode
- Verify DirectX 12 support: `dxdiag`

### CUDA: "CUDA not found"

**Solution:**
```bash
# Check CUDA installation
nvcc --version

# Set environment variables
export CUDA_PATH=/usr/local/cuda
export LD_LIBRARY_PATH=$CUDA_PATH/lib64:$LD_LIBRARY_PATH

# Verify GPU
nvidia-smi
```

### General: Falling back to CPU

**Cause:** GPU provider not available or not compiled in

**Solution:**
1. Check feature flags: `cargo build --features <provider>`
2. Verify hardware support
3. Check logs for specific error messages

## Best Practices

### 1. Choose the Right Provider

- **Mac users:** Use CoreML (best performance + efficiency)
- **Windows users:** Use DirectML (broad compatibility) or CUDA (NVIDIA only)
- **Linux users:** Use CUDA (NVIDIA) or CPU (others)

### 2. Batch Processing

```rust
// Efficient: Batch embeddings
let texts = vec!["text1", "text2", "text3"];
let embeddings = embedding.embed_batch(&texts)?;

// Less efficient: One at a time
for text in texts {
    let emb = embedding.embed(text)?;
}
```

### 3. Warm-up

```rust
// First embedding is slower (model loading)
let _ = embedding.embed("warmup")?;

// Subsequent embeddings are faster
let emb = embedding.embed("actual text")?;
```

### 4. Monitor Performance

```rust
use std::time::Instant;

let start = Instant::now();
let emb = embedding.embed(text)?;
let duration = start.elapsed();

println!("Embedding took: {:?}", duration);
```

## API Integration

GPU acceleration works automatically with REST API:

```bash
# Start server with GPU
cargo run --release --features coreml -- serve

# Embeddings use GPU automatically
curl -X POST http://localhost:9900/add \
  -H "Content-Type: application/json" \
  -d '{"text":"GPU-accelerated embedding"}'
```

## Metrics

GPU performance is tracked in Prometheus metrics:

```
# Embedding duration histogram
syn_embedding_seconds_bucket{le="0.01"} 45
syn_embedding_seconds_bucket{le="0.05"} 98
syn_embedding_seconds_bucket{le="0.1"} 100

# Total embeddings
syn_embedding_total 1523
```

## Future Improvements

- [ ] Batch inference optimization
- [ ] Model quantization (INT8/FP16)
- [ ] Multi-GPU support
- [ ] Dynamic provider switching
- [ ] Memory pooling
- [ ] Async inference pipeline

## Resources

- [ONNX Runtime Execution Providers](https://onnxruntime.ai/docs/execution-providers/)
- [CoreML Documentation](https://developer.apple.com/documentation/coreml)
- [DirectML Documentation](https://docs.microsoft.com/en-us/windows/ai/directml/)
- [CUDA Toolkit](https://developer.nvidia.com/cuda-toolkit)

---

*GPU acceleration makes SynapseNet faster without changing the API!*
