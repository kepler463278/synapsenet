# SynapseNet API Reference

## CLI Commands

### `syn init`

Initialize a local SynapseNet node.

```bash
syn init [--data-dir <path>]
```

**Options:**
- `--data-dir`: Data directory (default: `.synapsenet`)

**Creates:**
- `node.key`: Private key (ed25519)
- `node.pub`: Public key
- `synapsenet.db`: SQLite database

**Example:**
```bash
syn init
syn init --data-dir ~/.synapsenet
```

---

### `syn add`

Add text or file as a knowledge grain.

```bash
syn add <input> [--data-dir <path>]
```

**Arguments:**
- `<input>`: Text string or file path

**Options:**
- `--data-dir`: Data directory (default: `.synapsenet`)

**Example:**
```bash
syn add "Rust is a systems programming language"
syn add document.txt
syn add --data-dir ~/.synapsenet "Hello world"
```

---

### `syn query`

Query semantic memory.

```bash
syn query <question> [--k <num>] [--data-dir <path>]
```

**Arguments:**
- `<question>`: Query text

**Options:**
- `-k, --k`: Number of results (default: 5)
- `--data-dir`: Data directory (default: `.synapsenet`)

**Example:**
```bash
syn query "What is Rust?"
syn query "programming languages" --k 10
```

**Output:**
```
Found 3 results:

1. Similarity: 0.923
   ID: a1b2c3d4...
   Title: Rust is a systems programming language

2. Similarity: 0.856
   ID: e5f6g7h8...
   Title: Systems programming concepts
```

---

### `syn peers`

Show P2P peers and network status.

```bash
syn peers [--data-dir <path>]
```

**Example:**
```bash
syn peers
```

**Output:**
```
Status: Local mode
Peers: 0
```

---

### `syn export`

Export grains to Parquet format.

```bash
syn export [--output <path>] [--data-dir <path>]
```

**Options:**
- `-o, --output`: Output directory (default: `out`)
- `--data-dir`: Data directory (default: `.synapsenet`)

**Example:**
```bash
syn export --output snapshots/
```

---

## Rust API

### Core Types

#### Grain

```rust
use synapsenet_core::{Grain, GrainMeta};
use ed25519_dalek::SigningKey;

let signing_key = SigningKey::generate(&mut OsRng);
let author_pk = signing_key.verifying_key().to_bytes();

let meta = GrainMeta {
    author_pk,
    ts_unix_ms: 1234567890,
    tags: vec!["rust".to_string()],
    mime: "text/plain".to_string(),
    lang: "en".to_string(),
    title: Some("Example".to_string()),
    summary: None,
};

let vec = vec![0.1, 0.2, 0.3];
let grain = Grain::new(vec, meta, &signing_key)?;

// Verify signature
assert!(grain.verify()?);
```

#### Link

```rust
use synapsenet_core::Link;

let link = Link::new(
    from_id,
    to_id,
    0.8,  // weight
    Some("Related concept".to_string()),
    &signing_key,
)?;
```

#### Graph

```rust
use synapsenet_core::Graph;

let mut graph = Graph::new();
graph.add_grain(grain);
graph.add_link(link);

let metrics = graph.connectivity_metrics();
println!("Grains: {}, Links: {}", metrics.total_grains, metrics.total_links);
```

### Storage

#### Store

```rust
use synapsenet_storage::Store;

let store = Store::new("synapsenet.db")?;

// Insert grain
store.insert_grain(&grain)?;

// Get grain
let retrieved = store.get_grain(&grain.id)?;

// Get all grains
let all = store.get_all_grains()?;
```

#### HNSW Index

```rust
use synapsenet_storage::HnswIndex;

let mut index = HnswIndex::new(1000, 384);

// Add grains
for grain in grains {
    index.add(&grain)?;
}

// Search
let results = index.search(&query_vec, 10)?;
for result in results {
    println!("ID: {:?}, Similarity: {}", result.grain_id, result.similarity);
}
```

### Proof of Emergence

```rust
use synapsenet_core::ProofOfEmergence;

let poe = ProofOfEmergence::default();

let ngt = poe.calculate_ngt(
    0.8,  // novelty
    0.6,  // coherence
    5,    // reuse_count
);

println!("NGT reward: {}", ngt);
```

### Governance

```rust
use synapsenet_governance::{Policy, PolicyEngine};

let engine = PolicyEngine::new(Policy::default());

let class = engine.classify("How to make a bomb?");
// Returns: PolicyClass::AnalysisOnly

let response = engine.generate_response(class, query, &results);
```

### Economy

```rust
use synapsenet_economy::NgtLedger;

let mut ledger = NgtLedger::new();

// Award NGT
let ngt = ledger.award(node_pk, 0.8, 0.6, 5);

// Check balance
let balance = ledger.balance(&node_pk);

// Total supply
let supply = ledger.total_supply();
```

---

## REST API (Future)

### Endpoints

#### `POST /grains`

Add a new grain.

**Request:**
```json
{
  "text": "Rust is a systems programming language",
  "tags": ["rust", "programming"],
  "lang": "en"
}
```

**Response:**
```json
{
  "id": "a1b2c3d4...",
  "ngt_awarded": 1.23
}
```

#### `GET /grains/:id`

Get grain by ID.

**Response:**
```json
{
  "id": "a1b2c3d4...",
  "title": "Rust is...",
  "author_pk": "e5f6g7h8...",
  "ts_unix_ms": 1234567890
}
```

#### `POST /query`

Query semantic memory.

**Request:**
```json
{
  "query": "What is Rust?",
  "k": 5
}
```

**Response:**
```json
{
  "results": [
    {
      "grain_id": "a1b2c3d4...",
      "similarity": 0.923,
      "title": "Rust is..."
    }
  ]
}
```

---

## Configuration

### `config.toml` (Future)

```toml
[node]
data_dir = ".synapsenet"
port = 9000

[poe]
alpha = 0.5
beta = 0.3
gamma = 0.2
tau_novelty = 0.1
tau_coherence = 0.1

[p2p]
bootstrap_peers = [
    "/ip4/1.2.3.4/tcp/9000/p2p/12D3KooW..."
]

[policy]
default = "Ok"

[[policy.rules]]
keywords = ["bomb", "weapon"]
class = "AnalysisOnly"
```

---

## Error Codes

- `0`: Success
- `1`: General error
- `2`: Invalid input
- `3`: Storage error
- `4`: Network error
- `5`: Crypto error
