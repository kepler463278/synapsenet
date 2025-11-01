# SynapseNet Mobile - Deployment Guide

## Prerequisites

### Development Tools
- Xcode 15+ (for iOS)
- Android Studio (for Android)
- Rust 1.70+
- Node.js 18+
- Tauri CLI

### Accounts
- Apple Developer Account ($99/year)
- Google Play Developer Account ($25 one-time)

## iOS Deployment

### 1. Configure Project
```bash
cd apps/mobile/src-tauri
```

Edit `tauri.conf.json`:
```json
{
  "bundle": {
    "identifier": "com.synapsenet.mobile",
    "iOS": {
      "minimumSystemVersion": "14.0"
    }
  }
}
```

### 2. Signing
1. Open Xcode
2. Select project → Signing & Capabilities
3. Select your team
4. Enable automatic signing

### 3. Build
```bash
npm run tauri build -- --target ios
```

### 4. TestFlight
1. Archive in Xcode
2. Upload to App Store Connect
3. Add to TestFlight
4. Invite testers

### 5. App Store Submission
1. Complete App Store Connect listing
2. Add screenshots (6.5", 5.5")
3. Write description
4. Submit for review

## Android Deployment

### 1. Configure Project
Edit `tauri.conf.json`:
```json
{
  "bundle": {
    "android": {
      "minSdkVersion": 26
    }
  }
}
```

### 2. Signing Key
```bash
keytool -genkey -v -keystore synapsenet.keystore \
  -alias synapsenet -keyalg RSA -keysize 2048 -validity 10000
```

### 3. Build
```bash
npm run tauri build -- --target android
```

### 4. Internal Testing
1. Upload APK/AAB to Play Console
2. Create internal testing track
3. Add testers
4. Distribute

### 5. Production Release
1. Complete Play Console listing
2. Add screenshots
3. Write description
4. Submit for review

## CI/CD Pipeline

### GitHub Actions

`.github/workflows/mobile.yml`:
```yaml
name: Mobile Build

on:
  push:
    branches: [main]
  pull_request:

jobs:
  ios:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      - name: Setup
        run: |
          npm install
          cargo install tauri-cli
      - name: Build iOS
        run: npm run tauri build -- --target ios
      
  android:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Setup
        run: |
          npm install
          cargo install tauri-cli
      - name: Build Android
        run: npm run tauri build -- --target android
```

## App Store Assets

### iOS
- App Icon: 1024x1024px
- Screenshots: 6.5" (1284x2778), 5.5" (1242x2208)
- Preview Video: 30s max

### Android
- App Icon: 512x512px
- Feature Graphic: 1024x500px
- Screenshots: Phone, 7" tablet, 10" tablet
- Preview Video: 30s max

## Release Checklist

### Pre-Release
- [ ] All tests passing
- [ ] Performance benchmarks met
- [ ] Security audit complete
- [ ] Accessibility tested
- [ ] Translations complete
- [ ] Privacy policy updated
- [ ] Terms of service ready

### App Store
- [ ] Screenshots prepared
- [ ] Description written
- [ ] Keywords optimized
- [ ] Support URL set
- [ ] Privacy policy URL set
- [ ] Age rating completed

### Post-Release
- [ ] Monitor crash reports
- [ ] Track user feedback
- [ ] Respond to reviews
- [ ] Plan updates
- [ ] Marketing campaign

## Version Management

### Semantic Versioning
- Major: Breaking changes
- Minor: New features
- Patch: Bug fixes

Example: `0.5.0` → `0.5.1` (bug fix) → `0.6.0` (new feature)

### Build Numbers
- iOS: Increment for each build
- Android: versionCode (integer)

## Monitoring

### Crash Reporting
- iOS: Xcode Organizer
- Android: Play Console
- Third-party: Sentry (optional)

### Analytics
- User-controlled opt-in
- No personal data
- Usage patterns only

---

**Status:** Deployment process documented
**Last Updated:** 2024-10-31
