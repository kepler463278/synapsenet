# SynapseNet Mobile v0.5

Your Personal Knowledge Network - Mobile Edition

## Overview

SynapseNet Mobile is a cross-platform mobile application (iOS/Android) that enables users to create, store, and query their personal knowledge base using AI-powered semantic search, with peer-to-peer synchronization and proof-of-emergence rewards.

## Features

### 🧠 Knowledge Management
- Create knowledge grains with text or voice input
- Import from files (txt, md, json, csv)
- Semantic search with AI embeddings
- Tag-based organization

### 🎤 Voice Input
- iOS SFSpeechRecognizer integration
- Android SpeechRecognizer integration
- Real-time transcription
- Multiple language support

### 💰 PoE Rewards
- Earn NGT tokens for novel knowledge
- Novelty and coherence scoring
- Reuse tracking with bonuses
- Reward history and breakdown

### 🌐 P2P Networking
- WebRTC connections
- Circuit relay fallback
- Background synchronization
- Offline-first architecture

### 🔔 Smart Notifications
- Sync completion alerts
- New peer connections
- Reward notifications
- Backup reminders

### ♿ Accessibility
- VoiceOver (iOS) support
- TalkBack (Android) support
- High contrast mode
- Dynamic text sizing
- WCAG 2.1 AA compliant

### 🌍 Localization
- English
- Russian
- Spanish
- Chinese

## Tech Stack

- **Frontend:** React + TypeScript
- **Backend:** Rust + Tauri
- **AI:** CoreML (iOS), NNAPI (Android), ONNX (fallback)
- **Encryption:** AES-256-GCM + Kyber KEM
- **P2P:** libp2p with WebRTC
- **i18n:** i18next

## Getting Started

### Prerequisites
- Node.js 18+
- Rust 1.70+
- Xcode 15+ (for iOS)
- Android Studio (for Android)

### Installation

```bash
# Clone repository
git clone https://github.com/yourusername/synapsenet.git
cd synapsenet/apps/mobile

# Install dependencies
npm install

# Install Tauri CLI
cargo install tauri-cli

# Run development server
npm run tauri dev
```

### Build

```bash
# iOS
npm run tauri build -- --target ios

# Android
npm run tauri build -- --target android
```

## Project Structure

```
apps/mobile/
├── src/                    # React frontend
│   ├── screens/           # Main screens
│   ├── components/        # Reusable components
│   ├── hooks/            # Custom hooks
│   ├── i18n/             # Translations
│   └── styles.css        # Global styles
├── src-tauri/            # Rust backend
│   └── src/
│       ├── main.rs       # Entry point
│       ├── commands.rs   # Tauri commands
│       ├── voice.rs      # Voice input
│       ├── file_import.rs # File parsing
│       ├── notifications.rs # Push notifications
│       ├── poe.rs        # PoE calculations
│       └── accessibility.rs # Accessibility
├── TESTING.md            # Testing guide
├── DEPLOYMENT.md         # Deployment guide
├── PERFORMANCE.md        # Performance guide
├── SECURITY.md           # Security guide
└── README.md             # This file
```

## Documentation

- [Testing Guide](./TESTING.md)
- [Deployment Guide](./DEPLOYMENT.md)
- [Performance Guide](./PERFORMANCE.md)
- [Security Guide](./SECURITY.md)

## Development

### Running Tests

```bash
# Rust tests
cd src-tauri
cargo test

# React tests
npm test
```

### Code Style

```bash
# Format Rust
cargo fmt

# Lint Rust
cargo clippy

# Format TypeScript
npm run format

# Lint TypeScript
npm run lint
```

## Architecture

### Frontend (React)
- **Screens:** 6 main screens (Home, Add, Query, Peers, Wallet, Settings)
- **Components:** Reusable UI components
- **Hooks:** Custom React hooks for state management
- **i18n:** Multi-language support

### Backend (Rust)
- **Commands:** 34 Tauri commands for frontend-backend communication
- **Modules:** 7 specialized modules (voice, file_import, notifications, poe, accessibility)
- **State:** Centralized app state management
- **Platform:** iOS/Android specific implementations

## Security

- AES-256-GCM encryption for local data
- Kyber KEM for quantum-resistant key exchange
- iOS Keychain / Android Keystore integration
- Biometric authentication support
- Air-gap mode for offline operation

## Performance

- App launch: < 2s
- Screen transition: < 300ms
- Memory usage: < 150MB
- Battery drain: < 5%/hour (idle)

## Accessibility

- Screen reader support (VoiceOver/TalkBack)
- High contrast mode
- Reduced motion support
- Dynamic text sizing
- Keyboard navigation
- WCAG 2.1 AA compliant

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under MIT / Apache-2.0.

## Acknowledgments

- Tauri for cross-platform mobile framework
- React for UI framework
- Rust for backend performance
- libp2p for P2P networking
- i18next for localization

## Support

- Documentation: [docs/mobile/](../../docs/mobile/)
- Issues: [GitHub Issues](https://github.com/yourusername/synapsenet/issues)
- Discussions: [GitHub Discussions](https://github.com/yourusername/synapsenet/discussions)

## Roadmap

- [x] Core features (v0.5)
- [x] Voice input
- [x] File import
- [x] Notifications
- [x] PoE rewards
- [x] Accessibility
- [x] Localization
- [ ] Biometric authentication
- [ ] Advanced analytics
- [ ] More languages
- [ ] Desktop sync

## Status

**Version:** 0.5.0  
**Status:** Beta  
**Platform:** iOS 14+, Android 8+  
**Last Updated:** 2024-10-31

---

Made with ❤️ by the SynapseNet team
