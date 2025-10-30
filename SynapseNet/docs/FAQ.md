# Frequently Asked Questions

## General

### What is SynapseNet?

SynapseNet is a decentralized semantic memory network where nodes exchange knowledge grains and earn rewards through Proof of Emergence (PoE). It's designed to be local-first, privacy-preserving, and community-owned.

### Who owns SynapseNet?

No one. SynapseNet is open-source (MIT/Apache-2.0) and decentralized. There's no company, no premine, no privileged keys. See [GENESIS.txt](../GENESIS.txt).

### How is it different from other knowledge bases?

- **Decentralized**: No central server
- **Cryptographically signed**: All grains are verifiable
- **Proof of Emergence**: Rewards for valuable contributions
- **Safe by design**: Policy engine for harmful queries
- **Local-first**: Your data stays with you

## Technical

### What's a "grain"?

A grain is the atomic unit of knowledge in SynapseNet. It contains:
- Vector embedding (semantic representation)
- Metadata (author, timestamp, tags)
- Cryptographic signature
- Links to related grains

### How does vector search work?

We use HNSW (Hierarchical Navigable Small World) algorithm for approximate nearest neighbor search. It's fast and doesn't require external dependencies.

### Why Rust?

- Memory safety without garbage collection
- Cross-platform (macOS/Linux/Windows)
- Excellent performance
- Strong type system
- Great ecosystem

### Can I use my own embedding model?

Yes! The embedding interface is pluggable. v0.1 uses dummy embeddings, v0.2 will add ONNX support, and you can implement your own.

### What about GPU acceleration?

Planned for v0.3:
- Metal (macOS)
- CUDA (Linux)
- DirectML (Windows)

## Usage

### How do I get started?

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build --release

# Initialize
./target/release/syn init

# Add knowledge
./target/release/syn add "Your knowledge here"

# Query
./target/release/syn query "Your question"
```

See [QUICKSTART.md](QUICKSTART.md) for details.

### Where is my data stored?

By default in `.synapsenet/` directory:
- `node.key` - Your private key (keep safe!)
- `node.pub` - Your public key
- `synapsenet.db` - SQLite database

### Can I backup my data?

Yes! Just backup the `.synapsenet/` directory. Your private key is especially important.

### How do I delete a grain?

Currently, grains are immutable once created. Deletion will be added in a future version with proper tombstone handling.

## Security

### Is my data private?

Yes, in v0.1 (local-only mode). When P2P is enabled (v0.2+), only embeddings and metadata are shared, not the original content (unless you choose to).

### What if someone steals my private key?

They can impersonate you and create grains with your signature. Keep your `node.key` safe and backed up.

### How are harmful queries handled?

The policy engine classifies queries:
- **OK**: Normal response
- **AnalysisOnly**: Consequences only, no instructions
- **Curated**: Human review required

See [policy.rs](../crates/governance/src/policy.rs).

### Can I trust other nodes?

The reputation system (basic in v0.1) tracks peer behavior. Always verify signatures. Byzantine fault tolerance is planned for v1.0.

## Economics

### What is NGT?

Neural Graph Token - a measure of contribution to the network's emergence. Earned through Proof of Emergence.

### How do I earn NGT?

By contributing valuable grains:
- High novelty (new information)
- High coherence (fits well with existing knowledge)
- High reuse (others find it useful)

### Is NGT a cryptocurrency?

Not yet. v0.1 tracks NGT as points. Future versions may add token functionality.

### What's the total supply?

Unlimited. NGT is minted through contributions (Proof of Emergence). No premine.

## P2P Network

### When will P2P be ready?

v0.2 (Q2 2025). v0.1 is local-only.

### How do nodes discover each other?

- mDNS (local network)
- Bootstrap nodes (public network)
- Manual peer addition

### What if a node goes offline?

Other nodes continue operating. Data is replicated across multiple nodes (configurable).

### Can I run a private network?

Yes! Use DevNet scripts or configure your own bootstrap nodes.

## Development

### How can I contribute?

See [CONTRIBUTING.md](../CONTRIBUTING.md).

### Where's the roadmap?

See [ROADMAP.md](ROADMAP.md).

### Can I build commercial products on SynapseNet?

Yes! MIT/Apache-2.0 license allows commercial use.

### How do I report bugs?

Open an issue on GitHub or email security@synapsenet.org for security issues.

## Troubleshooting

### "command not found: cargo"

Install Rust: https://rustup.rs

### "error: linker `cc` not found"

Install build tools:
- macOS: `xcode-select --install`
- Linux: `sudo apt-get install build-essential`
- Windows: Install Visual Studio Build Tools

### "No grains in local memory"

Add some grains first: `syn add "text"`

### Query results are not relevant

v0.1 uses dummy embeddings. Real embeddings coming in v0.2.

### Build is slow

Use `cargo build` (debug) for development. Only use `--release` when you need performance.

## Community

### Where can I get help?

- GitHub Issues
- GitHub Discussions
- Community chat (coming soon)

### How do I stay updated?

- Watch the GitHub repo
- Follow releases
- Join the mailing list (coming soon)

### Can I translate documentation?

Yes! PRs welcome. We already have README.ru.md.

---

## Still have questions?

Open an issue on GitHub or start a discussion!
