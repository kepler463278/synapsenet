#!/usr/bin/env bash
# DevNet - Local SynapseNet cluster for testing
# Usage: ./scripts/devnet.sh [start|stop|status]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
DEVNET_DIR="$PROJECT_ROOT/.devnet"
NUM_NODES=3

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

start_devnet() {
    log_info "Starting DevNet with $NUM_NODES nodes..."
    
    # Build project
    log_info "Building SynapseNet..."
    cd "$PROJECT_ROOT"
    cargo build --release
    
    # Create devnet directory
    mkdir -p "$DEVNET_DIR"
    
    # Start nodes
    for i in $(seq 1 $NUM_NODES); do
        NODE_DIR="$DEVNET_DIR/node$i"
        mkdir -p "$NODE_DIR"
        
        log_info "Starting node $i..."
        
        # Initialize node if not exists
        if [ ! -f "$NODE_DIR/node.key" ]; then
            "$PROJECT_ROOT/target/release/syn" --data-dir "$NODE_DIR" init
        fi
        
        # Start node in background (when P2P is implemented)
        # For now, just show it's ready
        log_info "Node $i ready at $NODE_DIR"
    done
    
    log_info "DevNet started successfully!"
    log_info "Node directories:"
    for i in $(seq 1 $NUM_NODES); do
        echo "  Node $i: $DEVNET_DIR/node$i"
    done
    
    log_info ""
    log_info "Try these commands:"
    echo "  syn --data-dir $DEVNET_DIR/node1 add \"Hello from node 1\""
    echo "  syn --data-dir $DEVNET_DIR/node1 query \"Hello\""
}

stop_devnet() {
    log_info "Stopping DevNet..."
    
    # Kill any running node processes (when P2P is implemented)
    # pkill -f "syn.*node" || true
    
    log_info "DevNet stopped"
}

status_devnet() {
    log_info "DevNet status:"
    
    if [ ! -d "$DEVNET_DIR" ]; then
        log_warn "DevNet not initialized"
        return
    fi
    
    for i in $(seq 1 $NUM_NODES); do
        NODE_DIR="$DEVNET_DIR/node$i"
        if [ -d "$NODE_DIR" ]; then
            GRAIN_COUNT=$("$PROJECT_ROOT/target/release/syn" --data-dir "$NODE_DIR" query "test" 2>/dev/null | grep -c "Similarity" || echo "0")
            echo "  Node $i: $GRAIN_COUNT grains"
        else
            echo "  Node $i: not initialized"
        fi
    done
}

clean_devnet() {
    log_warn "Cleaning DevNet (all data will be lost)..."
    rm -rf "$DEVNET_DIR"
    log_info "DevNet cleaned"
}

case "${1:-start}" in
    start)
        start_devnet
        ;;
    stop)
        stop_devnet
        ;;
    status)
        status_devnet
        ;;
    clean)
        clean_devnet
        ;;
    *)
        echo "Usage: $0 [start|stop|status|clean]"
        exit 1
        ;;
esac
