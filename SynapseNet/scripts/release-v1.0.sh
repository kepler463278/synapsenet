#!/bin/bash
# Release script for SynapseNet v1.0

set -e

VERSION="1.0.0"
TAG="v${VERSION}"

echo "🚀 Preparing SynapseNet v1.0 Release..."
echo ""

# Check if we're on main branch
BRANCH=$(git branch --show-current)
if [ "$BRANCH" != "main" ]; then
    echo "⚠️  Warning: Not on main branch (current: $BRANCH)"
    read -p "Continue anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check for uncommitted changes
if [[ -n $(git status -s) ]]; then
    echo "⚠️  Warning: You have uncommitted changes"
    read -p "Continue anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Create release notes
echo "📝 Creating release notes..."
cat > RELEASE_NOTES_v1.0.md << 'EOF'
# SynapseNet v1.0 "Public Genesis" 🚀

**The moment SynapseNet becomes accessible to everyone.**

## What's New

### Desktop Application
- ✅ One-click node deployment
- ✅ Real-time NGT reward tracking
- ✅ Knowledge search across the network
- ✅ Simple grain contribution interface
- ✅ Offline-first operation
- ✅ Cross-platform support (Windows, macOS, Linux)

### Genesis Manifest
- ✅ Complete philosophical foundation
- ✅ Declaration of cognitive independence
- ✅ Homo Conexus vision
- ✅ Core principles and commitments

### Public Website
- ✅ synapsenet.org launched
- ✅ Comprehensive documentation
- ✅ Download page for all platforms
- ✅ Getting started guide

## Download

- **Windows:** [SynapseNet-1.0.0-windows.exe](https://github.com/synapsenet/synapsenet/releases/download/v1.0.0/SynapseNet-1.0.0-windows.exe)
- **macOS:** [SynapseNet-1.0.0-macos.dmg](https://github.com/synapsenet/synapsenet/releases/download/v1.0.0/SynapseNet-1.0.0-macos.dmg)
- **Linux:** [SynapseNet-1.0.0-linux.AppImage](https://github.com/synapsenet/synapsenet/releases/download/v1.0.0/SynapseNet-1.0.0-linux.AppImage)

## Installation

### Windows
1. Download the `.exe` installer
2. Run the installer
3. Launch SynapseNet from Start menu

### macOS
1. Download the `.dmg` file
2. Open and drag to Applications
3. Launch SynapseNet from Applications

### Linux
1. Download the `.AppImage` file
2. Make executable: `chmod +x SynapseNet-1.0.0-linux.AppImage`
3. Run: `./SynapseNet-1.0.0-linux.AppImage`

## Getting Started

1. Launch SynapseNet
2. Click "Start Node"
3. Add your first grain
4. Earn your first NGT
5. You're now part of the global mind!

## Links

- Website: https://synapsenet.org
- Documentation: https://synapsenet.org/docs
- Genesis Manifest: https://synapsenet.org/whitepaper
- Getting Started: https://synapsenet.org/join

## What This Means

v1.0 is the transition from developer tool to human tool. This is when:
- Technology meets philosophy
- Complexity becomes simplicity
- Private becomes public
- Code becomes movement

**The network is ready to meet the world!** 🌍✨

---

*"This is not just software. This is the foundation of collective intelligence. This is how we think together."*

— Genesis Manifest v1.0
EOF

echo "✅ Release notes created"
echo ""

# Tag the release
echo "🏷️  Creating git tag: $TAG"
git tag -a "$TAG" -m "SynapseNet v1.0 'Public Genesis' - The moment SynapseNet becomes accessible to everyone"

echo ""
echo "✅ Release preparation complete!"
echo ""
echo "Next steps:"
echo "  1. Build binaries: ./scripts/build-desktop.sh"
echo "  2. Test on all platforms"
echo "  3. Push tag: git push origin $TAG"
echo "  4. Create GitHub release with binaries"
echo "  5. Deploy website to synapsenet.org"
echo "  6. Post on HackerNews"
echo ""
echo "🎉 Ready for the new era!"
