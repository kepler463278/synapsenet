# SynapseNet v1.0 - Installation Guide

Complete installation instructions for all platforms.

---

## Table of Contents

1. [Windows Installation](#windows-installation)
2. [macOS Installation](#macos-installation)
3. [Linux Installation](#linux-installation)
4. [First Run](#first-run)
5. [Troubleshooting](#troubleshooting)
6. [Uninstallation](#uninstallation)

---

## Windows Installation

### System Requirements
- Windows 10 or Windows 11
- 2GB RAM (4GB recommended)
- 500MB disk space
- Internet connection

### Installation Steps

1. **Download the installer**
   - Go to https://synapsenet.org/download
   - Click "Download for Windows"
   - Save `SynapseNet-1.0.0-windows.exe`

2. **Run the installer**
   - Double-click the downloaded file
   - If Windows SmartScreen appears, click "More info" → "Run anyway"
   - Click "Next" in the installation wizard

3. **Choose installation location**
   - Default: `C:\Program Files\SynapseNet`
   - Or choose custom location
   - Click "Next"

4. **Select components**
   - Desktop shortcut (recommended)
   - Start menu entry (recommended)
   - Click "Install"

5. **Complete installation**
   - Wait for installation to finish
   - Click "Finish"
   - Check "Launch SynapseNet" to start immediately

### Post-Installation

**Desktop shortcut:**
- Double-click "SynapseNet" icon on desktop

**Start menu:**
- Press Windows key
- Type "SynapseNet"
- Click to launch

---

## macOS Installation

### System Requirements
- macOS 10.15 (Catalina) or later
- 2GB RAM (4GB recommended)
- 500MB disk space
- Internet connection

### Installation Steps

1. **Download the DMG**
   - Go to https://synapsenet.org/download
   - Click "Download for macOS"
   - Save `SynapseNet-1.0.0-macos.dmg`

2. **Open the DMG**
   - Double-click the downloaded DMG file
   - A window will open showing SynapseNet icon

3. **Install the app**
   - Drag the SynapseNet icon to the Applications folder
   - Wait for copy to complete
   - Eject the DMG (right-click → Eject)

4. **First launch**
   - Open Applications folder
   - Double-click SynapseNet
   - If you see "unidentified developer" warning:
     - Click "Cancel"
     - Go to System Preferences → Security & Privacy
     - Click "Open Anyway"
     - Click "Open" in the confirmation dialog

5. **Grant permissions (if asked)**
   - Network access: Allow
   - Notifications: Allow (optional)

### Post-Installation

**Launch from Applications:**
- Open Finder
- Go to Applications
- Double-click SynapseNet

**Launch from Spotlight:**
- Press Cmd+Space
- Type "SynapseNet"
- Press Enter

---

## Linux Installation

### System Requirements
- Linux distribution with GTK 3.0+
- 2GB RAM (4GB recommended)
- 500MB disk space
- Internet connection

### Supported Distributions
- Ubuntu 20.04+
- Debian 11+
- Fedora 35+
- Arch Linux
- Most other modern distributions

### Installation Steps

1. **Download the AppImage**
   ```bash
   cd ~/Downloads
   wget https://github.com/synapsenet/synapsenet/releases/download/v1.0.0/SynapseNet-1.0.0-linux.AppImage
   ```

2. **Make it executable**
   ```bash
   chmod +x SynapseNet-1.0.0-linux.AppImage
   ```

3. **Run the application**
   ```bash
   ./SynapseNet-1.0.0-linux.AppImage
   ```

### Optional: Desktop Integration

**Create desktop entry:**
```bash
cat > ~/.local/share/applications/synapsenet.desktop << EOF
[Desktop Entry]
Name=SynapseNet
Exec=$HOME/Downloads/SynapseNet-1.0.0-linux.AppImage
Icon=synapsenet
Type=Application
Categories=Network;P2P;
EOF
```

**Move to system location:**
```bash
sudo mv ~/Downloads/SynapseNet-1.0.0-linux.AppImage /opt/synapsenet/
sudo ln -s /opt/synapsenet/SynapseNet-1.0.0-linux.AppImage /usr/local/bin/synapsenet
```

**Launch from terminal:**
```bash
synapsenet
```

### Distribution-Specific Notes

**Ubuntu/Debian:**
```bash
# Install dependencies if needed
sudo apt install libgtk-3-0 libwebkit2gtk-4.0-37
```

**Fedora:**
```bash
# Install dependencies if needed
sudo dnf install gtk3 webkit2gtk3
```

**Arch Linux:**
```bash
# Install dependencies if needed
sudo pacman -S gtk3 webkit2gtk
```

---

## First Run

### Initial Setup

1. **Launch SynapseNet**
   - Use method appropriate for your platform

2. **Welcome screen**
   - Read the brief introduction
   - Click "Get Started"

3. **Start your node**
   - Click the "Start Node" button
   - Wait for connection (30-60 seconds)
   - Watch peer count increase

4. **Explore the interface**
   - Home: Node status and statistics
   - Knowledge: Search the network
   - Add: Contribute your knowledge
   - Rewards: Track your NGT earnings
   - Settings: Configuration and Genesis Manifest

### Add Your First Grain

1. Go to the "Add" tab
2. Enter some knowledge or insight
3. Click "Add to Network"
4. See your first NGT reward!

### Search the Network

1. Go to the "Knowledge" tab
2. Enter a search query
3. View results from local and network
4. See confidence scores and sources

---

## Troubleshooting

### Windows Issues

**"Windows protected your PC" message:**
- Click "More info"
- Click "Run anyway"
- This is normal for new applications

**Application won't start:**
- Check Windows version (must be 10 or 11)
- Ensure you have .NET Framework installed
- Try running as administrator

**Firewall blocking:**
- Go to Windows Defender Firewall
- Allow SynapseNet through firewall
- Ports needed: 9900-9910

### macOS Issues

**"Cannot open because developer cannot be verified":**
- Go to System Preferences → Security & Privacy
- Click "Open Anyway"
- This is normal for apps outside App Store

**Application crashes on launch:**
- Check macOS version (must be 10.15+)
- Try removing and reinstalling
- Check Console.app for error messages

**Network connection fails:**
- Check firewall settings
- Allow incoming connections for SynapseNet
- Ports needed: 9900-9910

### Linux Issues

**AppImage won't run:**
```bash
# Install FUSE if needed
sudo apt install fuse libfuse2  # Ubuntu/Debian
sudo dnf install fuse fuse-libs  # Fedora
```

**Missing dependencies:**
```bash
# Check for missing libraries
ldd SynapseNet-1.0.0-linux.AppImage
# Install any missing packages
```

**Permission denied:**
```bash
# Ensure executable permission
chmod +x SynapseNet-1.0.0-linux.AppImage
```

### Common Issues (All Platforms)

**Node won't connect:**
- Check internet connection
- Ensure ports 9900-9910 are not blocked
- Wait a few minutes for peer discovery
- Try restarting the application

**No peers connecting:**
- Check firewall settings
- Ensure you're not behind restrictive NAT
- Wait for peer discovery (can take 2-3 minutes)

**Search returns no results:**
- Ensure node is running and connected
- Wait for network synchronization
- Try different search terms

**Application uses too much memory:**
- This is normal during initial sync
- Memory usage should stabilize after sync
- Expected: 100-200MB

---

## Uninstallation

### Windows

**Using Control Panel:**
1. Open Control Panel
2. Go to "Programs and Features"
3. Find "SynapseNet"
4. Click "Uninstall"
5. Follow the wizard

**Manual removal:**
1. Delete installation folder
2. Delete desktop shortcut
3. Delete Start menu entry

**Data location:**
```
C:\Users\<YourName>\AppData\Roaming\SynapseNet
```

### macOS

**Remove application:**
1. Open Applications folder
2. Drag SynapseNet to Trash
3. Empty Trash

**Remove data:**
```bash
rm -rf ~/Library/Application\ Support/SynapseNet
rm -rf ~/Library/Caches/SynapseNet
```

### Linux

**Remove AppImage:**
```bash
rm ~/Downloads/SynapseNet-1.0.0-linux.AppImage
# Or if moved to /opt:
sudo rm /opt/synapsenet/SynapseNet-1.0.0-linux.AppImage
sudo rm /usr/local/bin/synapsenet
```

**Remove desktop entry:**
```bash
rm ~/.local/share/applications/synapsenet.desktop
```

**Remove data:**
```bash
rm -rf ~/.local/share/SynapseNet
rm -rf ~/.cache/SynapseNet
```

---

## Data Backup

### Backup Your Data

**Windows:**
```
Copy: C:\Users\<YourName>\AppData\Roaming\SynapseNet
To: Your backup location
```

**macOS:**
```bash
cp -r ~/Library/Application\ Support/SynapseNet ~/Backups/
```

**Linux:**
```bash
cp -r ~/.local/share/SynapseNet ~/Backups/
```

### Restore Your Data

1. Uninstall SynapseNet
2. Copy backup to original location
3. Reinstall SynapseNet
4. Your data will be restored

---

## Getting Help

**Documentation:**
- https://synapsenet.org/docs

**GitHub Issues:**
- https://github.com/synapsenet/synapsenet/issues

**Community:**
- Check GitHub Discussions
- Read the Genesis Manifest
- Join the network and ask questions

---

**Welcome to SynapseNet!** 🌍✨

The network is ready. The protocol is open. The future is distributed.
