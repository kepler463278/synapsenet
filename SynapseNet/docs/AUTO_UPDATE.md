# SynapseNet Auto-Update System

SynapseNet v0.4 includes an automatic update system powered by Tauri's built-in updater.

## How It Works

1. **Automatic Checks**: The app checks for updates every 6 hours and on startup
2. **User Notification**: When an update is available, a notification appears in the top-right corner
3. **One-Click Install**: Users can install updates with a single click
4. **Seamless Restart**: The app automatically restarts after update installation

## For Users

### Checking for Updates

Updates are checked automatically, but you can also:
- The app checks on startup
- Checks run every 6 hours in the background
- A notification appears when an update is available

### Installing Updates

When an update notification appears:

1. Click **"Update Now"** to download and install immediately
2. Click **"Later"** to dismiss the notification (it will reappear on next check)
3. View release notes by clicking **"What's new?"**

The app will restart automatically after installation.

### Manual Update Check

Currently, manual update checks are automatic. Future versions may add a manual check button in Settings.

### Troubleshooting

**Update check fails:**
- Check your internet connection
- Ensure you're not behind a restrictive firewall
- Try restarting the app

**Update installation fails:**
- Ensure you have write permissions to the app directory
- Close any other instances of SynapseNet
- Try downloading the installer manually from GitHub releases

**Update notification doesn't appear:**
- You're already on the latest version
- Check Settings > About to see your current version
- Visit https://github.com/synapsenet/synapsenet/releases for latest version

## For Developers

### Update Server Setup

SynapseNet uses GitHub Releases as the update server. The updater checks:

```
https://github.com/synapsenet/synapsenet/releases/latest/download/latest.json
```

### Creating a Release

1. **Build installers for all platforms:**
   ```bash
   ./scripts/build-installers.sh all
   ```

2. **Generate update manifests:**
   ```bash
   # For each platform, create a signature
   tauri signer sign dist/SynapseNet-0.4.0-macos.dmg
   tauri signer sign dist/SynapseNet-0.4.0-windows-x64.msi
   tauri signer sign dist/synapsenet-0.4.0-linux-amd64.deb
   ```

3. **Create latest.json:**
   ```json
   {
     "version": "0.4.0",
     "notes": "Release notes here",
     "pub_date": "2024-10-31T00:00:00Z",
     "platforms": {
       "darwin-x86_64": {
         "signature": "SIGNATURE_HERE",
         "url": "https://github.com/synapsenet/synapsenet/releases/download/v0.4.0/SynapseNet-0.4.0-macos.dmg"
       },
       "darwin-aarch64": {
         "signature": "SIGNATURE_HERE",
         "url": "https://github.com/synapsenet/synapsenet/releases/download/v0.4.0/SynapseNet-0.4.0-macos.dmg"
       },
       "linux-x86_64": {
         "signature": "SIGNATURE_HERE",
         "url": "https://github.com/synapsenet/synapsenet/releases/download/v0.4.0/synapsenet-0.4.0-linux-amd64.deb"
       },
       "windows-x86_64": {
         "signature": "SIGNATURE_HERE",
         "url": "https://github.com/synapsenet/synapsenet/releases/download/v0.4.0/SynapseNet-0.4.0-windows-x64.msi"
       }
     }
   }
   ```

4. **Create GitHub Release:**
   ```bash
   gh release create v0.4.0 \
     --title "SynapseNet v0.4.0" \
     --notes-file RELEASE_NOTES.md \
     dist/SynapseNet-0.4.0-macos.dmg \
     dist/SynapseNet-0.4.0-windows-x64.msi \
     dist/synapsenet-0.4.0-linux-amd64.deb \
     dist/SynapseNet-0.4.0-linux-x86_64.AppImage \
     dist/latest.json \
     dist/checksums.txt
   ```

### Signing Keys

Generate signing keys for the updater:

```bash
# Generate a new keypair
tauri signer generate -w ~/.tauri/synapsenet.key

# This creates:
# - Private key: ~/.tauri/synapsenet.key
# - Public key: printed to stdout
```

**Important:** 
- Keep the private key secure and never commit it
- Add the public key to `tauri.conf.json` under `updater.pubkey`
- Use the private key to sign all releases

### Configuration

In `tauri.conf.json`:

```json
{
  "updater": {
    "active": true,
    "endpoints": [
      "https://github.com/synapsenet/synapsenet/releases/latest/download/latest.json"
    ],
    "dialog": true,
    "pubkey": "YOUR_PUBLIC_KEY_HERE"
  }
}
```

**Options:**
- `active`: Enable/disable updater
- `endpoints`: Array of update server URLs (tries in order)
- `dialog`: Show native dialog for updates (true) or use custom UI (false)
- `pubkey`: Public key for signature verification

### Custom Update Server

To use a custom update server instead of GitHub:

1. **Host latest.json** at a public URL
2. **Update tauri.conf.json:**
   ```json
   {
     "updater": {
       "endpoints": [
         "https://updates.synapsenet.io/{{target}}/{{current_version}}"
       ]
     }
   }
   ```

3. **Implement server endpoint** that returns latest.json

**URL Variables:**
- `{{target}}`: Platform target (e.g., `darwin-x86_64`)
- `{{current_version}}`: Current app version
- `{{arch}}`: Architecture (e.g., `x86_64`)

### Testing Updates

**Development Mode:**
Updates are disabled in development mode to prevent accidental updates during testing.

**Testing in Production Build:**

1. Build a production version with an old version number:
   ```json
   // tauri.conf.json
   {
     "version": "0.3.0"
   }
   ```

2. Build and run:
   ```bash
   cargo tauri build
   ./target/release/synapsenet
   ```

3. Create a release with version 0.4.0

4. The app should detect and offer the update

### CI/CD Integration

Example GitHub Actions workflow:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  create-release:
    runs-on: ubuntu-latest
    outputs:
      release_id: ${{ steps.create-release.outputs.result }}
    steps:
      - uses: actions/checkout@v3
      - name: Create release
        id: create-release
        uses: actions/github-script@v6
        with:
          script: |
            const { data } = await github.rest.repos.createRelease({
              owner: context.repo.owner,
              repo: context.repo.repo,
              tag_name: `${context.ref.replace('refs/tags/', '')}`,
              name: `SynapseNet ${context.ref.replace('refs/tags/', '')}`,
              body: 'Release notes here',
              draft: true,
              prerelease: false
            })
            return data.id

  build-tauri:
    needs: create-release
    strategy:
      matrix:
        platform: [macos-latest, ubuntu-latest, windows-latest]
    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: 18
      - uses: dtolnay/rust-toolchain@stable
      
      - name: Install dependencies (Ubuntu)
        if: matrix.platform == 'ubuntu-latest'
        run: |
          sudo apt update
          sudo apt install -y libgtk-3-dev libwebkit2gtk-4.0-dev \
            libssl-dev libayatana-appindicator3-dev librsvg2-dev
      
      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          TAURI_PRIVATE_KEY: ${{ secrets.TAURI_PRIVATE_KEY }}
        with:
          releaseId: ${{ needs.create-release.outputs.release_id }}
```

### Security Considerations

1. **Signature Verification**: All updates are cryptographically signed and verified
2. **HTTPS Only**: Update checks and downloads use HTTPS
3. **Public Key Pinning**: The public key is embedded in the app
4. **No Automatic Installation**: Users must approve updates (unless configured otherwise)

### Rollback Strategy

If an update causes issues:

1. **Immediate**: Users can reinstall the previous version manually
2. **Automated**: Implement version pinning in update server
3. **Emergency**: Remove the problematic release from GitHub

### Monitoring

Track update adoption:

```typescript
// In your analytics
invoke('check_for_updates').then(info => {
  analytics.track('update_check', {
    current_version: info.current_version,
    latest_version: info.version,
    update_available: info.available
  });
});
```

## Best Practices

1. **Test thoroughly** before releasing updates
2. **Write clear release notes** explaining changes
3. **Use semantic versioning** (major.minor.patch)
4. **Monitor update adoption** rates
5. **Keep update sizes small** when possible
6. **Provide manual download** option as fallback
7. **Test on all platforms** before release
8. **Sign all releases** with the same key
9. **Keep private key secure** and backed up
10. **Document breaking changes** clearly

## Troubleshooting

### "Update signature verification failed"

- Ensure you're using the correct private key to sign
- Verify the public key in tauri.conf.json matches
- Check that the signature in latest.json is correct

### "Update download failed"

- Verify the URL in latest.json is accessible
- Check file permissions on the server
- Ensure CORS headers are set if using custom server

### "Update installation failed"

- Check app has write permissions
- Ensure no other instances are running
- Verify the downloaded file is not corrupted

## Future Enhancements

- [ ] Delta updates (only download changes)
- [ ] Background downloads
- [ ] Scheduled update windows
- [ ] Rollback on failure
- [ ] Update channels (stable, beta, nightly)
- [ ] Bandwidth throttling
- [ ] Pause/resume downloads

## Resources

- [Tauri Updater Documentation](https://tauri.app/v1/guides/distribution/updater)
- [Tauri Signer Tool](https://tauri.app/v1/api/cli#signer)
- [GitHub Releases API](https://docs.github.com/en/rest/releases)

## Support

For update-related issues:
- Check the [Troubleshooting](#troubleshooting) section
- Open an issue on GitHub
- Join our Discord community
