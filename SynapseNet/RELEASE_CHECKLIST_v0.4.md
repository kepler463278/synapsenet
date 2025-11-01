# SynapseNet v0.4.0 Release Checklist

**Release Date:** TBD  
**Version:** 0.4.0 "Emergence"  
**Status:** 🚧 IN PREPARATION

---

## Pre-Release Checklist

### Code Quality ✅
- [x] All tests passing
- [x] No compiler warnings
- [x] Code formatted with `cargo fmt`
- [x] Linting passed with `cargo clippy`
- [x] Documentation complete
- [x] CHANGELOG.md updated

### Testing ✅
- [x] Unit tests passing
- [x] Integration tests passing
- [x] Performance benchmarks run
- [ ] Manual testing on all platforms
  - [ ] macOS (Intel & Apple Silicon)
  - [ ] Windows 10/11
  - [ ] Linux (Ubuntu, Fedora, Arch)
- [ ] UI/UX testing complete
- [ ] Security audit passed

### Documentation ✅
- [x] README.md updated
- [x] API documentation complete
- [x] User guide written
- [x] Migration guide from v0.3
- [x] Release notes drafted
- [x] CHANGELOG.md updated

### Build & Distribution 🚧
- [ ] Version numbers updated
  - [ ] Cargo.toml (root)
  - [ ] All crate Cargo.tomls
  - [ ] package.json
  - [ ] tauri.conf.json
- [ ] Release builds created
  - [ ] macOS DMG
  - [ ] Windows MSI
  - [ ] Linux DEB
  - [ ] Linux AppImage
- [ ] Installers tested
- [ ] Code signing complete (macOS, Windows)
- [ ] Checksums generated

### Release Assets 🚧
- [ ] GitHub release created
- [ ] Release notes published
- [ ] Installers uploaded
- [ ] Checksums uploaded
- [ ] Source code tagged
- [ ] Docker images built (if applicable)

---

## Release Steps

### 1. Version Bump

```bash
# Update version in all Cargo.toml files
./scripts/bump-version.sh 0.4.0

# Update package.json
cd crates/tauri-app
npm version 0.4.0

# Update tauri.conf.json
# (manually edit version field)
```

### 2. Final Testing

```bash
# Run all tests
cargo test --all

# Run benchmarks
cargo bench

# Build release
cargo build --release

# Test installers
./scripts/build-installers.sh all
```

### 3. Create Release Builds

```bash
# Clean previous builds
./scripts/build-installers.sh clean

# Build for all platforms
./scripts/build-installers.sh all

# Verify builds
ls -lh dist/
```

### 4. Sign & Notarize (macOS)

```bash
# Set environment variables
export APPLE_DEVELOPER_ID="Developer ID Application: Your Name (TEAM_ID)"
export APPLE_ID="your@email.com"
export APPLE_PASSWORD="app-specific-password"
export APPLE_TEAM_ID="YOUR_TEAM_ID"

# Build with signing
./scripts/build-installers.sh macos
```

### 5. Generate Checksums

```bash
cd dist
shasum -a 256 * > checksums.txt
cat checksums.txt
```

### 6. Create GitHub Release

```bash
# Create git tag
git tag -a v0.4.0 -m "Release v0.4.0: Emergence"
git push origin v0.4.0

# Create GitHub release
gh release create v0.4.0 \
  --title "SynapseNet v0.4.0: Emergence" \
  --notes-file RELEASE_NOTES_v0.4.md \
  dist/SynapseNet-0.4.0-macos.dmg \
  dist/SynapseNet-0.4.0-windows-x64.msi \
  dist/synapsenet-0.4.0-linux-amd64.deb \
  dist/SynapseNet-0.4.0-linux-x86_64.AppImage \
  dist/checksums.txt
```

### 7. Update Documentation Sites

- [ ] Update project website
- [ ] Update documentation site
- [ ] Update API reference
- [ ] Update examples

### 8. Announce Release

- [ ] Blog post published
- [ ] Twitter/X announcement
- [ ] Reddit post (r/rust, r/programming)
- [ ] Hacker News submission
- [ ] Discord announcement
- [ ] Email newsletter

---

## Post-Release Checklist

### Monitoring 📊
- [ ] Monitor GitHub issues for bug reports
- [ ] Check download statistics
- [ ] Monitor error reporting (if implemented)
- [ ] Review user feedback

### Support 💬
- [ ] Respond to GitHub issues
- [ ] Answer questions on Discord
- [ ] Update FAQ based on common questions
- [ ] Create troubleshooting guides as needed

### Planning 🗓️
- [ ] Create v0.4.1 milestone for bug fixes
- [ ] Start planning v0.5.0 features
- [ ] Review and prioritize feature requests
- [ ] Update roadmap

---

## Version Numbers to Update

### Rust Crates
- `Cargo.toml` (workspace root)
- `crates/core/Cargo.toml`
- `crates/p2p/Cargo.toml`
- `crates/storage/Cargo.toml`
- `crates/ai/Cargo.toml`
- `crates/economy/Cargo.toml`
- `crates/governance/Cargo.toml`
- `crates/api/Cargo.toml`
- `crates/cli/Cargo.toml`
- `crates/tauri-app/Cargo.toml`

### Frontend
- `crates/tauri-app/package.json`
- `crates/tauri-app/tauri.conf.json`

### Documentation
- `README.md`
- `CHANGELOG.md`
- `docs/QUICKSTART.md`

---

## Known Issues

Document any known issues that won't be fixed in this release:

- None currently identified

---

## Breaking Changes

Document any breaking changes from v0.3:

1. **Configuration Format**: New TOML structure (migration automatic)
2. **API Endpoints**: v1 deprecated, use v2 (v1 still works)
3. **Storage Schema**: Automatic migration on first run
4. **CLI Commands**: Some flags renamed (see migration guide)

---

## Migration Guide

See `docs/MIGRATION_v0.3_to_v0.4.md` for detailed migration instructions.

**Quick Summary:**
1. Backup your data directory
2. Install v0.4
3. Run once to trigger automatic migration
4. Update any custom scripts to use new CLI flags
5. Update API calls to use v2 endpoints

---

## Rollback Plan

If critical issues are discovered:

1. **Immediate**: Remove release from GitHub
2. **Communication**: Post issue on GitHub and Discord
3. **Fix**: Create hotfix branch
4. **Release**: v0.4.1 with fix
5. **Timeline**: Within 24-48 hours

---

## Success Criteria

Release is considered successful when:

- [ ] 100+ downloads in first week
- [ ] < 5 critical bugs reported
- [ ] Positive community feedback
- [ ] No security vulnerabilities reported
- [ ] Stable performance metrics

---

## Contact

**Release Manager:** TBD  
**Technical Lead:** TBD  
**Community Manager:** TBD

---

## Notes

Add any additional notes or reminders here:

- Remember to update auto-updater configuration
- Prepare demo video for social media
- Schedule AMA session on Discord
- Consider writing technical blog post about new features

---

**Last Updated:** 2024-10-31  
**Next Review:** Before release
