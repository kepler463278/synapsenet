# Requirements Document - SynapseNet v0.5 Mobile

## Introduction

SynapseNet v0.5 "Mobile Emergence" brings the decentralized semantic memory network to mobile devices (iOS and Android). This release enables users to carry their personal knowledge network in their pocket, with full offline AI capabilities, encrypted memory capsules, and P2P networking optimized for mobile constraints.

## Glossary

- **Mobile App**: Native iOS and Android application built with Tauri Mobile
- **Memory Capsule**: Encrypted local storage container for user's knowledge grains
- **CoreML**: Apple's machine learning framework for iOS devices
- **NNAPI**: Android Neural Networks API for hardware acceleration
- **Keychain**: iOS secure storage for cryptographic keys
- **Android Keystore**: Android secure storage for cryptographic keys
- **Recovery Phrase**: 12-word mnemonic for backup and restoration
- **Foreground Service**: Android service that keeps app running in background
- **BackgroundTasks**: iOS framework for scheduled background work
- **WebRTC**: Web Real-Time Communication protocol for P2P connections
- **Circuit Relay**: libp2p relay mechanism for NAT traversal
- **Air-gap Mode**: Offline-only mode with no network connectivity

## Requirements

### Requirement 1: Mobile Application Foundation

**User Story:** As a mobile user, I want a native app on my phone, so that I can access my knowledge network anywhere.

#### Acceptance Criteria

1. WHEN the user installs the app, THE Mobile App SHALL launch successfully on iOS 14+ devices
2. WHEN the user installs the app, THE Mobile App SHALL launch successfully on Android 8+ devices
3. WHEN the app launches, THE Mobile App SHALL initialize the Rust core within 3 seconds
4. WHEN the app is in foreground, THE Mobile App SHALL maintain full functionality
5. WHEN the app moves to background, THE Mobile App SHALL queue operations for later execution

### Requirement 2: On-Device AI Inference

**User Story:** As a mobile user, I want AI to run locally on my device, so that my data stays private and works offline.

#### Acceptance Criteria

1. WHEN the app starts on iOS, THE Mobile App SHALL detect and use CoreML if available
2. WHEN the app starts on Android, THE Mobile App SHALL detect and use NNAPI if available
3. IF hardware acceleration is unavailable, THEN THE Mobile App SHALL fallback to CPU inference
4. WHEN generating embeddings, THE Mobile App SHALL complete within 500ms per text on modern devices
5. WHEN the device is on battery, THE Mobile App SHALL limit GPU usage to preserve battery life

### Requirement 3: Encrypted Memory Capsules

**User Story:** As a security-conscious user, I want my knowledge encrypted on device, so that only I can access it.

#### Acceptance Criteria

1. WHEN a grain is stored, THE Mobile App SHALL encrypt it using AES-256-GCM
2. WHEN the app initializes, THE Mobile App SHALL retrieve encryption keys from Keychain (iOS) or Android Keystore
3. WHEN the user creates a new capsule, THE Mobile App SHALL generate a 12-word recovery phrase
4. WHEN the user enters recovery phrase, THE Mobile App SHALL restore the encryption key using Kyber KEM
5. WHEN the user enables air-gap mode, THE Mobile App SHALL disable all network operations

### Requirement 4: Mobile-Optimized P2P Networking

**User Story:** As a mobile user, I want to connect with peers efficiently, so that my battery and data aren't wasted.

#### Acceptance Criteria

1. WHEN connecting to peers, THE Mobile App SHALL use WebRTC for NAT traversal
2. WHEN direct connection fails, THE Mobile App SHALL use Circuit Relay v2
3. WHEN the app is in background, THE Mobile App SHALL batch network operations
4. WHEN on cellular data, THE Mobile App SHALL limit bandwidth usage to user-configured threshold
5. WHEN WiFi is available, THE Mobile App SHALL sync queued operations

### Requirement 5: Mobile UI/UX

**User Story:** As a mobile user, I want an intuitive touch interface, so that I can easily manage my knowledge.

#### Acceptance Criteria

1. WHEN the user opens the app, THE Mobile App SHALL display Home screen with node status
2. WHEN the user taps Add, THE Mobile App SHALL show text input with voice-to-text option
3. WHEN the user taps Query, THE Mobile App SHALL show search interface with results
4. WHEN the user taps Peers, THE Mobile App SHALL display connected peers and reputation
5. WHEN the user taps Wallet, THE Mobile App SHALL show NGT balance and PoE rewards

### Requirement 6: Offline-First Operation

**User Story:** As a mobile user, I want full functionality offline, so that I'm not dependent on connectivity.

#### Acceptance Criteria

1. WHEN offline, THE Mobile App SHALL allow adding new grains
2. WHEN offline, THE Mobile App SHALL allow searching local grains
3. WHEN offline, THE Mobile App SHALL queue P2P operations
4. WHEN connectivity returns, THE Mobile App SHALL sync queued operations automatically
5. WHEN in air-gap mode, THE Mobile App SHALL never attempt network operations

### Requirement 7: Battery and Performance Optimization

**User Story:** As a mobile user, I want the app to be efficient, so that it doesn't drain my battery.

#### Acceptance Criteria

1. WHEN on battery power, THE Mobile App SHALL limit background processing to 5% CPU usage
2. WHEN charging, THE Mobile App SHALL allow full GPU/NPU acceleration
3. WHEN idle for 5 minutes, THE Mobile App SHALL reduce background activity
4. WHEN memory is low, THE Mobile App SHALL unload unused models
5. WHEN the user configures power mode, THE Mobile App SHALL respect the setting

### Requirement 8: Cross-Platform Sync

**User Story:** As a user with multiple devices, I want my knowledge synced, so that I have access everywhere.

#### Acceptance Criteria

1. WHEN connected to network, THE Mobile App SHALL sync grains with desktop app
2. WHEN a grain is added on mobile, THE Mobile App SHALL propagate it to P2P network
3. WHEN a grain is received from network, THE Mobile App SHALL store it in local capsule
4. WHEN conflicts occur, THE Mobile App SHALL use timestamp-based resolution
5. WHEN the user exports data, THE Mobile App SHALL create encrypted backup file

### Requirement 9: Privacy and Security

**User Story:** As a privacy-conscious user, I want control over my data, so that I decide what's shared.

#### Acceptance Criteria

1. WHEN sharing grains, THE Mobile App SHALL only transmit embeddings and signatures
2. WHEN the user disables sharing, THE Mobile App SHALL keep all data local
3. WHEN accessing secure storage, THE Mobile App SHALL require biometric authentication
4. WHEN the user views privacy settings, THE Mobile App SHALL show what data is shared
5. WHEN the user deletes data, THE Mobile App SHALL securely wipe it from storage

### Requirement 10: App Store Compliance

**User Story:** As a developer, I want the app to meet store requirements, so that it can be published.

#### Acceptance Criteria

1. WHEN submitted to App Store, THE Mobile App SHALL comply with Apple's privacy guidelines
2. WHEN submitted to Play Store, THE Mobile App SHALL comply with Google's privacy guidelines
3. WHEN requesting permissions, THE Mobile App SHALL provide clear explanations
4. WHEN collecting data, THE Mobile App SHALL have user consent
5. WHEN the user reviews permissions, THE Mobile App SHALL show minimal required permissions

### Requirement 11: PoE Rewards on Mobile

**User Story:** As a contributor, I want to earn rewards on mobile, so that my contributions are recognized.

#### Acceptance Criteria

1. WHEN a grain is added, THE Mobile App SHALL calculate local PoE score
2. WHEN connected to network, THE Mobile App SHALL sync PoE scores
3. WHEN rewards are earned, THE Mobile App SHALL update NGT balance
4. WHEN the user views wallet, THE Mobile App SHALL show reward history
5. WHEN the user exports rewards, THE Mobile App SHALL generate proof of contribution

### Requirement 12: Multi-Model Support

**User Story:** As a power user, I want to choose AI models, so that I can balance quality and performance.

#### Acceptance Criteria

1. WHEN the app starts, THE Mobile App SHALL load default model (all-MiniLM-L6-v2)
2. WHEN the user selects a model, THE Mobile App SHALL download it if not present
3. WHEN switching models, THE Mobile App SHALL complete within 2 seconds
4. WHEN storage is low, THE Mobile App SHALL prompt to delete unused models
5. WHEN a model is incompatible, THE Mobile App SHALL show clear error message

### Requirement 13: Voice Input

**User Story:** As a mobile user, I want voice input, so that I can add knowledge hands-free.

#### Acceptance Criteria

1. WHEN the user taps microphone, THE Mobile App SHALL start voice recording
2. WHEN recording completes, THE Mobile App SHALL transcribe using device speech recognition
3. WHEN transcription completes, THE Mobile App SHALL populate text field
4. WHEN transcription fails, THE Mobile App SHALL show error and allow retry
5. WHEN the user configures language, THE Mobile App SHALL use selected language for transcription

### Requirement 14: File Import

**User Story:** As a mobile user, I want to import files, so that I can add existing documents.

#### Acceptance Criteria

1. WHEN the user taps import, THE Mobile App SHALL show file picker
2. WHEN a file is selected, THE Mobile App SHALL parse supported formats (txt, md, pdf)
3. WHEN parsing completes, THE Mobile App SHALL create grains from content
4. WHEN import is large, THE Mobile App SHALL show progress indicator
5. WHEN import fails, THE Mobile App SHALL show specific error message

### Requirement 15: Notifications

**User Story:** As a mobile user, I want notifications, so that I'm informed of important events.

#### Acceptance Criteria

1. WHEN a peer responds to query, THE Mobile App SHALL show notification
2. WHEN sync completes, THE Mobile App SHALL show notification (if enabled)
3. WHEN rewards are earned, THE Mobile App SHALL show notification
4. WHEN the user configures notifications, THE Mobile App SHALL respect preferences
5. WHEN the user taps notification, THE Mobile App SHALL navigate to relevant screen

### Requirement 16: Accessibility

**User Story:** As a user with accessibility needs, I want the app to be accessible, so that I can use it effectively.

#### Acceptance Criteria

1. WHEN using VoiceOver (iOS), THE Mobile App SHALL provide clear labels
2. WHEN using TalkBack (Android), THE Mobile App SHALL provide clear labels
3. WHEN the user increases text size, THE Mobile App SHALL scale UI appropriately
4. WHEN the user enables high contrast, THE Mobile App SHALL adjust colors
5. WHEN the user uses switch control, THE Mobile App SHALL be fully navigable

### Requirement 17: Localization

**User Story:** As an international user, I want the app in my language, so that I can understand it.

#### Acceptance Criteria

1. WHEN the app starts, THE Mobile App SHALL detect device language
2. WHEN supported language is detected, THE Mobile App SHALL use that language
3. WHEN unsupported language is detected, THE Mobile App SHALL fallback to English
4. WHEN the user changes language, THE Mobile App SHALL update UI immediately
5. WHEN displaying dates/times, THE Mobile App SHALL use locale-appropriate format

### Requirement 18: Testing and Quality

**User Story:** As a developer, I want comprehensive testing, so that the app is reliable.

#### Acceptance Criteria

1. WHEN code is committed, THE Mobile App SHALL pass all unit tests
2. WHEN building release, THE Mobile App SHALL pass integration tests on real devices
3. WHEN testing UI, THE Mobile App SHALL pass automated UI tests
4. WHEN profiling, THE Mobile App SHALL meet performance benchmarks
5. WHEN analyzing, THE Mobile App SHALL have no memory leaks

---

## Non-Functional Requirements

### Performance
- App launch time: < 3 seconds
- Embedding generation: < 500ms per text
- Search latency: < 100ms for local queries
- UI responsiveness: 60 FPS on modern devices

### Security
- AES-256-GCM encryption for local storage
- Kyber KEM for key derivation
- Dilithium5 signatures for grains
- Biometric authentication for sensitive operations

### Compatibility
- iOS: 14.0+
- Android: 8.0+ (API level 26+)
- Devices: iPhone 8+, Android devices with 2GB+ RAM

### Storage
- App size: < 50MB (excluding models)
- Model size: 25-35MB per model
- Local storage: Configurable, default 1GB

### Battery
- Background CPU: < 5% average
- Foreground CPU: < 20% average
- Network: Batch operations to minimize radio usage

### Privacy
- No raw data transmitted
- Only embeddings and signatures shared
- User consent for all data sharing
- Compliance with GDPR, CCPA

---

## Success Criteria

The v0.5 Mobile release is successful when:

1. App is published on both App Store and Play Store
2. 1000+ downloads in first month
3. < 10 critical bugs reported
4. Average rating > 4.0 stars
5. Battery usage < 5% per day (background)
6. Positive user feedback on privacy features

---

**Version:** 0.5.0  
**Status:** Draft  
**Last Updated:** 2024-10-31
