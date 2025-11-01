# SynapseNet Desktop v1.0

Decentralized Intelligence for Everyone.

## Overview

SynapseNet Desktop is a Tauri-based application that provides a simple, beautiful interface for running a SynapseNet node. No terminal, no commands - just one-click operation.

## Features

- **One-Click Node Operation** - Start and stop your node with a single click
- **Real-Time Rewards** - See your NGT earnings as they happen
- **Knowledge Search** - Search across the entire network
- **Easy Contribution** - Add your knowledge and earn rewards
- **Offline-First** - Works even without internet connection
- **Privacy by Design** - All data stored locally, you control what you share

## Development

### Prerequisites

- Rust 1.70+
- Node.js 18+
- npm or yarn

### Setup

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri:dev

# Build for production
npm run tauri:build
```

### Project Structure

```
apps/desktop/
├── src-tauri/          # Rust backend
│   ├── src/
│   │   ├── main.rs     # Entry point
│   │   ├── commands.rs # Tauri commands
│   │   ├── node.rs     # Node management
│   │   ├── state.rs    # Application state
│   │   └── events.rs   # Real-time events
│   └── Cargo.toml
├── src/                # React frontend
│   ├── App.tsx         # Main component
│   ├── screens/        # Screen components
│   └── styles.css      # Styling
└── package.json
```

## Building

### Windows

```bash
npm run tauri build -- --target x86_64-pc-windows-msvc
```

Output: `src-tauri/target/release/bundle/nsis/SynapseNet_1.0.0_x64-setup.exe`

### macOS

```bash
# Intel
npm run tauri build -- --target x86_64-apple-darwin

# Apple Silicon
npm run tauri build -- --target aarch64-apple-darwin

# Universal
npm run tauri build
```

Output: `src-tauri/target/release/bundle/dmg/SynapseNet_1.0.0_x64.dmg`

### Linux

```bash
npm run tauri build -- --target x86_64-unknown-linux-gnu
```

Output: `src-tauri/target/release/bundle/appimage/synapsenet-desktop_1.0.0_amd64.AppImage`

## Architecture

### Backend (Rust)

The Tauri backend handles:
- Node lifecycle management
- P2P network communication
- Local storage
- Reward calculation
- Real-time event emission

### Frontend (React)

The React frontend provides:
- Clean, intuitive UI
- Real-time updates
- Screen navigation
- User interactions

### Communication

Frontend and backend communicate via:
- Tauri commands (frontend → backend)
- Event emission (backend → frontend)

## Integration with SynapseNet Core

The desktop app integrates with existing SynapseNet crates:
- `synapsenet-core` - Core functionality
- `synapsenet-p2p` - P2P networking
- `synapsenet-storage` - Data storage
- `synapsenet-swarm` - Swarm intelligence
- `synapsenet-ai` - AI/ML capabilities

## Performance

- **Startup Time:** < 3 seconds
- **Memory Usage:** < 200MB
- **Bundle Size:** < 100MB
- **Search Response:** < 2 seconds

## Security

- Tauri's security model (no Node.js runtime)
- Content Security Policy (CSP)
- Sandboxed WebView
- Local-first data storage
- No telemetry or tracking

## License

MIT

## Links

- Website: https://synapsenet.org
- GitHub: https://github.com/synapsenet/synapsenet
- Documentation: https://synapsenet.org/docs
- Genesis Manifest: https://synapsenet.org/whitepaper

---

**The network is ready to meet the world!** 🌍✨
