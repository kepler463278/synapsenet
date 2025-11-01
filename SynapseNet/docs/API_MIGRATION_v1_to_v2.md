# API Migration Guide: v1 to v2

This guide helps you migrate from SynapseNet REST API v1 (v0.3) to v2 (v0.4).

## Overview

API v2 introduces new endpoints for v0.4 features while maintaining backward compatibility with v1 endpoints. All v1 endpoints continue to work but are marked as deprecated.

## Key Changes

### 1. Endpoint Versioning

All new endpoints are prefixed with `/v2`:

```
v1: POST /add
v2: POST /v2/batch/import
```

### 2. Batch Operations

**v1 (Deprecated):**
```bash
# Add single grain
POST /add
{
  "text": "My knowledge",
  "tags": ["tag1"]
}
```

**v2 (Recommended):**
```bash
# Add multiple grains in batch
POST /v2/batch/import
{
  "items": [
    {"text": "Knowledge 1", "tags": ["tag1"]},
    {"text": "Knowledge 2", "title": "Title 2"}
  ],
  "model": "all-MiniLM-L6-v2"
}
```

**Benefits:**
- Faster processing (batch embedding)
- Better error handling (partial success)
- Model selection per batch

### 3. Model Management

**New in v2:**

```bash
# List all available models
GET /v2/models

# Get specific model info
GET /v2/models/all-MiniLM-L6-v2
```

**Response:**
```json
{
  "name": "all-MiniLM-L6-v2",
  "dimensions": 384,
  "file_size_mb": 22.0,
  "loaded_at": 1698765432,
  "status": "loaded"
}
```

### 4. PoE (Proof of Emergence) Scores

**New in v2:**

```bash
# Get PoE scores for all grains
GET /v2/poe/scores?limit=100&min_score=0.5

# Get PoE score for specific grain
GET /v2/poe/scores/{grain_id}
```

**Response:**
```json
{
  "grain_id": "abc123...",
  "novelty": 0.8,
  "coherence": 0.6,
  "reuse": 0.4,
  "total": 0.64,
  "ngt_reward": 7.4,
  "calculated_at": 1698765432
}
```

### 5. Network & Clustering

**New in v2:**

```bash
# Get network peers with cluster info
GET /v2/network/peers

# Get peer clusters by topic
GET /v2/network/clusters
```

**Response:**
```json
{
  "peer_id": "12D3KooW...",
  "topics": ["ai", "rust"],
  "similarity": 0.85,
  "last_seen": 1698765432,
  "connection_status": "connected"
}
```

## Migration Checklist

### Immediate Actions (Optional)

- [ ] Update client code to use `/v2` endpoints
- [ ] Test batch import with your data
- [ ] Explore new PoE score endpoints
- [ ] Check network clustering information

### Before v0.5 (Required)

- [ ] Migrate all `/add` calls to `/v2/batch/import`
- [ ] Update error handling for batch responses
- [ ] Remove deprecated endpoint usage

## Backward Compatibility

### v1 Endpoints (Still Working)

All v1 endpoints continue to work in v0.4:

- `POST /init` - Initialize node
- `POST /add` - Add single grain (⚠️ deprecated)
- `POST /query` - Search grains
- `GET /stats` - Node statistics
- `GET /peers` - Network peers (basic)
- `GET /health` - Health check

### Deprecation Warnings

v1 endpoints now return deprecation warnings in logs:

```
WARN POST /add is deprecated, use POST /v2/batch/import instead
```

### Timeline

- **v0.4**: v1 endpoints work with deprecation warnings
- **v0.5**: v1 endpoints may be removed (TBD)

## Code Examples

### Python Client Migration

**Before (v1):**
```python
import requests

# Add single grain
response = requests.post('http://localhost:9900/add', json={
    'text': 'My knowledge',
    'tags': ['ai']
})
grain_id = response.json()['grain_id']
```

**After (v2):**
```python
import requests

# Add multiple grains
response = requests.post('http://localhost:9900/v2/batch/import', json={
    'items': [
        {'text': 'Knowledge 1', 'tags': ['ai']},
        {'text': 'Knowledge 2', 'tags': ['rust']}
    ]
})

result = response.json()
print(f"Imported {result['succeeded']}/{result['total']} grains")
grain_ids = result['grain_ids']
```

### JavaScript/TypeScript Migration

**Before (v1):**
```typescript
const response = await fetch('http://localhost:9900/add', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    text: 'My knowledge',
    tags: ['ai']
  })
});
const { grain_id } = await response.json();
```

**After (v2):**
```typescript
const response = await fetch('http://localhost:9900/v2/batch/import', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    items: [
      { text: 'Knowledge 1', tags: ['ai'] },
      { text: 'Knowledge 2', tags: ['rust'] }
    ]
  })
});

const result = await response.json();
console.log(`Imported ${result.succeeded}/${result.total} grains`);
```

## Error Handling

### v2 Batch Import Errors

v2 batch import provides detailed error information:

```json
{
  "total": 10,
  "succeeded": 8,
  "failed": 2,
  "grain_ids": ["abc...", "def...", ...],
  "errors": [
    "Item 3: Invalid text format",
    "Item 7: Embedding failed"
  ],
  "processing_time_ms": 1234
}
```

**Best Practice:**
```python
result = response.json()
if result['failed'] > 0:
    print(f"Warning: {result['failed']} items failed:")
    for error in result['errors']:
        print(f"  - {error}")
```

## Performance Improvements

### Batch Processing

v2 batch import is significantly faster:

- **v1**: 10 grains = 10 requests = ~2000ms
- **v2**: 10 grains = 1 request = ~500ms

**Speedup: 4x faster**

### Recommended Batch Sizes

- Small batches: 10-50 items
- Medium batches: 50-200 items
- Large batches: 200-1000 items

## Support

For questions or issues:

- GitHub Issues: https://github.com/yourusername/synapsenet/issues
- Documentation: https://docs.synapsenet.io
- Discord: https://discord.gg/synapsenet

## Changelog

### v0.4.0 (2024-10)

- Added `/v2/models` endpoints
- Added `/v2/batch/import` for batch operations
- Added `/v2/poe/scores` for PoE score queries
- Added `/v2/network/peers` with cluster information
- Deprecated `/add` endpoint (use `/v2/batch/import`)
- All v1 endpoints remain functional with warnings
