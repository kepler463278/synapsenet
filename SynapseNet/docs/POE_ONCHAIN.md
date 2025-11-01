# PoE On-Chain Economics

## Overview

Proof of Existence (PoE) rewards are distributed on-chain based on node contributions to reasoning, swarm consensus, and actions.

---

## Batch Structure

### PoE Batch
```rust
{
  "epoch": 1,
  "chain_id": "synapsenet-testnet-1",
  "root": "0x1234...",  // Merkle root
  "participants": ["node1", "node2", ...],
  "signers": ["validator1", "validator2", ...],
  "sigs": ["sig1", "sig2", ...]
}
```

### PoE Item
```rust
{
  "node": "node_id",
  "goal": "uuid",
  "novelty": 0.8,
  "coherence": 0.9,
  "reuse": 0.7,
  "weight": 0.85,
  "leaf_hash": "0xABCD..."
}
```

---

## Merkle Tree

### Leaf Hash
```
leaf = SHA256(node || goal || novelty || coherence || reuse || weight)
```

### Root Calculation
```
1. Hash all leaves
2. Pair and hash recursively
3. Single root hash
```

### Verification
```
1. Submit root on-chain
2. Provide leaf + proof
3. Verify path to root
```

---

## Reward Formula

### Per-Node Reward
```
reward = (reward_per_epoch * node_weight) / total_weight
```

### Weight Calculation
```
weight = α*novelty + β*coherence + γ*reuse
```

**Default weights:**
- α (novelty) = 0.4
- β (coherence) = 0.4
- γ (reuse) = 0.2

---

## Epoch Timing

### Configuration
```
epoch_secs = 600  # 10 minutes
```

### Timeline
```
00:00 - Epoch N starts
10:00 - Epoch N ends, batch submitted
10:01 - Epoch N+1 starts
```

### Submission Window
- Batches can be submitted anytime after epoch ends
- Only one batch per epoch accepted
- First valid batch wins

---

## Multi-Signature

### Requirements
```
min_signers = 3  # Minimum validator signatures
```

### Process
1. Validators collect local metrics
2. Aggregate into batch
3. Sign batch hash
4. Submit with ≥3 signatures
5. Contract verifies signatures

### Signature Format
```
sig = Ed25519(batch_hash, validator_key)
```

---

## Reward Distribution

### Accrual
```
ACCRUAL[node] += reward
```

### Claiming
```
1. Query ACCRUAL[node]
2. Submit ClaimReward transaction
3. Contract mints/transfers NGT
4. ACCRUAL[node] = 0
```

### Gas Costs
- SubmitBatch: ~200k gas
- ClaimReward: ~100k gas

---

## Statistics

### Global Stats
```
{
  "total_epochs": 1000,
  "total_rewards": "1000000 NGT",
  "total_participants": 5000
}
```

### Per-Node Stats
```
{
  "node": "node_id",
  "epochs_participated": 500,
  "total_earned": "50000 NGT",
  "avg_weight": 0.85
}
```

---

## Security

### Sybil Resistance
- Minimum stake required
- Reputation weighting
- Rate limiting

### Spam Prevention
- Gas costs
- Minimum signers
- Epoch uniqueness

### Verification
- Merkle proofs
- Multi-signature
- On-chain validation

---

## Examples

### Submit Batch
```bash
# Create batch
syn poe batch --epoch 1

# Submit
syn poe submit --batch batch_1.json
```

### Claim Rewards
```bash
# Check balance
syn poe balance --node node_id

# Claim
syn poe claim --node node_id
```

### Query Contract
```bash
# Get epoch data
wasmd query wasm contract-state smart $CONTRACT \
  '{"get_epoch":{"epoch":1}}'

# Get accrual
wasmd query wasm contract-state smart $CONTRACT \
  '{"get_accrual":{"node":"node_id"}}'
```

---

## Future Enhancements

### v1.0
- ZK proofs for privacy
- TEE verification
- Cross-chain bridges
- Automated claiming

### v1.1
- Slashing for misbehavior
- Delegation
- Governance integration
- Advanced metrics
