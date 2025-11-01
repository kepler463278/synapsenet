#!/bin/bash
# Build SynapseNet Desktop for all platforms

set -e

echo "🚀 Building SynapseNet Desktop v1.0 for ALL platforms..."
echo ""

cd apps/desktop

# Install dependencies
echo "📦 Installing dependencies..."
npm install

# Build for current platform
echo ""
echo "🔨 Building for current platform..."
npm run tauri build

# Cross-compilation instructions
echo ""
echo "📝 Cross-compilation notes:"
echo ""
echo "For Windows (from Linux/macOS):"
echo "  rustup target add x86_64-pc-windows-msvc"
echo "  npm run tauri build -- --target x86_64-pc-windows-msvc"
echo ""
echo "For macOS Intel (from macOS):"
echo "  rustup target add x86_64-apple-darwin"
echo "  npm run tauri build -- --target x86_64-apple-darwin"
echo ""
echo "For macOS Apple Silicon (from macOS):"
echo "  rustup target add aarch64-apple-darwin"
echo "  npm run tauri build -- --target aarch64-apple-darwin"
echo ""
echo "For Linux (from Linux):"
echo "  rustup target add x86_64-unknown-linux-gnu"
echo "  npm run tauri build -- --target x86_64-unknown-linux-gnu"
echo ""
echo "✅ Build complete for current platform!"
echo ""
echo "Output locations:"
echo "  - Windows: src-tauri/target/release/bundle/nsis/"
echo "  - macOS:   src-tauri/target/release/bundle/dmg/"
echo "  - Linux:   src-tauri/target/release/bundle/appimage/"
