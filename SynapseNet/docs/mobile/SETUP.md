# Mobile Development Setup Guide

This guide will help you set up your development environment for building SynapseNet mobile apps.

## Prerequisites

### All Platforms

- **Rust** 1.70+ with cargo
- **Node.js** 18+ with npm
- **Git**

### iOS Development (macOS only)

- **macOS** 12.0+ (Monterey or later)
- **Xcode** 14.0+ with Command Line Tools
- **CocoaPods** (for iOS dependencies)

### Android Development

- **Android Studio** 2022.1+ (Electric Eel or later)
- **Android SDK** API Level 26+ (Android 8.0+)
- **Android NDK** r25+
- **Java JDK** 11+

## Installation Steps

### 1. Install Rust Mobile Targets

```bash
# iOS targets
rustup target add aarch64-apple-ios
rustup target add x86_64-apple-ios
rustup target add aarch64-apple-ios-sim

# Android targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android
```

### 2. Install Mobile Development Tools

```bash
# Tauri CLI (mobile support)
cargo install tauri-cli --version "^2.0.0-beta"

# Android NDK helper
cargo install cargo-ndk

# iOS development tools (macOS only)
cargo install cargo-lipo
```

### 3. Set Up iOS Development (macOS only)

#### Install Xcode

1. Download Xcode from the App Store
2. Install Command Line Tools:
   ```bash
   xcode-select --install
   ```

#### Install CocoaPods

```bash
sudo gem install cocoapods
```

#### Configure Code Signing

1. Open Xcode
2. Go to Preferences > Accounts
3. Add your Apple ID
4. Select your team
5. Download certificates

### 4. Set Up Android Development

#### Install Android Studio

1. Download from [developer.android.com](https://developer.android.com/studio)
2. Run the installer
3. Follow the setup wizard

#### Install SDK and NDK

1. Open Android Studio
2. Go to Settings > Appearance & Behavior > System Settings > Android SDK
3. Install:
   - Android SDK Platform 26+ (Android 8.0+)
   - Android SDK Build-Tools
   - Android SDK Platform-Tools
   - Android SDK Tools
4. Go to SDK Tools tab
5. Install:
   - Android NDK (Side by side)
   - CMake

#### Set Environment Variables

Add to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.):

```bash
# Android SDK
export ANDROID_HOME=$HOME/Library/Android/sdk  # macOS
# export ANDROID_HOME=$HOME/Android/Sdk        # Linux

# Android NDK
export NDK_HOME=$ANDROID_HOME/ndk/25.2.9519653  # Adjust version

# Add to PATH
export PATH=$PATH:$ANDROID_HOME/emulator
export PATH=$PATH:$ANDROID_HOME/platform-tools
export PATH=$PATH:$ANDROID_HOME/tools
export PATH=$PATH:$ANDROID_HOME/tools/bin
```

Reload your shell:
```bash
source ~/.zshrc  # or ~/.bashrc
```

### 5. Verify Installation

#### Check Rust Targets

```bash
rustup target list --installed | grep -E "(ios|android)"
```

You should see:
```
aarch64-apple-ios
aarch64-apple-ios-sim
aarch64-linux-android
armv7-linux-androideabi
x86_64-apple-ios
x86_64-linux-android
```

#### Check Tauri CLI

```bash
cargo tauri --version
```

Should output: `tauri-cli 2.0.0-beta.x`

#### Check Android Setup

```bash
# Check SDK
echo $ANDROID_HOME
ls $ANDROID_HOME

# Check NDK
echo $NDK_HOME
ls $NDK_HOME

# Check ADB
adb version
```

#### Check iOS Setup (macOS only)

```bash
# Check Xcode
xcodebuild -version

# Check CocoaPods
pod --version

# Check iOS simulators
xcrun simctl list devices
```

## Project Setup

### Initialize Mobile Project

```bash
cd apps/mobile
cargo tauri init --mobile
```

This will create:
- `src-tauri/` - Rust backend
- `src/` - React frontend
- iOS and Android project files

### Install Dependencies

```bash
# Install npm dependencies
npm install

# Install iOS pods (macOS only)
cd src-tauri/gen/apple
pod install
cd ../../..

# Sync Android dependencies
cd src-tauri
cargo tauri android init
cd ..
```

## Development Workflow

### iOS Development

```bash
# Run on simulator
cargo tauri ios dev

# Run on specific simulator
cargo tauri ios dev --target "iPhone 14 Pro"

# Run on physical device
cargo tauri ios dev --device "Your iPhone"

# Build release
cargo tauri ios build --release
```

### Android Development

```bash
# Run on emulator
cargo tauri android dev

# Run on specific emulator
cargo tauri android dev --target emulator-5554

# Run on physical device
cargo tauri android dev --device

# Build release
cargo tauri android build --release
```

## Troubleshooting

### iOS Issues

**Problem:** "No signing certificate found"
```bash
# Solution: Configure signing in Xcode
open src-tauri/gen/apple/synapsenet.xcodeproj
# Go to Signing & Capabilities, select your team
```

**Problem:** "Pod install failed"
```bash
# Solution: Update CocoaPods
sudo gem install cocoapods
pod repo update
```

**Problem:** "Simulator not found"
```bash
# Solution: List available simulators
xcrun simctl list devices
# Use exact name from list
```

### Android Issues

**Problem:** "NDK not found"
```bash
# Solution: Set NDK_HOME
export NDK_HOME=$ANDROID_HOME/ndk/25.2.9519653
```

**Problem:** "SDK license not accepted"
```bash
# Solution: Accept licenses
$ANDROID_HOME/tools/bin/sdkmanager --licenses
```

**Problem:** "Emulator won't start"
```bash
# Solution: Create new AVD
$ANDROID_HOME/tools/bin/avdmanager create avd \
  -n Pixel_6_API_33 \
  -k "system-images;android-33;google_apis;x86_64"
```

### General Issues

**Problem:** "Rust target not found"
```bash
# Solution: Install missing target
rustup target add <target-name>
```

**Problem:** "Build fails with linker error"
```bash
# Solution: Clean and rebuild
cargo clean
cargo tauri build
```

## IDE Setup

### VS Code

Install extensions:
- Rust Analyzer
- Tauri
- React
- ESLint
- Prettier

### Android Studio

1. Open `apps/mobile/src-tauri/gen/android` as Android project
2. Sync Gradle
3. Use for Android-specific debugging

### Xcode

1. Open `apps/mobile/src-tauri/gen/apple/synapsenet.xcodeproj`
2. Use for iOS-specific debugging
3. Configure signing and capabilities

## Next Steps

- Read [BUILD_iOS.md](BUILD_iOS.md) for iOS-specific build instructions
- Read [BUILD_Android.md](BUILD_Android.md) for Android-specific build instructions
- Check [ARCHITECTURE.md](ARCHITECTURE.md) for mobile architecture overview

## Resources

- [Tauri Mobile Docs](https://tauri.app/v1/guides/building/mobile)
- [iOS Developer](https://developer.apple.com/ios/)
- [Android Developer](https://developer.android.com/)
- [Rust Mobile Book](https://rust-mobile.github.io/book/)

## Support

- **Discord:** [discord.gg/synapsenet](https://discord.gg/synapsenet)
- **GitHub Issues:** [github.com/synapsenet/synapsenet/issues](https://github.com/synapsenet/synapsenet/issues)
