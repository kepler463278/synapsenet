# SynapseNet REST API Examples

Complete guide to using the SynapseNet REST API with curl.

## Starting the Server

```bash
# Start with default address (127.0.0.1:9900)
syn serve

# Or specify custom address
syn serve --addr 0.0.0.0:8080
```

## Basic Endpoints

### 1. Root - API Information

```bash
curl http://localhost:9900/
```

**Response:**
```json
{
  "name": "SynapseNet API",
  "version": "0.3.0",
  "endpoints": [
    "GET  /",
    "GET  /health",
    "POST /init",
    "POST /add",
    "POST /query",
    "GET  /stats",
    "GET  /peers"
  ]
}
```

### 2. Health Check

```bash
curl http://localhost:9900/health
```

**Response:**
```json
{
  "status": "ok",
  "timestamp": 1699564800
}
```

## Grain Operations

### 3. Add Grain (Simple)

```bash
curl -X POST http://localhost:9900/add \
  -H "Content-Type: application/json" \
  -d '{
    "text": "Rust is a systems programming language focused on safety and performance"
  }'
```

**Response:**
```json
{
  "grain_id": "a1b2c3d4e5f6...",
  "embedding_time_ms": 45
}
```

### 4. Add Grain with Tags

```bash
curl -X POST http://localhost:9900/add \
  -H "Content-Type: application/json" \
  -d '{
    "text": "Machine learning is a subset of artificial intelligence",
    "tags": ["AI", "ML", "technology"]
  }'
```

**Response:**
```json
{
  "grain_id": "f6e5d4c3b2a1...",
  "embedding_time_ms": 42
}
```

### 5. Add Multiple Grains (Batch)

```bash
# Add first grain
curl -X POST http://localhost:9900/add \
  -H "Content-Type: application/json" \
  -d '{"text": "Python is a high-level programming language"}'

# Add second grain
curl -X POST http://localhost:9900/add \
  -H "Content-Type: application/json" \
  -d '{"text": "JavaScript is used for web development"}'

# Add third grain
curl -X POST http://localhost:9900/add \
  -H "Content-Type: application/json" \
  -d '{"text": "Go is designed for concurrent programming"}'
```

## Query Operations

### 6. Simple Query

```bash
curl -X POST http://localhost:9900/query \
  -H "Content-Type: application/json" \
  -d '{
    "text": "What is Rust?"
  }'
```

**Response:**
```json
{
  "results": [
    {
      "grain_id": "a1b2c3d4e5f6...",
      "similarity": 0.92,
      "title": "Rust is a systems programming language..."
    }
  ],
  "query_time_ms": 28
}
```

### 7. Query with Custom K

```bash
curl -X POST http://localhost:9900/query \
  -H "Content-Type: application/json" \
  -d '{
    "text": "programming languages",
    "k": 10
  }'
```

**Response:**
```json
{
  "results": [
    {
      "grain_id": "...",
      "similarity": 0.95,
      "title": "Rust is a systems..."
    },
    {
      "grain_id": "...",
      "similarity": 0.89,
      "title": "Python is a high-level..."
    },
    {
      "grain_id": "...",
      "similarity": 0.87,
      "title": "JavaScript is used..."
    }
  ],
  "query_time_ms": 35
}
```

## Monitoring

### 8. Node Statistics

```bash
curl http://localhost:9900/stats
```

**Response:**
```json
{
  "grains_total": 42,
  "peers_connected": 3,
  "uptime_seconds": 3600
}
```

### 9. Peer Information

```bash
curl http://localhost:9900/peers
```

**Response:**
```json
{
  "peers": [],
  "count": 0
}
```

### 10. Prometheus Metrics

```bash
curl http://localhost:9900/metrics
```

**Response:**
```
# HELP syn_embedding_seconds Time spent generating embeddings
# TYPE syn_embedding_seconds histogram
syn_embedding_seconds_bucket{le="0.005"} 0
syn_embedding_seconds_bucket{le="0.01"} 5
syn_embedding_seconds_bucket{le="0.025"} 15
syn_embedding_seconds_bucket{le="0.05"} 42
syn_embedding_seconds_bucket{le="0.1"} 42
syn_embedding_seconds_bucket{le="+Inf"} 42
syn_embedding_seconds_sum 1.89
syn_embedding_seconds_count 42

# HELP syn_grains_total Total number of grains in storage
# TYPE syn_grains_total gauge
syn_grains_total 42

# HELP syn_query_total Total number of queries processed
# TYPE syn_query_total counter
syn_query_total 15
```

## Advanced Examples

### 11. Semantic Search

```bash
# Add knowledge base
curl -X POST http://localhost:9900/add \
  -H "Content-Type: application/json" \
  -d '{"text": "The Eiffel Tower is located in Paris, France"}'

curl -X POST http://localhost:9900/add \
  -H "Content-Type: application/json" \
  -d '{"text": "The Statue of Liberty is in New York City"}'

curl -X POST http://localhost:9900/add \
  -H "Content-Type: application/json" \
  -d '{"text": "Big Ben is a famous clock tower in London"}'

# Query
curl -X POST http://localhost:9900/query \
  -H "Content-Type: application/json" \
  -d '{"text": "famous landmarks in Europe", "k": 5}'
```

### 12. Pipeline with jq

```bash
# Add and extract grain ID
GRAIN_ID=$(curl -s -X POST http://localhost:9900/add \
  -H "Content-Type: application/json" \
  -d '{"text": "Docker is a containerization platform"}' \
  | jq -r '.grain_id')

echo "Added grain: $GRAIN_ID"

# Query and format results
curl -s -X POST http://localhost:9900/query \
  -H "Content-Type: application/json" \
  -d '{"text": "containers", "k": 3}' \
  | jq '.results[] | {similarity, title}'
```

### 13. Monitoring Script

```bash
#!/bin/bash
# monitor.sh - Monitor SynapseNet metrics

while true; do
  echo "=== SynapseNet Stats ==="
  curl -s http://localhost:9900/stats | jq '.'
  
  echo -e "\n=== Recent Metrics ==="
  curl -s http://localhost:9900/metrics | grep -E "syn_(grains|query|embedding)_total"
  
  echo -e "\n"
  sleep 5
done
```

### 14. Load Testing

```bash
#!/bin/bash
# load_test.sh - Simple load test

for i in {1..100}; do
  curl -s -X POST http://localhost:9900/add \
    -H "Content-Type: application/json" \
    -d "{\"text\": \"Test grain number $i\"}" &
done

wait
echo "Load test complete!"
```

## Error Handling

### 15. Invalid Request

```bash
curl -X POST http://localhost:9900/add \
  -H "Content-Type: application/json" \
  -d '{"invalid": "field"}'
```

**Response:**
```json
{
  "error": "missing field `text`"
}
```

### 16. Server Not Running

```bash
curl http://localhost:9900/health
```

**Response:**
```
curl: (7) Failed to connect to localhost port 9900: Connection refused
```

## Integration Examples

### Python

```python
import requests

# Add grain
response = requests.post('http://localhost:9900/add', json={
    'text': 'Python is great for data science',
    'tags': ['python', 'data-science']
})
print(f"Grain ID: {response.json()['grain_id']}")

# Query
response = requests.post('http://localhost:9900/query', json={
    'text': 'data science tools',
    'k': 5
})
for result in response.json()['results']:
    print(f"Similarity: {result['similarity']:.2f} - {result['title']}")
```

### JavaScript/Node.js

```javascript
const axios = require('axios');

// Add grain
async function addGrain(text, tags = []) {
  const response = await axios.post('http://localhost:9900/add', {
    text,
    tags
  });
  return response.data.grain_id;
}

// Query
async function query(text, k = 5) {
  const response = await axios.post('http://localhost:9900/query', {
    text,
    k
  });
  return response.data.results;
}

// Usage
(async () => {
  const grainId = await addGrain('Node.js is a JavaScript runtime');
  console.log(`Added grain: ${grainId}`);
  
  const results = await query('JavaScript runtime');
  results.forEach(r => {
    console.log(`${r.similarity.toFixed(2)} - ${r.title}`);
  });
})();
```

### Go

```go
package main

import (
    "bytes"
    "encoding/json"
    "fmt"
    "net/http"
)

type AddRequest struct {
    Text string   `json:"text"`
    Tags []string `json:"tags,omitempty"`
}

type QueryRequest struct {
    Text string `json:"text"`
    K    int    `json:"k,omitempty"`
}

func addGrain(text string) (string, error) {
    req := AddRequest{Text: text}
    body, _ := json.Marshal(req)
    
    resp, err := http.Post("http://localhost:9900/add",
        "application/json", bytes.NewBuffer(body))
    if err != nil {
        return "", err
    }
    defer resp.Body.Close()
    
    var result map[string]interface{}
    json.NewDecoder(resp.Body).Decode(&result)
    return result["grain_id"].(string), nil
}

func main() {
    grainID, _ := addGrain("Go is efficient and concurrent")
    fmt.Printf("Added grain: %s\n", grainID)
}
```

## Tips & Best Practices

### Performance

1. **Batch operations** - Add multiple grains before querying
2. **Use appropriate k** - Don't request more results than needed
3. **Monitor metrics** - Watch `/metrics` for performance insights
4. **Enable GPU** - Use `--features coreml/directml/cuda` for 2-4x speedup

### Production

1. **Use reverse proxy** - nginx or Caddy with TLS
2. **Add authentication** - JWT or API keys
3. **Rate limiting** - Prevent abuse
4. **Monitoring** - Prometheus + Grafana
5. **Backup** - Regular database backups

### Development

1. **Use jq** - Format JSON responses
2. **Save responses** - `curl ... > response.json`
3. **Test scripts** - Automate common workflows
4. **Check logs** - `RUST_LOG=info syn serve`

## Troubleshooting

### Server won't start

```bash
# Check if port is in use
lsof -i :9900

# Try different port
syn serve --addr 127.0.0.1:8080
```

### Slow responses

```bash
# Check if GPU is enabled
curl http://localhost:9900/stats

# Enable GPU acceleration
cargo build --release --features coreml  # or directml, cuda
```

### Connection refused

```bash
# Ensure server is running
ps aux | grep syn

# Check firewall
sudo ufw status
```

## Resources

- **API Documentation:** [docs/API.md](docs/API.md)
- **GPU Guide:** [docs/GPU.md](docs/GPU.md)
- **Release Notes:** [RELEASE_NOTES_v0.3.md](RELEASE_NOTES_v0.3.md)

---

*Happy querying! 🚀*
