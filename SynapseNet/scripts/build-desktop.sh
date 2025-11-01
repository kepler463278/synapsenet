#!/bin/bash
# Build script for SynapseNet Desktop v1.0

set -e

echo "🚀 Building SynapseNet Desktop v1.0..."
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Navigate to desktop app
cd apps/desktop

echo -e "${BLUE}📦 Installing dependencies...${NC}"
npm install

echo ""
echo -e "${BLUE}🔨 Building application...${NC}"
npm run tauri build

echo ""
echo -e "${GREEN}✅ Build complete!${NC}"
echo ""
echo "Binaries location:"
echo "  - Windows: src-tauri/target/release/bundle/nsis/"
echo "  - macOS:   src-tauri/target/release/bundle/dmg/"
echo "  - Linux:   src-tauri/target/release/bundle/appimage/"
echo ""
echo -e "${GREEN}🎉 SynapseNet Desktop v1.0 is ready!${NC}"
