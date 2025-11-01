# SynapseNet Quick Start Guide

Get up and running with SynapseNet in 5 minutes!

## 1. Install

### macOS
```bash
brew install synapsenet
# or download .dmg from releases
```

### Windows
```bash
# Download .msi installer from releases
# Double-click to install
```

### Linux
```bash
# Debian/Ubuntu
sudo dpkg -i synapsenet_0.4.0_amd64.deb

# Or use AppImage
chmod +x SynapseNet-0.4.0.AppImage
./SynapseNet-0.4.0.AppImage
```

## 2. First Launch

1. **Launch** SynapseNet
2. **Choose** data directory (default: `~/.synapsenet`)
3. **Wait** for model download (~22MB)
4. **Done!** Ready to use

## 3. Add Your First Knowledge

1. Click **➕ Add**
2. Type: `"Rust prevents data races at compile time"`
3. Tags: `rust, programming`
4. Click **Add Grain**

✅ Your first grain is stored!

## 4. Search

1. Click **🔍 Search**
2. Type: `"memory safety"`
3. See your grain appear!

🎯 Semantic search finds related concepts!

## 5. Explore

- **🕸️ Graph** - Visualize connections
- **📊 Stats** - View metrics
- **⚙️ Settings** - Customize

## Next Steps

- 📖 Read the [User Guide](USER_GUIDE.md)
- 🚀 Try [batch import](USER_GUIDE.md#batch-import)
- 🌐 Enable [P2P networking](USER_GUIDE.md#enable-p2p-networking)
- 💰 Learn about [PoE rewards](USER_GUIDE.md#poe-rewards)

## Common Commands

```bash
# CLI usage
syn add "Your knowledge"
syn query "search term"
syn stats
syn export --output ./backup
```

## Need Help?

- 📖 [Full Documentation](USER_GUIDE.md)
- 💬 [Discord Community](https://discord.gg/synapsenet)
- 🐛 [Report Issues](https://github.com/yourusername/synapsenet/issues)

---

**That's it!** You're ready to build your semantic memory. 🧠✨
