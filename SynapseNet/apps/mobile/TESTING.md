# SynapseNet Mobile - Testing Guide

## Overview

This document outlines the testing strategy for SynapseNet v0.5 Mobile application.

## Test Structure

```
apps/mobile/
├── src-tauri/
│   └── src/
│       ├── *.rs (with #[cfg(test)] modules)
│       └── tests/ (integration tests)
└── src/
    └── __tests__/ (React component tests)
```

## Unit Tests (Rust)

### Existing Tests

**PoE Module (`poe.rs`):**
- ✅ `test_cosine_similarity` - Vector similarity calculation
- ✅ `test_novelty_first_grain` - First grain novelty
- ✅ `test_coherence` - Embedding coherence
- ✅ `test_poe_score_calculation` - Full PoE calculation

**Accessibility Module (`accessibility.rs`):**
- ✅ `test_grain_accessibility_label` - Grain label generation
- ✅ `test_peer_accessibility_label` - Peer label generation
- ✅ `test_reward_accessibility_label` - Reward label generation

**File Import Module (`file_import.rs`):**
- ✅ `test_mime_type_detection` - File type detection
- ✅ `test_text_chunking` - Text splitting algorithm

**Voice Module (`voice.rs`):**
- ✅ `test_voice_config` - Configuration validation
- ✅ `test_voice_availability` - Availability check

**Notifications Module (`notifications.rs`):**
- ✅ `test_notification_creation` - Notification factory methods
- ✅ `test_notification_settings_default` - Default settings

### Running Tests

```bash
cd apps/mobile/src-tauri
cargo test
```

### Test Coverage

Current coverage: ~40% (core algorithms tested)

**Covered:**
- PoE calculations
- Accessibility helpers
- File parsing
- Configuration validation

**Not Covered (TODO):**
- Platform-specific code (iOS/Android)
- Network operations
- State management
- Error handling paths

## Integration Tests

### Test Scenarios

**1. End-to-End Grain Flow:**
```
User Input → Embedding → Storage → Query → Results
```

**2. P2P Sync:**
```
Local Grain → Network → Peer Discovery → Sync → Verification
```

**3. Background Tasks:**
```
App Background → Scheduler → Task Execution → Completion
```

**4. Recovery Process:**
```
Recovery Phrase → Key Derivation → Data Decryption → Restore
```

### Running Integration Tests

```bash
cd apps/mobile/src-tauri
cargo test --test '*' -- --test-threads=1
```

## UI Tests (React)

### Test Framework

- **Jest** - Test runner
- **React Testing Library** - Component testing
- **@testing-library/react-hooks** - Hook testing

### Test Structure

```typescript
// Example: HomeScreen.test.tsx
import { render, screen } from '@testing-library/react';
import HomeScreen from '../screens/HomeScreen';

describe('HomeScreen', () => {
  it('renders stats correctly', () => {
    render(<HomeScreen />);
    expect(screen.getByText('Grains')).toBeInTheDocument();
  });
});
```

### Running UI Tests

```bash
cd apps/mobile
npm test
```

## Performance Tests

### Metrics to Track

**App Launch:**
- Target: < 2 seconds
- Measure: Time to interactive

**Embedding Generation:**
- Target: < 500ms per grain
- Measure: Average processing time

**Battery Usage:**
- Target: < 5% per hour (idle)
- Measure: Battery drain rate

**Memory Usage:**
- Target: < 150MB
- Measure: Peak memory consumption

**Network Performance:**
- Target: < 1s for sync
- Measure: Sync completion time

### Performance Testing Tools

**iOS:**
- Instruments (Xcode)
- Time Profiler
- Energy Log

**Android:**
- Android Profiler
- Battery Historian
- Systrace

### Running Performance Tests

```bash
# iOS
xcodebuild test -scheme SynapseNet -destination 'platform=iOS Simulator,name=iPhone 14'

# Android
./gradlew connectedAndroidTest
```

## Accessibility Testing

### Manual Testing Checklist

**iOS:**
- [ ] Enable VoiceOver
- [ ] Test all screens with VoiceOver
- [ ] Test with Dynamic Type (largest size)
- [ ] Test with Reduce Motion
- [ ] Test with High Contrast

**Android:**
- [ ] Enable TalkBack
- [ ] Test all screens with TalkBack
- [ ] Test with Font Size (largest)
- [ ] Test with Remove Animations
- [ ] Test with High Contrast Text

### Automated Accessibility Tests

```typescript
// Example: Accessibility test
import { render } from '@testing-library/react';
import { axe, toHaveNoViolations } from 'jest-axe';

expect.extend(toHaveNoViolations);

it('should not have accessibility violations', async () => {
  const { container } = render(<HomeScreen />);
  const results = await axe(container);
  expect(results).toHaveNoViolations();
});
```

## Test on Real Devices

### Device Matrix

**iOS:**
- iPhone 14 Pro (iOS 17)
- iPhone 12 (iOS 16)
- iPhone SE (iOS 15)
- iPad Air (iOS 17)

**Android:**
- Pixel 7 (Android 14)
- Samsung Galaxy S22 (Android 13)
- OnePlus 9 (Android 12)
- Budget device (Android 11)

### Testing Checklist

- [ ] App installation
- [ ] First launch experience
- [ ] All screens functional
- [ ] Voice input works
- [ ] File import works
- [ ] Notifications appear
- [ ] Accessibility features work
- [ ] Performance acceptable
- [ ] Battery drain reasonable
- [ ] Network sync works

## Continuous Integration

### GitHub Actions Workflow

```yaml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Rust
        uses: actions-rs/toolchain@v1
      - name: Run tests
        run: cargo test
      - name: Run UI tests
        run: npm test
```

## Test Reports

### Coverage Reports

Generate coverage with:
```bash
cargo tarpaulin --out Html
```

View report at: `target/tarpaulin/index.html`

### Test Results

- Unit tests: See `cargo test` output
- Integration tests: See CI logs
- UI tests: See Jest report
- Performance tests: See profiler results

## Best Practices

1. **Write tests first** for critical algorithms
2. **Mock external dependencies** (network, filesystem)
3. **Test edge cases** and error conditions
4. **Keep tests fast** (< 1s per test)
5. **Use descriptive test names**
6. **Test one thing per test**
7. **Clean up after tests** (no side effects)

## Known Issues

- Platform-specific code not fully testable in CI
- Some tests require real devices
- Performance tests need manual execution
- Accessibility tests partially automated

## Future Improvements

- [ ] Increase test coverage to 80%
- [ ] Add snapshot testing for UI
- [ ] Implement visual regression testing
- [ ] Add load testing for P2P
- [ ] Automate device testing
- [ ] Add mutation testing
- [ ] Implement property-based testing

---

**Last Updated:** 2024-10-31
**Test Coverage:** ~40%
**Status:** Basic tests implemented, comprehensive suite pending
