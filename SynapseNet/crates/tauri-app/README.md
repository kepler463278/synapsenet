# SynapseNet Tauri App

Native desktop application for SynapseNet with Web UI.

## Prerequisites

- Rust 1.70+
- Node.js 18+
- npm or yarn

## Development

1. Install frontend dependencies:
```bash
cd crates/tauri-app
npm install
```

2. Run in development mode:
```bash
npm run tauri:dev
```

This will:
- Start the Vite dev server (frontend)
- Build and run the Tauri app (Rust backend)
- Enable hot-reload for both frontend and backend

## Building

Build production binaries:

```bash
npm run tauri:build
```

This creates installers in `src-tauri/target/release/bundle/`:
- macOS: `.dmg`
- Windows: `.msi`
- Linux: `.deb` and `.AppImage`

## Project Structure

```
crates/tauri-app/
├── src/              # Rust backend (Tauri commands)
│   ├── main.rs       # Entry point
│   ├── commands.rs   # IPC command handlers
│   └── state.rs      # Application state
├── src-ui/           # React frontend
│   ├── main.tsx      # Entry point
│   ├── App.tsx       # Main app component
│   ├── components/   # UI components
│   └── styles.css    # Global styles
├── tauri.conf.json   # Tauri configuration
├── package.json      # Frontend dependencies
└── Cargo.toml        # Rust dependencies
```

## Features

- ✅ Add knowledge grains
- ✅ Semantic search
- ✅ Node statistics
- 🚧 Graph visualization (coming in Task 3)
- 🚧 Batch import (coming in Task 6)
- 🚧 P2P networking (coming in Task 5)

## Configuration

Configuration file is automatically created at:
- macOS: `~/Library/Application Support/net.synapse.app/config.toml`
- Linux: `~/.config/synapsenet/config.toml`
- Windows: `%APPDATA%\synapsenet\config.toml`

## Troubleshooting

### Model not found error

Download the embedding model first:
```bash
mkdir -p ~/.synapsenet/models
# Download all-MiniLM-L6-v2.onnx to this directory
```

### Build errors

Make sure all workspace crates are built:
```bash
cd ../..  # Go to workspace root
cargo build --workspace
```

### Frontend not loading

Check that Vite dev server is running on port 5173:
```bash
npm run dev
```
