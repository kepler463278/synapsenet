# SynapseNet Testnet Guide

## Overview

SynapseNet testnet is a Cosmos-based blockchain for testing on-chain PoE economics, staking, and model registry.

**Chain ID:** `synapsenet-testnet-1`  
**Denom:** `ungt` (1 NGT = 1,000,000 ungt)  
**Block Time:** ~6 seconds  
**Consensus:** CometBFT

---

## Quick Start

### Prerequisites

- Docker & Docker Compose
- Rust (for building contracts)
- wasmd CLI (optional)

### 1. Start Testnet

```bash
# Clone repository
git clone https://github.com/synapsenet/synapsenet
cd synapsenet

# Start testnet
docker-compose -f docker/compose.yaml up -d

# Check status
docker-compose ps
```

### 2. Get Test Tokens

```bash
# Request from faucet
curl -X POST http://localhost:8000/faucet \
  -H "Content-Type: application/json" \
  -d '{"address":"your_address"}'

# You'll receive 1000 NGT
```

### 3. Deploy Contracts

```bash
# Make scripts executable
chmod +x scripts/*.sh

# Deploy contracts
./scripts/deploy-contracts.sh

# Contract addresses saved to contracts.json
```

---

## Services

### Chain Node
- **RPC:** http://localhost:26657
- **REST:** http://localhost:1317
- **gRPC:** localhost:9090

### Faucet
- **URL:** http://localhost:8000
- **Amount:** 1000 NGT per request
- **Rate Limit:** 1 request per hour per address

### Explorer
- **URL:** http://localhost:3000
- View transactions, blocks, and contracts

---

## Using the Testnet

### Submit PoE Batch

```bash
# Create batch
syn poe batch --epoch 1

# Submit to chain
syn poe submit --batch batch_1.json

# Check status
syn poe balance --node your_node_id
```

### Claim Rewards

```bash
# Claim accrued rewards
syn poe claim --node your_node_id

# Check balance
wasmd query bank balances your_address
```

### Query Contract

```bash
# Get config
wasmd query wasm contract-state smart CONTRACT_ADDR \
  '{"get_config":{}}'

# Get epoch
wasmd query wasm contract-state smart CONTRACT_ADDR \
  '{"get_epoch":{"epoch":1}}'

# Get accrual
wasmd query wasm contract-state smart CONTRACT_ADDR \
  '{"get_accrual":{"node":"node_id"}}'
```

---

## Validator Operations

### Add Validator

```bash
# Create validator key
wasmd keys add my_validator

# Get tokens from faucet
# ...

# Create validator
wasmd tx staking create-validator \
  --amount=100000000ungt \
  --pubkey=$(wasmd tendermint show-validator) \
  --moniker="My Validator" \
  --chain-id=synapsenet-testnet-1 \
  --commission-rate="0.10" \
  --commission-max-rate="0.20" \
  --commission-max-change-rate="0.01" \
  --min-self-delegation="1" \
  --from=my_validator
```

---

## Troubleshooting

### Chain Not Starting

```bash
# Check logs
docker-compose logs chain

# Reset chain
docker-compose down -v
./scripts/init-testnet.sh
docker-compose up -d
```

### Contract Deployment Failed

```bash
# Check gas
# Increase --gas-adjustment to 1.5

# Check balance
wasmd query bank balances your_address
```

### Faucet Not Working

```bash
# Check faucet logs
docker-compose logs faucet

# Restart faucet
docker-compose restart faucet
```

---

## Network Parameters

```toml
[poe]
epoch_secs = 600        # 10 minutes
min_signers = 3
reward_per_epoch = 1000 NGT

[staking]
unbonding_time = 604800  # 7 days
max_validators = 100

[gov]
min_deposit = 100 NGT
voting_period = 86400    # 1 day
```

---

## Support

- **Discord:** https://discord.gg/synapsenet
- **Docs:** https://docs.synapsenet.ai
- **GitHub:** https://github.com/synapsenet/synapsenet
