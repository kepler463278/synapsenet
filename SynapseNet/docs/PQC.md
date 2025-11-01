# Post-Quantum Cryptography in SynapseNet

## Overview

SynapseNet supports **Post-Quantum Cryptography (PQC)** to protect against future quantum computer attacks. This document explains the PQC implementation and how to use it.

## Why Post-Quantum Cryptography?

Quantum computers pose a threat to current cryptographic systems:

- **Classical signatures (ed25519)**: Vulnerable to Shor's algorithm
- **Classical key exchange (Noise/ECDH)**: Vulnerable to quantum attacks
- **Hash functions (blake3)**: Remain secure (Grover's algorithm only provides quadratic speedup)

SynapseNet implements PQC to ensure long-term security of:
- Grain signatures
- Link signatures  
- P2P connection establishment

## Supported Algorithms

### Signatures

| Algorithm | Type | Public Key | Signature | Security Level |
|-----------|------|------------|-----------|----------------|
| **ed25519** | Classical | 32 bytes | 64 bytes | ~128-bit classical |
| **Dilithium5** | PQC | 2592 bytes | 4595 bytes | NIST Level 5 (256-bit quantum) |

**Dilithium** is a NIST-standardized lattice-based signature scheme (CRYSTALS-Dilithium).

### Key Exchange

| Algorithm | Type | Public Key | Ciphertext | Shared Secret | Security Level |
|-----------|------|------------|------------|---------------|----------------|
| **Noise (X25519)** | Classical | 32 bytes | 32 bytes | 32 bytes | ~128-bit classical |
| **Kyber1024** | PQC | 1568 bytes | 1568 bytes | 32 bytes | NIST Level 5 (256-bit quantum) |

**Kyber** is a NIST-standardized lattice-based KEM (CRYSTALS-Kyber).

## Architecture

### Crypto Abstraction Layer

SynapseNet uses a unified crypto interface that supports both classical and PQC:

```rust
pub trait SigningKeyTrait {
    fn sign(&self, message: &[u8]) -> Vec<u8>;
    fn public_key(&self) -> Vec<u8>;
    fn backend(&self) -> CryptoBackend;
}

pub enum UnifiedSigningKey {
    Classical(ClassicalSigningKey),    // ed25519
    PostQuantum(PqcSigningKey),        // Dilithium
}
```

### Components

```
┌─────────────────────────────────────────────────────────┐
│                     SynapseNet Node                      │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────────┐         ┌──────────────┐             │
│  │   Grain      │         │     P2P      │             │
│  │ Signatures   │         │  Handshake   │             │
│  └──────────────┘         └──────────────┘             │
│         │                        │                      │
│         ▼                        ▼                      │
│  ┌──────────────┐         ┌──────────────┐             │
│  │  Dilithium   │         │    Kyber     │             │
│  │  (or ed25519)│         │ (or Noise)   │             │
│  └──────────────┘         └──────────────┘             │
│                                                          │
│  PoE, Storage, HNSW remain unchanged                    │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

## Usage

### Feature Flags

Enable PQC with Cargo features:

```toml
# Classical crypto only (default)
synapsenet = { version = "0.2", features = ["classical-crypto"] }

# Post-quantum crypto
synapsenet = { version = "0.2", features = ["pqc"] }

# Both (hybrid mode)
synapsenet = { version = "0.2", features = ["classical-crypto", "pqc"] }
```

### Creating Grains with PQC

```rust
use synapsenet_core::{CryptoBackend, UnifiedSigningKey, Grain, GrainMeta};

// Generate PQC signing key
let signing_key = UnifiedSigningKey::generate(CryptoBackend::PostQuantum);

// Create grain metadata
let meta = GrainMeta {
    author_pk: signing_key.public_key().try_into().unwrap(),
    ts_unix_ms: chrono::Utc::now().timestamp_millis(),
    tags: vec!["quantum-safe".to_string()],
    mime: "text/plain".to_string(),
    lang: "en".to_string(),
    title: Some("My PQC Grain".to_string()),
    summary: None,
};

// Create grain (signature is automatically PQC)
let vec = vec![0.1, 0.2, 0.3]; // embedding
let grain = Grain::new_with_unified_key(vec, meta, &signing_key)?;
```

### P2P with Kyber KEM

```rust
use synapsenet_p2p::KyberHandshake;

// Alice initiates connection
let mut alice = KyberHandshake::new();
let bob_public_key = /* Bob's public key */;

let initiation = alice.initiate(&bob_public_key)?;
// Send initiation to Bob

// Bob responds
let mut bob = KyberHandshake::new();
let response = bob.respond(&initiation)?;
// Send response to Alice

// Alice finalizes
alice.finalize(&response)?;

// Both now have shared secret
let shared_secret = alice.shared_secret().unwrap();
let encryption_key = alice.derive_key().unwrap();
```

## Performance Comparison

### Signature Generation

| Algorithm | Time | Size |
|-----------|------|------|
| ed25519 | ~50 μs | 64 bytes |
| Dilithium5 | ~200 μs | 4595 bytes |

### Signature Verification

| Algorithm | Time |
|-----------|------|
| ed25519 | ~100 μs |
| Dilithium5 | ~150 μs |

### Key Exchange

| Algorithm | Time | Bandwidth |
|-----------|------|-----------|
| Noise (X25519) | ~100 μs | 64 bytes |
| Kyber1024 | ~150 μs | 3136 bytes |

**Note**: PQC is ~2-3x slower and uses ~50-70x more bandwidth, but provides quantum resistance.

## Migration Strategy

### Phase 1: Hybrid Mode (Current)

Support both classical and PQC:
- Nodes advertise supported crypto backends
- Grains include backend metadata
- P2P connections negotiate crypto

### Phase 2: PQC Default

Make PQC the default:
- New nodes use PQC by default
- Classical crypto available for compatibility
- Gradual network transition

### Phase 3: PQC Only

Remove classical crypto:
- All nodes must support PQC
- Smaller codebase
- Maximum security

## Security Considerations

### Quantum Threat Timeline

- **2030s**: Small quantum computers (50-100 qubits)
- **2040s**: Medium quantum computers (1000+ qubits)
- **2050s**: Large quantum computers (threat to RSA/ECC)

### "Harvest Now, Decrypt Later"

Adversaries may store encrypted data today to decrypt with future quantum computers. PQC protects against this threat.

### Hybrid Signatures

For maximum security, use both classical and PQC signatures:

```rust
let classical_sig = classical_key.sign(message);
let pqc_sig = pqc_key.sign(message);
let hybrid_sig = [classical_sig, pqc_sig].concat();
```

## Implementation Details

### Dilithium Parameters

- **Dilithium5**: NIST security level 5 (256-bit quantum resistance)
- **Public key**: 2592 bytes
- **Secret key**: 4864 bytes
- **Signature**: 4595 bytes

### Kyber Parameters

- **Kyber1024**: NIST security level 5 (256-bit quantum resistance)
- **Public key**: 1568 bytes
- **Secret key**: 3168 bytes
- **Ciphertext**: 1568 bytes
- **Shared secret**: 32 bytes

### Storage Impact

PQC increases storage requirements:

| Component | Classical | PQC | Increase |
|-----------|-----------|-----|----------|
| Grain signature | 64 bytes | 4595 bytes | 72x |
| Link signature | 64 bytes | 4595 bytes | 72x |
| Public key | 32 bytes | 2592 bytes | 81x |

For 100K grains:
- Classical: ~6.4 MB signatures
- PQC: ~459 MB signatures

**Mitigation**: Use Parquet compression (reduces PQC overhead to ~10-20x).

## Testing

Run PQC demo:

```bash
# With PQC features enabled
cargo run --example pqc_demo --features pqc

# Classical only
cargo run --example pqc_demo --features classical-crypto
```

Run tests:

```bash
# Test classical crypto
cargo test --features classical-crypto

# Test PQC
cargo test --features pqc

# Test both
cargo test --all-features
```

## Future Work

### NIST Standards

- **ML-DSA** (Dilithium): Finalized 2024
- **ML-KEM** (Kyber): Finalized 2024
- **SLH-DSA** (SPHINCS+): Finalized 2024

### Optimizations

- Hardware acceleration (AVX2, NEON)
- Batch verification
- Signature aggregation
- Compressed public keys

### Additional Algorithms

- **Falcon**: Smaller signatures than Dilithium
- **SPHINCS+**: Hash-based signatures (stateless)
- **FrodoKEM**: Conservative lattice-based KEM

## References

- [NIST PQC Standardization](https://csrc.nist.gov/projects/post-quantum-cryptography)
- [CRYSTALS-Dilithium](https://pq-crystals.org/dilithium/)
- [CRYSTALS-Kyber](https://pq-crystals.org/kyber/)
- [pqcrypto Rust crates](https://github.com/rustpq/pqcrypto)

## FAQ

### Q: Should I use PQC now?

**A**: For long-term data (10+ years), yes. For short-term data, classical crypto is sufficient.

### Q: What's the performance impact?

**A**: ~2-3x slower, ~50-70x more bandwidth. Acceptable for most use cases.

### Q: Can I mix classical and PQC nodes?

**A**: Yes, in hybrid mode. Nodes negotiate the best common crypto.

### Q: Is PQC standardized?

**A**: Yes, NIST finalized ML-DSA (Dilithium) and ML-KEM (Kyber) in 2024.

### Q: What about hash functions?

**A**: blake3 remains secure against quantum computers (Grover's algorithm only provides quadratic speedup).

---

*"Quantum-safe intelligence for the future."*
