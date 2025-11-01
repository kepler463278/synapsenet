# Building SynapseNet Installers

This guide explains how to build cross-platform installers for SynapseNet v0.4.

## Prerequisites

### All Platforms

- Rust 1.70+ with cargo
- Node.js 18+ with npm
- Tauri CLI: `cargo install tauri-cli`

### macOS

- Xcode Command Line Tools
- macOS 10.15+ for building
- (Optional) Apple Developer ID for code signing
- (Optional) Apple ID for notarization

### Windows

- Visual Studio 2019+ with C++ build tools
- Windows 10+ SDK
- (Optional) Code signing certificate

### Linux

- GCC/Clang compiler
- GTK 3 development libraries: `sudo apt install libgtk-3-dev libwebkit2gtk-4.0-dev`
- Additional dependencies: `sudo apt install libssl-dev libayatana-appindicator3-dev librsvg2-dev`

## Quick Start

### Build for Current Platform

```bash
# Build for your current platform
./scripts/build-installers.sh macos    # On macOS
./scripts/build-installers.sh linux    # On Linux
./scripts/build-installers.sh windows  # On Windows
```

### Build All Available Platforms

```bash
# Attempts to build for all platforms available on current system
./scripts/build-installers.sh all
```

### Clean Build Artifacts

```bash
./scripts/build-installers.sh clean
```

## Platform-Specific Instructions

### macOS

#### Building DMG

```bash
./scripts/build-installers.sh macos
```

This creates: `dist/SynapseNet-0.4.0-macos.dmg`

#### Code Signing (Optional)

To sign the macOS app, set your Apple Developer ID:

```bash
export APPLE_DEVELOPER_ID="Developer ID Application: Your Name (TEAM_ID)"
./scripts/build-installers.sh macos
```

#### Notarization (Optional)

To notarize the app for distribution:

```bash
export APPLE_ID="your@email.com"
export APPLE_PASSWORD="app-specific-password"
export APPLE_TEAM_ID="YOUR_TEAM_ID"
./scripts/build-installers.sh macos
```

**Note:** Generate an app-specific password at https://appleid.apple.com

#### Skip Signing/Notarization

```bash
export SKIP_SIGNING=true
export SKIP_NOTARIZATION=true
./scripts/build-installers.sh macos
```

### Windows

#### Building MSI

```bash
./scripts/build-installers.sh windows
```

This creates: `dist/SynapseNet-0.4.0-windows-x64.msi`

#### Code Signing (Optional)

To sign the Windows installer, configure in `tauri.conf.json`:

```json
{
  "bundle": {
    "windows": {
      "certificateThumbprint": "YOUR_CERT_THUMBPRINT",
      "timestampUrl": "http://timestamp.digicert.com"
    }
  }
}
```

### Linux

#### Building DEB and AppImage

```bash
./scripts/build-installers.sh linux
```

This creates:
- `dist/synapsenet-0.4.0-linux-amd64.deb`
- `dist/SynapseNet-0.4.0-linux-x86_64.AppImage`

#### Installing DEB

```bash
sudo dpkg -i dist/synapsenet-0.4.0-linux-amd64.deb
```

#### Running AppImage

```bash
chmod +x dist/SynapseNet-0.4.0-linux-x86_64.AppImage
./dist/SynapseNet-0.4.0-linux-x86_64.AppImage
```

## Manual Build Process

If you prefer to build manually without the script:

### 1. Install Dependencies

```bash
cd crates/tauri-app
npm install
```

### 2. Build Frontend

```bash
npm run build
```

### 3. Build Tauri App

```bash
# Development build
cargo tauri dev

# Production build
cargo tauri build
```

### 4. Find Artifacts

Built installers are located in:
- macOS: `crates/tauri-app/target/universal-apple-darwin/release/bundle/dmg/`
- Windows: `crates/tauri-app/target/x86_64-pc-windows-msvc/release/bundle/msi/`
- Linux: `crates/tauri-app/target/x86_64-unknown-linux-gnu/release/bundle/deb/` and `.../appimage/`

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
      - uses: actions/setup-node@v3
        with:
          node-version: 18
      - uses: dtolnay/rust-toolchain@stable
      - name: Build macOS
        run: ./scripts/build-installers.sh macos
      - uses: actions/upload-artifact@v3
        with:
          name: macos-installer
          path: dist/*.dmg

  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: 18
      - uses: dtolnay/rust-toolchain@stable
      - name: Build Windows
        run: ./scripts/build-installers.sh windows
      - uses: actions/upload-artifact@v3
        with:
          name: windows-installer
          path: dist/*.msi

  build-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: 18
      - uses: dtolnay/rust-toolchain@stable
      - name: Install dependencies
        run: |
          sudo apt update
          sudo apt install -y libgtk-3-dev libwebkit2gtk-4.0-dev \
            libssl-dev libayatana-appindicator3-dev librsvg2-dev
      - name: Build Linux
        run: ./scripts/build-installers.sh linux
      - uses: actions/upload-artifact@v3
        with:
          name: linux-installers
          path: dist/*
```

## Troubleshooting

### macOS: "App is damaged and can't be opened"

This happens with unsigned apps. Users need to:
1. Right-click the app and select "Open"
2. Click "Open" in the security dialog

Or disable Gatekeeper temporarily:
```bash
sudo spctl --master-disable
```

### Windows: "Windows protected your PC"

This happens with unsigned installers. Users need to:
1. Click "More info"
2. Click "Run anyway"

### Linux: Missing dependencies

Install required libraries:
```bash
# Debian/Ubuntu
sudo apt install libgtk-3-0 libwebkit2gtk-4.0-37

# Fedora
sudo dnf install gtk3 webkit2gtk3

# Arch
sudo pacman -S gtk3 webkit2gtk
```

### Build fails with "out of memory"

Increase system memory or build with fewer parallel jobs:
```bash
cargo tauri build -- -j 2
```

## Checksums

After building, checksums are automatically generated in `dist/checksums.txt`:

```bash
cat dist/checksums.txt
```

Users can verify downloads:
```bash
shasum -a 256 -c checksums.txt
```

## Distribution

### GitHub Releases

1. Create a new release on GitHub
2. Upload all files from `dist/` directory
3. Include `checksums.txt` for verification

### Direct Download

Host installers on your own server and provide download links:
- macOS: `https://downloads.synapsenet.io/v0.4.0/SynapseNet-0.4.0-macos.dmg`
- Windows: `https://downloads.synapsenet.io/v0.4.0/SynapseNet-0.4.0-windows-x64.msi`
- Linux DEB: `https://downloads.synapsenet.io/v0.4.0/synapsenet-0.4.0-linux-amd64.deb`
- Linux AppImage: `https://downloads.synapsenet.io/v0.4.0/SynapseNet-0.4.0-linux-x86_64.AppImage`

## Auto-Updates

SynapseNet supports automatic updates via Tauri's updater. Configure in `tauri.conf.json`:

```json
{
  "updater": {
    "active": true,
    "endpoints": [
      "https://updates.synapsenet.io/{{target}}/{{current_version}}"
    ],
    "dialog": true,
    "pubkey": "YOUR_PUBLIC_KEY"
  }
}
```

See Task 13.2 for full auto-update implementation.

## Support

For build issues:
- Check [Tauri documentation](https://tauri.app/v1/guides/building/)
- Open an issue on GitHub
- Join our Discord community

## License

SynapseNet is licensed under MIT/Apache-2.0. See LICENSE files for details.
