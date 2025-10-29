# Docker Deployment Guide

## Quick Start

### Build Image

```bash
docker build -t synapsenet:latest .
```

### Run Single Node

```bash
docker run -it --rm \
  -v synapsenet-data:/data \
  -p 9000:9000 \
  synapsenet:latest init

docker run -it --rm \
  -v synapsenet-data:/data \
  synapsenet:latest add "Hello from Docker"

docker run -it --rm \
  -v synapsenet-data:/data \
  synapsenet:latest query "Hello"
```

## Docker Compose (Multi-Node)

### Start Cluster

```bash
docker-compose up -d
```

This starts 3 nodes:
- node1: localhost:9001
- node2: localhost:9002
- node3: localhost:9003

### Initialize Nodes

```bash
docker exec synapsenet-node1 syn init
docker exec synapsenet-node2 syn init
docker exec synapsenet-node3 syn init
```

### Add Grains

```bash
docker exec synapsenet-node1 syn add "Knowledge from node 1"
docker exec synapsenet-node2 syn add "Knowledge from node 2"
```

### Query

```bash
docker exec synapsenet-node1 syn query "Knowledge"
```

### Stop Cluster

```bash
docker-compose down
```

### Clean Up

```bash
docker-compose down -v  # Remove volumes too
```

## Custom Configuration

### Environment Variables

```bash
docker run -it --rm \
  -e RUST_LOG=info \
  -v synapsenet-data:/data \
  synapsenet:latest query "test"
```

### Custom Data Directory

```bash
docker run -it --rm \
  -v /path/to/data:/custom \
  synapsenet:latest --data-dir /custom init
```

## Production Deployment

### Docker Swarm

```yaml
version: '3.8'

services:
  synapsenet:
    image: synapsenet:latest
    deploy:
      replicas: 3
      restart_policy:
        condition: on-failure
    volumes:
      - synapsenet-data:/data
    networks:
      - synapsenet-net

volumes:
  synapsenet-data:

networks:
  synapsenet-net:
    driver: overlay
```

### Kubernetes

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: synapsenet
spec:
  serviceName: synapsenet
  replicas: 3
  selector:
    matchLabels:
      app: synapsenet
  template:
    metadata:
      labels:
        app: synapsenet
    spec:
      containers:
      - name: synapsenet
        image: synapsenet:latest
        ports:
        - containerPort: 9000
        volumeMounts:
        - name: data
          mountPath: /data
  volumeClaimTemplates:
  - metadata:
      name: data
    spec:
      accessModes: [ "ReadWriteOnce" ]
      resources:
        requests:
          storage: 10Gi
```

## Monitoring

### Health Check

```bash
docker exec synapsenet-node1 syn peers
```

### Logs

```bash
docker logs synapsenet-node1
docker logs -f synapsenet-node1  # Follow
```

### Resource Usage

```bash
docker stats synapsenet-node1
```

## Troubleshooting

### Container won't start

Check logs:
```bash
docker logs synapsenet-node1
```

### Permission issues

Make sure volumes have correct permissions:
```bash
docker run --rm -v synapsenet-data:/data alpine chown -R 1000:1000 /data
```

### Network issues

Check network:
```bash
docker network inspect synapsenet_synapsenet
```

## Best Practices

1. **Use volumes**: Don't store data in containers
2. **Backup keys**: Export `node.key` from volumes
3. **Monitor resources**: Set memory/CPU limits
4. **Use health checks**: Implement proper health endpoints
5. **Secure secrets**: Use Docker secrets for production

## Next Steps

- Set up monitoring (Prometheus/Grafana)
- Configure backup strategy
- Implement CI/CD pipeline
- Scale horizontally

---

For more information, see [ARCHITECTURE.md](ARCHITECTURE.md).
