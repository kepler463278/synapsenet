#!/bin/bash
set -e

echo "🚀 Initializing SynapseNet Testnet..."

CHAIN_ID="synapsenet-testnet-1"
DENOM="ungt"
VALIDATOR_NAME="validator"

# Initialize chain
echo "📦 Initializing chain..."
wasmd init synapsenet --chain-id $CHAIN_ID

# Create validator key
echo "🔑 Creating validator key..."
wasmd keys add $VALIDATOR_NAME --keyring-backend test

# Get validator address
VALIDATOR_ADDR=$(wasmd keys show $VALIDATOR_NAME -a --keyring-backend test)

# Add genesis account
echo "💰 Adding genesis account..."
wasmd add-genesis-account $VALIDATOR_ADDR 1000000000000$DENOM

# Create genesis transaction
echo "📝 Creating genesis transaction..."
wasmd gentx $VALIDATOR_NAME 100000000$DENOM \
  --chain-id $CHAIN_ID \
  --keyring-backend test

# Collect genesis transactions
echo "🔗 Collecting genesis transactions..."
wasmd collect-gentxs

# Update config
echo "⚙️  Updating configuration..."
sed -i 's/cors_allowed_origins = \[\]/cors_allowed_origins = ["*"]/' ~/.wasmd/config/config.toml
sed -i 's/enable = false/enable = true/' ~/.wasmd/config/app.toml
sed -i 's/swagger = false/swagger = true/' ~/.wasmd/config/app.toml

echo "✅ Testnet initialized successfully!"
echo "Validator address: $VALIDATOR_ADDR"
echo ""
echo "To start the chain:"
echo "  wasmd start"
