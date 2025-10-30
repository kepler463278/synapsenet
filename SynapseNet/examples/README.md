# SynapseNet Examples

This directory contains example code demonstrating various features of SynapseNet.

## Running Examples

```bash
# Basic usage (grains, storage, search)
cargo run --example basic_usage

# Policy engine (safe responses)
cargo run --example policy_demo

# Proof of Emergence (rewards)
cargo run --example poe_demo

# P2P networking (peer discovery, grain broadcasting)
cargo run --example p2p_demo

# P2P grain broadcasting (with signature verification)
cargo run --example p2p_broadcast

# P2P distributed queries (KNN search across peers)
cargo run --example p2p_query

# P2P with storage integration (NEW!)
cargo run --example p2p_with_storage

# Export grains to Parquet
cargo run --bin syn -- export -o export_dir

# Import grains from Parquet
cargo run --bin syn -- import -i export_dir
```

## Examples

### 1. basic_usage.rs

Demonstrates:
- Creating grains with signatures
- Storing in SQLite
- Building HNSW index
- Querying semantic memory
- Verifying signatures

### 2. policy_demo.rs

Demonstrates:
- Policy classification (OK/AnalysisOnly/Curated)
- Safe response generation
- Consequence-based answers for harmful queries

### 3. poe_demo.rs

Demonstrates:
- Proof of Emergence calculation
- NGT reward distribution
- Anti-spam mechanisms
- Ledger management

### 4. p2p_demo.rs

Demonstrates:
- P2P swarm initialization with libp2p
- mDNS peer discovery on local network
- GossipSub topic subscription
- Grain broadcasting to peers
- Noise protocol encryption

Run multiple instances to see peer discovery:
```bash
# Terminal 1
cargo run --example p2p_demo

# Terminal 2
cargo run --example p2p_demo -- --port 9001
```

### 5. p2p_broadcast.rs

Demonstrates:
- Broadcasting grains with signatures
- Receiving and verifying grains
- Duplicate detection
- Rate limiting (100 grains/min per peer)
- Peer reputation tracking
- Automatic disconnection of bad peers

Run multiple instances to see grain propagation:
```bash
# Terminal 1
cargo run --example p2p_broadcast

# Terminal 2
cargo run --example p2p_broadcast -- --port 9001
```

### 6. p2p_query.rs

Demonstrates:
- Distributed KNN queries across peers
- Query broadcasting with unique IDs
- Response collection and merging
- Timeout handling (2 seconds default)
- Result sorting by similarity

Run multiple instances to see distributed search:
```bash
# Terminal 1
cargo run --example p2p_query

# Terminal 2
cargo run --example p2p_query -- --port 9001
```

## Next Steps

After running examples:
1. Try the CLI: `cargo run --bin syn -- --help`
2. Read the [API documentation](../docs/API.md)
3. Check the [architecture overview](../docs/ARCHITECTURE.md)

## Contributing

Have an interesting use case? Add an example and submit a PR!
