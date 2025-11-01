# SynapseNet v0.5 "Mobile Emergence" - Specification

## Quick Links

- **[Requirements](requirements.md)** - 18 functional requirements with acceptance criteria
- **[Design](design.md)** - Complete architecture and component design
- **[Tasks](tasks.md)** - 20 major tasks with 100+ subtasks

## Overview

SynapseNet v0.5 brings the decentralized semantic memory network to mobile devices (iOS and Android). This release enables users to carry their personal knowledge network in their pocket, with full offline AI capabilities, encrypted memory capsules, and P2P networking optimized for mobile constraints.

## Key Features

### 📱 Native Mobile Apps
- iOS 14+ and Android 8+ support
- Built with Tauri Mobile
- React + TypeScript UI
- Native performance

### 🤖 On-Device AI
- **iOS:** CoreML (Metal GPU acceleration)
- **Android:** NNAPI (NPU/GPU acceleration)
- **Fallback:** CPU (ONNX Runtime Mobile)
- **Model:** all-MiniLM-L6-v2 (384-dim, ~30MB)

### 🔐 Encrypted Memory Capsules
- AES-256-GCM encryption
- iOS Keychain / Android Keystore
- 12-word recovery phrase
- Kyber KEM for backup
- Biometric authentication

### 🌍 Mobile P2P Networking
- WebRTC for NAT traversal
- Circuit Relay v2 fallback
- Operation queue for offline
- Battery-aware sync
- WiFi/Cellular detection

### 🔋 Battery Optimized
- Adaptive processing
- Background task scheduling
- Thermal monitoring
- < 5% battery per day (background)

### 🔒 Privacy-First
- Only embeddings transmitted
- Air-gap mode available
- User-controlled sharing
- No raw data leaves device

## Getting Started

### Prerequisites

```bash
# Install Rust mobile targets
rustup target add aarch64-apple-ios x86_64-apple-ios
rustup target add aarch64-linux-android armv7-linux-androideabi

# Install tools
cargo install cargo-ndk
cargo install tauri-cli --version "^2.0.0-beta"

# iOS: Install Xcode 14+
# Android: Install Android Studio + NDK
```

### Project Structure

```
apps/mobile/                    # Tauri Mobile app
├── src-tauri/                  # Rust backend
│   ├── src/
│   │   ├── commands.rs         # Tauri commands
│   │   ├── mobile/             # Mobile-specific code
│   │   │   ├── ai.rs           # AI providers
│   │   │   ├── capsule.rs      # Encrypted storage
│   │   │   ├── p2p.rs          # Mobile P2P
│   │   │   └── background.rs   # Background tasks
│   │   └── main.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                        # React UI
│   ├── screens/
│   │   ├── Home.tsx
│   │   ├── Add.tsx
│   │   ├── Query.tsx
│   │   ├── Peers.tsx
│   │   ├── Wallet.tsx
│   │   └── Settings.tsx
│   ├── components/
│   └── App.tsx
└── package.json

crates/core/src/mobile/         # Mobile core code
crates/p2p/src/mobile/          # Mobile P2P code
crates/ai/src/mobile/           # Mobile AI providers
```

### Development Workflow

```bash
# 1. Create project
cd apps
cargo tauri init --mobile

# 2. iOS development
cd mobile
cargo tauri ios dev

# 3. Android development
cargo tauri android dev

# 4. Build release
cargo tauri ios build --release
cargo tauri android build --release
```

## Implementation Timeline

### Phase 1: Foundation (Weeks 1-2)
- Mobile development environment
- Mobile AI provider system
- Encrypted memory capsule

### Phase 2: Networking (Weeks 3-4)
- Mobile P2P networking
- Background task scheduler

### Phase 3: Application (Weeks 5-7)
- Tauri Mobile application
- React UI components

### Phase 4: Features (Weeks 8-9)
- Voice input
- File import
- Notifications
- PoE rewards
- Accessibility
- Localization

### Phase 5: Quality (Weeks 10-11)
- Testing
- Performance optimization
- Security features

### Phase 6: Release (Week 12)
- App store preparation
- Documentation
- CI/CD
- Release

**Total:** ~12 weeks

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Mobile App (Tauri)                    │
├─────────────────────────────────────────────────────────┤
│  React UI Layer                                          │
│  ├─ Home, Add, Query, Peers, Wallet, Settings          │
│  └─ Voice Input, File Import, Notifications             │
├─────────────────────────────────────────────────────────┤
│  Tauri Command Bridge (IPC)                             │
│  └─ syn_init, syn_add, syn_query, syn_stats, etc.      │
├─────────────────────────────────────────────────────────┤
│  Rust Core (Mobile-Optimized)                           │
│  ├─ Mobile AI Provider (CoreML/NNAPI/CPU)              │
│  ├─ Encrypted Capsule Storage                           │
│  ├─ Mobile P2P Manager (WebRTC/Relay)                  │
│  ├─ Background Task Scheduler                           │
│  └─ Battery/Performance Monitor                         │
├─────────────────────────────────────────────────────────┤
│  Platform Services                                       │
│  ├─ iOS: Keychain, CoreML, BackgroundTasks             │
│  └─ Android: Keystore, NNAPI, ForegroundService        │
└─────────────────────────────────────────────────────────┘
```

## Success Criteria

- ✅ Published on App Store and Play Store
- ✅ 1000+ downloads in first month
- ✅ < 10 critical bugs reported
- ✅ Average rating > 4.0 stars
- ✅ Battery usage < 5% per day (background)
- ✅ Positive user feedback on privacy

## Resources

### Documentation
- [Tauri Mobile](https://tauri.app/v1/guides/building/mobile)
- [CoreML](https://developer.apple.com/documentation/coreml)
- [NNAPI](https://developer.android.com/ndk/guides/neuralnetworks)
- [libp2p WebRTC](https://github.com/libp2p/rust-libp2p/tree/master/transports/webrtc)

### Community
- **Discord:** [discord.gg/synapsenet](https://discord.gg/synapsenet)
- **GitHub:** [github.com/synapsenet/synapsenet](https://github.com/synapsenet/synapsenet)

## License

MIT OR Apache-2.0

---

**Version:** 0.5.0  
**Status:** Specification Complete  
**Last Updated:** 2024-10-31
