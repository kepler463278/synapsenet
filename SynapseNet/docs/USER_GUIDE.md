# SynapseNet v0.4 User Guide

Welcome to SynapseNet - your personal decentralized semantic memory system!

## Table of Contents

1. [Getting Started](#getting-started)
2. [Core Features](#core-features)
3. [Configuration](#configuration)
4. [Troubleshooting](#troubleshooting)
5. [Advanced Usage](#advanced-usage)

---

## Getting Started

### Installation

#### macOS
```bash
# Download the .dmg file
open SynapseNet-0.4.0.dmg

# Drag to Applications folder
# Launch from Applications
```

#### Windows
```bash
# Download the .msi installer
# Double-click to install
# Launch from Start Menu
```

#### Linux
```bash
# Debian/Ubuntu
sudo dpkg -i synapsenet_0.4.0_amd64.deb

# Or use AppImage
chmod +x SynapseNet-0.4.0.AppImage
./SynapseNet-0.4.0.AppImage
```

### First Launch

When you first launch SynapseNet:

1. **Welcome Screen** - Brief introduction
2. **Data Directory** - Choose where to store your knowledge
3. **Model Download** - Download embedding model (22MB)
4. **Ready!** - Start adding knowledge

---

## Core Features

### 1. Adding Knowledge (Grains)

**What is a Grain?**  
A "grain" is a piece of knowledge - text, notes, ideas, or information you want to remember.

**How to Add:**

1. Click **➕ Add** in the navigation
2. Type or paste your text
3. Add tags (optional): `ai, rust, learning`
4. Click **Add Grain**

**Example:**
```
Text: "Rust's ownership system prevents data races at compile time"
Tags: rust, programming, memory-safety
```

**Tips:**
- Keep grains focused (1 idea per grain)
- Use descriptive tags
- Add context for better search results

### 2. Semantic Search

**What is Semantic Search?**  
Unlike keyword search, semantic search understands *meaning*. Search for "machine learning" and find results about "neural networks" and "AI".

**How to Search:**

1. Click **🔍 Search** in the navigation
2. Type your question or topic
3. View results ranked by relevance

**Example Searches:**
- "How does Rust prevent memory leaks?"
- "Explain quantum computing"
- "Best practices for API design"

**Search Tips:**
- Use natural language
- Be specific for better results
- Try different phrasings

### 3. Knowledge Graph

**Visualize Connections:**

1. Click **🕸️ Graph** in the navigation
2. See your knowledge as an interconnected network
3. Click nodes to view details
4. Zoom and pan to explore

**What You'll See:**
- **Nodes** - Your grains
- **Edges** - Semantic connections
- **Clusters** - Related topics

### 4. Statistics & Monitoring

**Track Your Knowledge:**

1. Click **📊 Stats** in the navigation
2. View total grains, storage usage
3. See network peers (if P2P enabled)
4. Monitor performance metrics

**Key Metrics:**
- Total Grains
- Storage Size
- Embedding Model
- Network Status

### 5. Settings & Configuration

**Customize SynapseNet:**

1. Click **⚙️ Settings** in the navigation
2. Configure 5 sections:
   - **Node** - Name, data directory
   - **Network** - P2P, DHT, clustering
   - **AI** - Models, GPU settings
   - **Economy** - PoE rewards
   - **UI** - Theme, preferences

**Quick Settings:**
- **Theme:** Light / Dark / Auto
- **Default View:** Choose startup screen
- **Results Per Page:** 10, 25, 50

---

## Configuration

### Configuration File

Location: `~/.synapsenet/config.toml`

### Basic Configuration

```toml
[node]
name = "my-synapsenet"
data_dir = ".synapsenet"

[ai]
model_name = "all-MiniLM-L6-v2"
embedding_dim = 384
provider = "cpu"  # or "coreml", "cuda"

[ui]
theme = "dark"
default_view = "search"
```

### Advanced Configuration

#### Enable P2P Networking

```toml
[p2p]
enabled = true
port = 9000
mdns_enabled = true

[network]
dht_enabled = true
relay_enabled = true
max_peers = 50
```

#### Multi-Model Setup

```toml
[ai]
multi_model_enabled = true

[[ai.additional_models]]
name = "all-mpnet-base-v2"
path = "models/all-mpnet-base-v2.onnx"
size = "medium"
auto_load = false
```

#### PoE Rewards

```toml
[economy]
poe_enabled = true
novelty_weight = 0.4
coherence_weight = 0.3
reuse_weight = 0.3
min_novelty_threshold = 0.1
```

### GPU Acceleration

**macOS (CoreML):**
```toml
[ai]
provider = "coreml"
```

**Windows (DirectML):**
```toml
[ai]
provider = "directml"
```

**Linux (CUDA):**
```toml
[ai]
provider = "cuda"
```

---

## Troubleshooting

### Common Issues

#### 1. Model Download Fails

**Problem:** "Failed to download embedding model"

**Solutions:**
- Check internet connection
- Try manual download from [releases page]
- Place model in `~/.synapsenet/models/`

#### 2. Slow Embedding Generation

**Problem:** Adding grains takes too long

**Solutions:**
- Enable GPU acceleration (if available)
- Use smaller model (all-MiniLM-L6-v2)
- Close other applications
- Check CPU usage

#### 3. Search Returns No Results

**Problem:** Search doesn't find anything

**Solutions:**
- Add more grains (need at least 5-10)
- Try different search terms
- Check if grains were added successfully
- Rebuild index: Settings → Advanced → Rebuild Index

#### 4. P2P Connection Issues

**Problem:** Can't connect to peers

**Solutions:**
- Check firewall settings
- Enable UPnP on router
- Try different port (default: 9000)
- Check network connectivity

#### 5. High Memory Usage

**Problem:** Application uses too much RAM

**Solutions:**
- Disable multi-model support
- Reduce max_peers setting
- Close unused features
- Restart application

### Error Messages

#### "Database migration failed"

**Cause:** Incompatible database version

**Fix:**
```bash
# Backup your data first!
cp -r ~/.synapsenet ~/.synapsenet.backup

# Run migration
syn migrate --db-path ~/.synapsenet/synapsenet.db
```

#### "Model not found"

**Cause:** Embedding model not loaded

**Fix:**
1. Go to Settings → AI
2. Check model name
3. Download model if missing
4. Restart application

#### "Out of memory"

**Cause:** Batch too large or insufficient RAM

**Fix:**
- Reduce batch size in settings
- Close other applications
- Use smaller embedding model
- Add more RAM (if possible)

### Getting Help

**Resources:**
- 📖 Documentation: https://docs.synapsenet.io
- 💬 Discord: https://discord.gg/synapsenet
- 🐛 GitHub Issues: https://github.com/yourusername/synapsenet/issues
- 📧 Email: support@synapsenet.io

**Before Reporting Issues:**
1. Check this troubleshooting section
2. Search existing GitHub issues
3. Collect error logs (Settings → Advanced → Export Logs)
4. Note your OS and SynapseNet version

---

## Advanced Usage

### Batch Import

**Import Multiple Grains:**

1. Prepare JSON file:
```json
{
  "items": [
    {"text": "First grain", "tags": ["tag1"]},
    {"text": "Second grain", "tags": ["tag2"]}
  ]
}
```

2. Use REST API:
```bash
curl -X POST http://localhost:9900/v2/batch/import \
  -H "Content-Type: application/json" \
  -d @grains.json
```

### CLI Usage

**Command Line Interface:**

```bash
# Initialize node
syn init

# Add grain
syn add "Your knowledge here"

# Search
syn query "search term" --k 5

# Export to Parquet
syn export --output ./backup

# Import from Parquet
syn import --input ./backup

# Show statistics
syn stats

# Migrate database
syn migrate
```

### REST API

**Available Endpoints:**

```bash
# v1 API (deprecated)
POST /add
POST /query
GET  /stats
GET  /peers

# v2 API (recommended)
GET  /v2/models
POST /v2/batch/import
GET  /v2/poe/scores
GET  /v2/network/peers
```

**Example: Batch Import**

```python
import requests

response = requests.post('http://localhost:9900/v2/batch/import', json={
    'items': [
        {'text': 'Knowledge 1', 'tags': ['ai']},
        {'text': 'Knowledge 2', 'tags': ['rust']}
    ]
})

result = response.json()
print(f"Imported {result['succeeded']}/{result['total']} grains")
```

### Keyboard Shortcuts

**Navigation:**
- `Cmd/Ctrl + 1` - Add Grain
- `Cmd/Ctrl + 2` - Search
- `Cmd/Ctrl + 3` - Graph
- `Cmd/Ctrl + 4` - Stats
- `Cmd/Ctrl + ,` - Settings

**Actions:**
- `Cmd/Ctrl + Enter` - Submit form
- `Cmd/Ctrl + K` - Focus search
- `Esc` - Close modal/dialog

### Performance Tips

**Optimize for Speed:**

1. **Use GPU** - 3-5x faster embeddings
2. **Batch Operations** - Import multiple grains at once
3. **Smaller Models** - Faster but less accurate
4. **Index Tuning** - Adjust HNSW parameters
5. **Disable Features** - Turn off unused features

**HNSW Index Tuning:**

```toml
[storage]
hnsw_m = 16              # Connections per layer (default)
hnsw_ef_construction = 200  # Build quality (higher = better)
```

### Data Management

**Backup Your Data:**

```bash
# Full backup
cp -r ~/.synapsenet ~/synapsenet-backup

# Export to Parquet (portable)
syn export --output ~/synapsenet-export
```

**Restore from Backup:**

```bash
# From directory backup
cp -r ~/synapsenet-backup ~/.synapsenet

# From Parquet export
syn import --input ~/synapsenet-export
```

**Clean Up:**

```bash
# Remove old access logs (90+ days)
# Automatic in Settings → Economy → Access Retention

# Rebuild index (if corrupted)
# Settings → Advanced → Rebuild Index
```

---

## Best Practices

### Knowledge Organization

1. **Atomic Grains** - One idea per grain
2. **Descriptive Tags** - Use consistent tagging
3. **Context Matters** - Add enough detail
4. **Regular Review** - Search and refine

### Search Strategies

1. **Natural Language** - Ask questions
2. **Multiple Queries** - Try different phrasings
3. **Use Tags** - Filter by topic
4. **Explore Graph** - Find connections

### Performance

1. **Batch Import** - Add multiple grains at once
2. **GPU Acceleration** - Enable if available
3. **Regular Cleanup** - Remove unused data
4. **Monitor Metrics** - Check performance dashboard

### Security

1. **Backup Regularly** - Protect your knowledge
2. **Update Software** - Stay current
3. **Review Logs** - Check for issues
4. **Secure Network** - Use firewall

---

## Glossary

**Grain** - A piece of knowledge stored in SynapseNet

**Embedding** - Vector representation of text meaning

**Semantic Search** - Search by meaning, not keywords

**HNSW** - Fast approximate nearest neighbor search

**PoE** - Proof of Emergence, reward system

**DHT** - Distributed Hash Table for peer discovery

**NGT** - Network Grain Token, reward currency

---

## What's New in v0.4

### Major Features

✨ **Multi-Model Support** - Load multiple embedding models  
⚡ **Batch Processing** - 4x faster bulk operations  
💰 **PoE v2** - Enhanced reward system  
🌐 **REST API v2** - New endpoints  
⚙️ **Enhanced Config** - More customization  
🛡️ **Error Recovery** - Automatic retry & fallback  
📊 **Monitoring** - Real-time metrics dashboard

### Breaking Changes

- Configuration file format updated (auto-migrates)
- Database schema v4 (auto-migrates)
- API v1 deprecated (still works with warnings)

### Migration from v0.3

Your data automatically migrates on first launch. No action needed!

---

**Version:** 0.4.0  
**Last Updated:** 2024-10-31  
**License:** MIT
