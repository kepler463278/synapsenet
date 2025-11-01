#!/bin/bash
set -e

echo "🚀 Deploying SynapseNet Contracts..."

CHAIN_ID="synapsenet-testnet-1"
NODE="http://localhost:26657"
VALIDATOR="validator"

# Build contracts
echo "🔨 Building contracts..."
cd chain/poe_wasm
cargo wasm
cd ../..

# Optimize WASM
echo "📦 Optimizing WASM..."
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.13

# Store PoE contract
echo "📤 Storing PoE contract..."
POE_CODE_ID=$(wasmd tx wasm store artifacts/poe_wasm.wasm \
  --from $VALIDATOR \
  --chain-id $CHAIN_ID \
  --node $NODE \
  --gas auto \
  --gas-adjustment 1.3 \
  --keyring-backend test \
  --yes \
  --output json | jq -r '.logs[0].events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')

echo "✅ PoE contract stored with code ID: $POE_CODE_ID"

# Instantiate PoE contract
echo "🎬 Instantiating PoE contract..."
INIT_MSG='{"epoch_secs":600,"min_signers":3,"reward_per_epoch":"1000000000"}'

POE_ADDR=$(wasmd tx wasm instantiate $POE_CODE_ID "$INIT_MSG" \
  --from $VALIDATOR \
  --chain-id $CHAIN_ID \
  --node $NODE \
  --label "poe" \
  --admin $(wasmd keys show $VALIDATOR -a --keyring-backend test) \
  --gas auto \
  --gas-adjustment 1.3 \
  --keyring-backend test \
  --yes \
  --output json | jq -r '.logs[0].events[] | select(.type=="instantiate") | .attributes[] | select(.key=="_contract_address") | .value')

echo "✅ PoE contract instantiated at: $POE_ADDR"

# Save addresses
echo "💾 Saving contract addresses..."
cat > contracts.json <<EOF
{
  "poe": {
    "code_id": $POE_CODE_ID,
    "address": "$POE_ADDR"
  }
}
EOF

echo ""
echo "🎉 Deployment complete!"
echo "Contract addresses saved to contracts.json"
