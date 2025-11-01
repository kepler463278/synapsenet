# Building SynapseNet v0.4 Installers

This guide explains how to build cross-platform installers for SynapseNet v0.4.

---

## Prerequisites

### All Platforms
- Rust 1.70+
- Node.js 18+
- Git

### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install dependencies
brew install cmake pkg-config
```

### Windows
```bash
# Install Visual Studio 2022 with C++ tools
# Install WebView2 (usually pre-installed on Windows 11)

# Install dependencies via chocolatey
choco install cmake
```

### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.0-dev \
  build-essential \
  curl \
  wget \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  cmake \
  pkg-config
```

---

## Build Commands

### macOS (.dmg)

```bash
# Navigate to Tauri app
cd crates/tauri-app

# Install dependencies
npm install

# Build for macOS (Universal Binary)
cargo tauri build --target universal-apple-darwin

# Output: target/release/bundle/dmg/SynapseNet_0.4.0_universal.dmg
```

**Code Signing (Optional):**
```bash
# Set up signing identity
export APPLE_SIGNING_IDENTITY="Developer ID Application: Your Name"

# Build with signing
cargo tauri build --target universal-apple-darwin
```

### Windows (.msi)

```bash
# Navigate to Tauri app
cd crates/tauri-app

# Install dependencies
npm install

# Build for Windows
cargo tauri build --target x86_64-pc-windows-msvc

# Output: target/release/bundle/msi/SynapseNet_0.4.0_x64_en-US.msi
```

### Linux (.deb)

```bash
# Navigate to Tauri app
cd crates/tauri-app

# Install dependencies
npm install

# Build for Linux
cargo tauri build --target x86_64-unknown-linux-gnu

# Output: target/release/bundle/deb/synapsenet_0.4.0_amd64.deb
```

### Linux (.AppImage)

```bash
# Navigate to Tauri app
cd crates/tauri-app

# Install dependencies
npm install

# Build AppImage
cargo tauri build --target x86_64-unknown-linux-gnu --bundles appimage

# Output: target/release/bundle/appimage/synapsenet_0.4.0_amd64.AppImage
```

---

## Configuration

### tauri.conf.json

Key settings for installers:

```json
{
  "package": {
    "productName": "SynapseNet",
    "version": "0.4.0"
  },
  "tauri": {
    "bundle": {
      "identifier": "io.synapsenet.app",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "macOS": {
        "minimumSystemVersion": "10.15"
      },
      "windows": {
        "certificateThumbprint": null,
        "digestAlgorithm": "sha256",
        "timestampUrl": ""
      }
    }
  }
}
```

---

## Testing Installers

### macOS
```bash
# Mount DMG
open target/release/bundle/dmg/SynapseNet_0.4.0_universal.dmg

# Test installation
# Drag to Applications
# Launch from Applications folder
```

### Windows
```bash
# Run installer
target/release/bundle/msi/SynapseNet_0.4.0_x64_en-US.msi

# Test installation
# Check Start Menu
# Launch application
```

### Linux (.deb)
```bash
# Install
sudo dpkg -i target/release/bundle/deb/synapsenet_0.4.0_amd64.deb

# Test
synapsenet

# Uninstall
sudo apt remove synapsenet
```

### Linux (.AppImage)
```bash
# Make executable
chmod +x target/release/bundle/appimage/synapsenet_0.4.0_amd64.AppImage

# Run
./target/release/bundle/appimage/synapsenet_0.4.0_amd64.AppImage
```

---

## Troubleshooting

### macOS: Code Signing Issues
```bash
# Check signing identity
security find-identity -v -p codesigning

# Sign manually
codesign --force --deep --sign "Developer ID" SynapseNet.app
```

### Windows: Missing WebView2
```bash
# Download WebView2 Runtime
# https://developer.microsoft.com/en-us/microsoft-edge/webview2/
```

### Linux: Missing Dependencies
```bash
# Install all webkit dependencies
sudo apt install libwebkit2gtk-4.0-37
```

---

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Build Installers

on:
  push:
    tags:
      - 'v*'

jobs:
  build-macos:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
      - run: cd crates/tauri-app && npm install
      - run: cd crates/tauri-app && cargo tauri build
      - uses: actions/upload-artifact@v3
        with:
          name: macos-dmg
          path: crates/tauri-app/target/release/bundle/dmg/*.dmg

  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
      - run: cd crates/tauri-app && npm install
      - run: cd crates/tauri-app && cargo tauri build
      - uses: actions/upload-artifact@v3
        with:
          name: windows-msi
          path: crates/tauri-app/target/release/bundle/msi/*.msi

  build-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
      - run: sudo apt update && sudo apt install -y libwebkit2gtk-4.0-dev
      - run: cd crates/tauri-app && npm install
      - run: cd crates/tauri-app && cargo tauri build
      - uses: actions/upload-artifact@v3
        with:
          name: linux-packages
          path: |
            crates/tauri-app/target/release/bundle/deb/*.deb
            crates/tauri-app/target/release/bundle/appimage/*.AppImage
```

---

## Distribution

### Checksums

```bash
# Generate SHA256 checksums
cd target/release/bundle
shasum -a 256 dmg/*.dmg > checksums.txt
shasum -a 256 msi/*.msi >> checksums.txt
shasum -a 256 deb/*.deb >> checksums.txt
shasum -a 256 appimage/*.AppImage >> checksums.txt
```

### Upload to GitHub Releases

```bash
# Using GitHub CLI
gh release create v0.4.0 \
  target/release/bundle/dmg/*.dmg \
  target/release/bundle/msi/*.msi \
  target/release/bundle/deb/*.deb \
  target/release/bundle/appimage/*.AppImage \
  checksums.txt \
  --title "SynapseNet v0.4.0" \
  --notes-file RELEASE_NOTES_v0.4.md
```

---

## File Sizes (Approximate)

- **macOS .dmg:** ~50-70 MB
- **Windows .msi:** ~40-60 MB
- **Linux .deb:** ~40-60 MB
- **Linux .AppImage:** ~50-70 MB

---

## Support

For build issues:
- Check [Tauri Documentation](https://tauri.app/v1/guides/building/)
- Open issue on GitHub
- Ask in Discord #dev channel

---

**Last Updated:** 2024-10-31  
**Version:** 0.4.0
