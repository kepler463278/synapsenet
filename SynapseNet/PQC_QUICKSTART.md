# Post-Quantum Cryptography Quick Start

## 🔐 Making SynapseNet Quantum-Safe

This guide shows how to use Post-Quantum Cryptography (PQC) in SynapseNet.

## Quick Start

### 1. Build with PQC Support

```bash
# Build with PQC features
cargo build --release --features pqc

# Or add to Cargo.toml
[dependencies]
synapsenet = { version = "0.2", features = ["pqc"] }
```

### 2. Run PQC Demo

```bash
cargo run --example pqc_demo --features pqc
```

Output:
```
🔐 SynapseNet Post-Quantum Cryptography Demo
============================================================

📝 Demo 1: Classical Crypto (ed25519)
------------------------------------------------------------
Backend: Classical
Signature length: 64 bytes
Public key length: 32 bytes
✓ Signature valid: true

🔮 Demo 2: Post-Quantum Crypto (Dilithium)
------------------------------------------------------------
Backend: PostQuantum
Signature length: 4595 bytes
Public key length: 2592 bytes
✓ Signature valid: true

🔑 Demo 3: Post-Quantum Key Exchange (Kyber KEM)
------------------------------------------------------------
✓ Handshake complete!
  Secrets match: true
```

### 3. Use PQC in Your Code

```rust
use synapsenet_core::{CryptoBackend, UnifiedSigningKey};

// Generate quantum-safe key
let key = UnifiedSigningKey::generate(CryptoBackend::PostQuantum);

// Sign message
let signature = key.sign(b"Hello, quantum-safe world!");

// Verify
let public_key = key.public_key();
let verifying_key = UnifiedVerifyingKey::from_bytes(
    &public_key, 
    CryptoBackend::PostQuantum
)?;
assert!(verifying_key.verify(b"Hello, quantum-safe world!", &signature)?);
```

## What Changed?

### Before (Classical)

```rust
// ed25519 signatures
use ed25519_dalek::SigningKey;

let key = SigningKey::generate(&mut OsRng);
let signature = key.sign(message);  // 64 bytes
```

### After (PQC)

```rust
// Dilithium signatures
use synapsenet_core::{CryptoBackend, UnifiedSigningKey};

let key = UnifiedSigningKey::generate(CryptoBackend::PostQuantum);
let signature = key.sign(message);  // 4595 bytes
```

**That's it!** The rest of SynapseNet (PoE, storage, HNSW) works unchanged.

## Components

### ✅ Quantum-Safe

- **Grain signatures**: Dilithium5
- **Link signatures**: Dilithium5
- **P2P handshake**: Kyber1024 KEM
- **Hash functions**: blake3 (already quantum-resistant)

### ✅ Unchanged

- **Proof of Emergence**: Same formula
- **Storage**: SQLite + HNSW
- **Vector search**: HNSW index
- **Parquet export**: Same format

## Size Comparison

| Component | Classical | PQC | Ratio |
|-----------|-----------|-----|-------|
| Public key | 32 bytes | 2592 bytes | 81x |
| Signature | 64 bytes | 4595 bytes | 72x |
| Handshake | 64 bytes | 3136 bytes | 49x |

**Note**: Parquet compression reduces overhead to ~10-20x.

## Performance

| Operation | Classical | PQC | Ratio |
|-----------|-----------|-----|-------|
| Sign | 50 μs | 200 μs | 4x |
| Verify | 100 μs | 150 μs | 1.5x |
| Handshake | 100 μs | 150 μs | 1.5x |

**Conclusion**: PQC is 1.5-4x slower, but still fast enough for real-time use.

## Feature Flags

```toml
# Classical only (default)
default = ["classical-crypto"]

# PQC only
pqc = ["pqc-dilithium", "pqc-kyber"]

# Both (hybrid mode)
all-crypto = ["classical-crypto", "pqc"]
```

## CLI Usage

```bash
# Initialize node with PQC
syn init --crypto pqc

# Add grain with PQC signature
syn add "Quantum-safe knowledge" --crypto pqc

# Query (works with any crypto)
syn query "knowledge"

# Export (preserves crypto metadata)
syn export --output snapshots/
```

## Migration Path

### Phase 1: Hybrid (Now)

Both classical and PQC supported:
```bash
cargo build --features "classical-crypto,pqc"
```

### Phase 2: PQC Default (2025)

PQC becomes default:
```bash
cargo build  # Uses PQC by default
cargo build --features classical-crypto  # Opt-in to classical
```

### Phase 3: PQC Only (2026+)

Remove classical crypto:
```bash
cargo build  # Only PQC available
```

## Testing

```bash
# Test classical crypto
cargo test --features classical-crypto

# Test PQC
cargo test --features pqc

# Test both
cargo test --all-features
```

## Benchmarks

```bash
# Benchmark signatures
cargo bench --features pqc signature

# Benchmark handshakes
cargo bench --features pqc handshake
```

## FAQ

**Q: Why is PQC needed?**  
A: Quantum computers will break ed25519 and X25519. PQC protects against future attacks.

**Q: When will quantum computers be a threat?**  
A: Estimates range from 2030-2050. But "harvest now, decrypt later" attacks are already possible.

**Q: What's the performance cost?**  
A: ~2-4x slower, ~50-80x more storage. Acceptable for most use cases.

**Q: Can I use both classical and PQC?**  
A: Yes! Hybrid mode supports both. Nodes negotiate the best common crypto.

**Q: Is PQC standardized?**  
A: Yes. NIST finalized Dilithium (ML-DSA) and Kyber (ML-KEM) in 2024.

## Resources

- [Full PQC Documentation](docs/PQC.md)
- [NIST PQC Standards](https://csrc.nist.gov/projects/post-quantum-cryptography)
- [CRYSTALS-Dilithium](https://pq-crystals.org/dilithium/)
- [CRYSTALS-Kyber](https://pq-crystals.org/kyber/)

## Example: Full Workflow

```rust
use synapsenet_core::{CryptoBackend, UnifiedSigningKey, Grain, GrainMeta};
use synapsenet_storage::Store;
use synapsenet_p2p::{SynapseSwarm, P2pConfig, KyberHandshake};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Generate PQC keys
    let signing_key = UnifiedSigningKey::generate(CryptoBackend::PostQuantum);
    
    // 2. Create quantum-safe grain
    let meta = GrainMeta {
        author_pk: signing_key.public_key().try_into()?,
        ts_unix_ms: chrono::Utc::now().timestamp_millis(),
        tags: vec!["quantum-safe".to_string()],
        mime: "text/plain".to_string(),
        lang: "en".to_string(),
        title: Some("My PQC Grain".to_string()),
        summary: None,
    };
    
    let vec = vec![0.1, 0.2, 0.3];
    let grain = Grain::new_with_unified_key(vec, meta, &signing_key)?;
    
    // 3. Store grain
    let store = Store::new("synapsenet.db")?;
    store.insert_grain(&grain)?;
    
    // 4. P2P with Kyber KEM
    let mut swarm = SynapseSwarm::new_with_pqc(P2pConfig::default()).await?;
    swarm.broadcast_grain(&grain)?;
    
    println!("✓ Quantum-safe grain created and broadcasted!");
    
    Ok(())
}
```

---

**🎉 Congratulations! Your SynapseNet is now quantum-safe!**

*"Intelligence belongs to society. Quantum-safe for the future."*
