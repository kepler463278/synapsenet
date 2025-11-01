# SynapseNet Mobile - Security Guide

## Overview

Security features and best practices for SynapseNet v0.5 Mobile.

## Encryption

### Local Data
- **Algorithm:** AES-256-GCM
- **Key Derivation:** HKDF with SHA-256
- **Nonce:** Random 96-bit per encryption
- **Storage:** iOS Keychain / Android Keystore

### Key Exchange
- **Algorithm:** Kyber KEM (post-quantum)
- **Fallback:** X25519 for compatibility
- **Verification:** Ed25519 signatures

## Biometric Authentication

### iOS (Face ID / Touch ID)
```swift
// Implemented in keystore.rs
LAContext.evaluatePolicy(.deviceOwnerAuthenticationWithBiometrics)
```

### Android (BiometricPrompt)
```kotlin
// Implemented in keystore.rs
BiometricPrompt.authenticate(cryptoObject)
```

### Features
- Auto-lock after 5 minutes
- Biometric + passcode fallback
- Failed attempt limiting

## Secure Storage

### Sensitive Data
- Private keys → Keychain/Keystore
- Recovery phrase → Encrypted with biometric key
- User data → AES-GCM encrypted
- Temporary data → Secure deletion

### Data Integrity
- HMAC verification for all encrypted data
- Version checking to prevent rollback
- Checksum validation for imports

## Privacy Controls

### Air-Gap Mode
- Disables all network connections
- Local-only operation
- No P2P sync
- No notifications

### Data Sharing
- User controls what data syncs
- Opt-in for analytics
- No telemetry by default
- Export/delete all data

## Network Security

### P2P Connections
- TLS 1.3 for all connections
- Certificate pinning
- Peer verification
- Rate limiting

### API Calls
- HTTPS only
- Certificate validation
- Request signing
- Replay protection

## Threat Model

### Protected Against
- ✅ Local data theft
- ✅ Network eavesdropping
- ✅ Man-in-the-middle attacks
- ✅ Replay attacks
- ✅ Brute force attacks

### Not Protected Against
- ❌ Device compromise (rooted/jailbroken)
- ❌ Physical access with biometric bypass
- ❌ Quantum computers (for legacy data)

## Security Checklist

- [x] AES-256-GCM encryption
- [x] Kyber KEM key exchange
- [x] Keychain/Keystore integration
- [ ] Biometric authentication (ready, not implemented)
- [x] Secure deletion
- [x] Recovery phrase system
- [ ] Certificate pinning (TODO)
- [ ] Rate limiting (TODO)
- [x] Air-gap mode support

## Compliance

- GDPR compliant (data export/delete)
- No personal data collection
- Local-first architecture
- User controls all data

---

**Status:** Core security implemented, biometric auth pending
**Last Updated:** 2024-10-31
