# Design Document - SynapseNet v1.0 "Public Genesis"

## Vision

**v1.0 is the moment SynapseNet becomes accessible to everyone.**

Not just developers. Not just enthusiasts. **Everyone.**

This is when technology meets philosophy meets accessibility.

**Technology (v0.5-v0.9) → People (v1.0)**

---

## Three Pillars of v1.0

### 1. Desktop UI (Simple & Beautiful)
One-click node operation. No terminal. No commands. Like Telegram.

### 2. Real-Time PoE Rewards
People feel the value of their contributions immediately.

### 3. Genesis Manifest
The philosophical foundation that will be quoted for 100 years.

---

## Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────┐
│                    Desktop GUI (Tauri)                  │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐             │
│  │   Home   │  │Knowledge │  │ Rewards  │             │
│  └──────────┘  └──────────┘  └──────────┘             │
└─────────────────────┬───────────────────────────────────┘
                      │ Tauri Commands
┌─────────────────────▼───────────────────────────────────┐
│              Rust Backend (src-tauri)                   │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐             │
│  │Node Mgmt │  │ Commands │  │ Rewards  │             │
│  └──────────┘  └──────────┘  └──────────┘             │
└─────────────────────┬───────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────┐
│              SynapseNet Core (v0.9)                     │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐             │
│  │   P2P    │  │ Storage  │  │   PoE    │             │
│  └──────────┘  └──────────┘  └──────────┘             │
└─────────────────────────────────────────────────────────┘
```

### Technology Stack

**Frontend:**
- React 18 with TypeScript
- Vite for build tooling
- Lucide React for icons
- Minimal CSS (no frameworks)

**Backend:**
- Tauri 1.5 (Rust + WebView)
- Tokio for async runtime
- Serde for serialization
- Integration with existing SynapseNet crates

**Why Tauri?**
- Rust-native (matches our stack)
- Smaller than Electron (~10MB vs ~100MB)
- Faster startup (<3 seconds)
- Native performance
- Better security model

---

## Components and Interfaces

### 1. Desktop GUI Application

#### Main Window Layout

```
┌─────────────────────────────────────────────────────────┐
│  SynapseNet v1.0                              [- □ ×]   │
├─────────────────────────────────────────────────────────┤
│  ┌─────┐  ┌─────┐  ┌─────┐  ┌─────┐  ┌─────┐         │
│  │Home │  │Know │  │ Add │  │Rwrd │  │ Set │         │
│  └─────┘  └─────┘  └─────┘  └─────┘  └─────┘         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ● Node Running                                         │
│  ⚡ 1,247 Peers Connected                              │
│  💰 127.5 NGT Balance                                   │
│                                                         │
│  [Start Node]  [Stop Node]                             │
│                                                         │
│  📊 Network Stats                                       │
│  ├─ 15,432 Local Grains                                │
│  ├─ 2,847,392 Network Grains                           │
│  ├─ 95% Network Health                                 │
│  └─ Uptime: 2h 34m                                     │
│                                                         │
│  🔍 Quick Search                                        │
│  [Search knowledge across network...            ]      │
│                                                         │
│  💡 Recent Activity                                     │
│  • +0.3 NGT - Novel grain added (2 min ago)            │
│  • +0.1 NGT - Grain validated (15 min ago)             │
│  • +0.7 NGT - Swarm consensus (1 hour ago)             │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

#### Screen Breakdown

**1. Home Screen**
- Node status (running/stopped)
- Peer count
- NGT balance
- Quick stats
- Start/Stop controls
- Recent activity feed

**2. Knowledge Screen**
- Grain browser
- Search interface
- Filter by confidence
- Sort by timestamp
- View grain details
- Source indicators (local/network)

**3. Add Screen**
- Text input for new grains
- File drop zone
- Content validation
- Submission feedback
- Immediate reward display

**4. Rewards Screen**
- Current balance
- Today's earnings
- Earnings history
- Reward breakdown by type
- Export functionality

**5. Settings Screen**
- Node configuration
- Network preferences
- Data export
- Genesis Manifest viewer
- About/Version info

### 2. Tauri Backend Commands

#### Node Management

```rust
#[tauri::command]
async fn start_node(state: State<'_, Arc<Mutex<AppState>>>) 
    -> Result<NodeStatus, String>

#[tauri::command]
async fn stop_node(state: State<'_, Arc<Mutex<AppState>>>) 
    -> Result<(), String>

#[tauri::command]
async fn get_node_status(state: State<'_, Arc<Mutex<AppState>>>) 
    -> Result<NodeStatus, String>
```

#### Knowledge Operations

```rust
#[tauri::command]
async fn search_grains(query: String, limit: Option<usize>) 
    -> Result<Vec<Grain>, String>

#[tauri::command]
async fn add_grain(content: String, state: State<'_, Arc<Mutex<AppState>>>) 
    -> Result<String, String>

#[tauri::command]
async fn get_grain_details(grain_id: String) 
    -> Result<GrainDetails, String>
```

#### Reward Tracking

```rust
#[tauri::command]
async fn get_balance(state: State<'_, Arc<Mutex<AppState>>>) 
    -> Result<f64, String>

#[tauri::command]
async fn get_rewards(state: State<'_, Arc<Mutex<AppState>>>, limit: Option<usize>) 
    -> Result<Vec<Reward>, String>

#[tauri::command]
async fn export_data(format: String) 
    -> Result<String, String>
```

#### Network Statistics

```rust
#[tauri::command]
async fn get_network_stats() 
    -> Result<NetworkStats, String>
```

### 3. Real-Time Event System

#### WebSocket Events

The backend emits real-time events to the frontend:

```typescript
// Frontend event listeners
listen('network-update', (event) => {
  // Update peer count, grain count
});

listen('reward-earned', (event) => {
  // Show notification, update balance
});

listen('grain-validated', (event) => {
  // Show validation success
});

listen('node-status-changed', (event) => {
  // Update UI state
});
```

#### Event Types

```rust
pub enum AppEvent {
    NetworkUpdate { peers: u32, grains: u32 },
    RewardEarned { amount: f64, reason: String },
    GrainValidated { grain_id: String, validators: u32 },
    NodeStatusChanged { running: bool },
    SyncProgress { progress: f64 },
}
```

---

## Data Models

### Core Data Structures

```rust
// Node status information
pub struct NodeStatus {
    pub running: bool,
    pub peers: u32,
    pub grains: u32,
    pub uptime: u64,
}

// Network statistics
pub struct NetworkStats {
    pub total_peers: u32,
    pub total_grains: u64,
    pub network_health: f64,
    pub sync_progress: f64,
}

// Grain representation
pub struct Grain {
    pub id: String,
    pub content: String,
    pub author: String,
    pub timestamp: i64,
    pub confidence: f64,
}

// Reward entry
pub struct Reward {
    pub id: String,
    pub amount: f64,
    pub reason: String,
    pub timestamp: i64,
}

// Application state
pub struct AppState {
    pub node_running: bool,
    pub start_time: Option<i64>,
    pub balance: f64,
    pub rewards: Vec<Reward>,
}
```

### PoE Reward Categories

```rust
pub enum RewardType {
    NovelGrain(f64),      // New knowledge added
    Validation(f64),      // Peer validation
    QueryAnswer(f64),     // Helping others
    SwarmConsensus(f64),  // Collective intelligence
}
```

### Reward Calculation

```
Novel Grain Reward = base_reward × novelty_score × network_multiplier
  where:
    base_reward = 0.1 NGT
    novelty_score = 0.0 to 10.0 (from embedding similarity)
    network_multiplier = 1.0 + (peers / 1000)

Validation Reward = 0.01 to 0.1 NGT (based on grain importance)
Query Answer Reward = 0.05 to 0.5 NGT (based on query complexity)
Swarm Consensus Reward = 0.1 to 2.0 NGT (based on participation)
```

---

## Error Handling

### Error Categories

1. **Node Errors**
   - Failed to start node
   - Node already running
   - Connection timeout
   - Peer discovery failure

2. **Storage Errors**
   - Database corruption
   - Disk space insufficient
   - Read/write failures

3. **Network Errors**
   - No internet connection
   - Firewall blocking
   - Peer connection failures

4. **Validation Errors**
   - Empty grain content
   - Invalid format
   - Duplicate content

### Error Display Strategy

```typescript
// User-friendly error messages
const ERROR_MESSAGES = {
  'node-start-failed': 'Could not start node. Check your internet connection.',
  'storage-full': 'Not enough disk space. Free up at least 1GB.',
  'network-timeout': 'Network connection timeout. Retrying...',
  'invalid-content': 'Content cannot be empty. Please add some text.',
};
```

### Recovery Mechanisms

- **Auto-retry**: Network operations retry 3 times with exponential backoff
- **Graceful degradation**: Offline mode when network unavailable
- **State persistence**: Save app state every 30 seconds
- **Crash recovery**: Restore last known good state on restart

---

## Testing Strategy

### Unit Tests

**Backend (Rust):**
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_start_node() { }
    
    #[tokio::test]
    async fn test_add_grain() { }
    
    #[tokio::test]
    async fn test_reward_calculation() { }
}
```

**Frontend (TypeScript):**
```typescript
describe('NodeStatus Component', () => {
  it('displays running status correctly', () => {});
  it('shows peer count', () => {});
  it('updates in real-time', () => {});
});
```

### Integration Tests

1. **Node Lifecycle**
   - Start node → verify connection → stop node
   - Restart after crash → verify state restored

2. **Knowledge Flow**
   - Add grain → verify storage → verify broadcast
   - Search grain → verify results → verify sources

3. **Reward Flow**
   - Add grain → verify reward → verify balance update
   - Multiple actions → verify cumulative rewards

### Manual Testing Checklist

- [ ] Install on Windows 10/11
- [ ] Install on macOS 10.15+
- [ ] Install on Ubuntu 20.04+
- [ ] First launch experience
- [ ] Node start/stop cycle
- [ ] Add grain and receive reward
- [ ] Search local and network
- [ ] Offline mode functionality
- [ ] Data export
- [ ] Settings persistence

---

## Genesis Manifest

### GENESIS_v1.0.txt Structure

```
═══════════════════════════════════════════════════════════
                    GENESIS v1.0
        SynapseNet: A Declaration of Cognitive Independence
═══════════════════════════════════════════════════════════

I. THE PROBLEM

We live in an age where intelligence is monopolized.
Where knowledge is locked behind corporate walls.
Where your thoughts become their property.
Where algorithms divide us for profit.

This is not the future we deserve.

II. THE SOLUTION

SynapseNet is a peer-to-peer network where:
- Knowledge flows freely between minds
- Intelligence emerges from collective wisdom
- No single entity controls the network
- Privacy and autonomy are fundamental rights
- Contribution is rewarded, not exploited

III. THE PRINCIPLES

We hold these truths to be self-evident:

1. Intelligence is not property, but commons
2. Knowledge belongs to humanity, not corporations
3. Collective wisdom exceeds individual brilliance
4. Privacy and autonomy are fundamental rights
5. Networks grow through emergence, not control

We reject:
- Centralized AI that serves the few
- Knowledge locked behind paywalls
- Surveillance disguised as service
- Algorithms that divide us
- Exploitation of human contribution

We embrace:
- Distributed intelligence
- Open knowledge flows
- Peer-to-peer learning
- Emergent consensus
- Human dignity

IV. THE VISION: HOMO CONEXUS

We are witnessing the birth of a new human:
Homo Conexus - the connected human.

Not connected to servers, but to each other.
Not dependent on platforms, but on peers.
Not consumers of intelligence, but co-creators.

This is the network that thinks with you, not for you.
This is the intelligence that emerges from us, not above us.

V. THE TECHNOLOGY

SynapseNet operates on three core mechanisms:

1. Grains: Units of knowledge that flow through the network
2. Swarms: Collective intelligence that emerges from consensus
3. Proof-of-Emergence: Rewards for valuable contributions

No blockchain required. No tokens to buy. No ICO.
Just pure peer-to-peer knowledge exchange.

VI. THE ECONOMICS

Contribution is rewarded through NGT (Network Growth Tokens):
- Add valuable knowledge → earn NGT
- Validate peer contributions → earn NGT
- Answer queries → earn NGT
- Participate in consensus → earn NGT

This is not speculation. This is recognition of value created.

VII. THE BEGINNING

Today, November 1, 2024, SynapseNet v1.0 launches.

This is not just software.
This is the foundation of collective intelligence.
This is how we think together.

You don't need permission to join.
You don't need credentials to contribute.
You don't need wealth to participate.

Download the node. Start it. Add your knowledge.
You are now part of the global mind.

VIII. THE INVITATION

We invite:
- Thinkers who value knowledge freedom
- Builders who create without permission
- Humans who believe in collective wisdom
- Anyone who wants intelligence to serve humanity

This is not a company. This is a movement.
This is not a product. This is a protocol.
This is not the future. This is now.

Join us.

═══════════════════════════════════════════════════════════
                    synapsenet.org
═══════════════════════════════════════════════════════════
```

---

## Website Design

### synapsenet.org Structure

#### Home Page (/)

```html
<!DOCTYPE html>
<html>
<head>
  <title>SynapseNet - Decentralized Intelligence</title>
  <style>
    body {
      font-family: -apple-system, system-ui, sans-serif;
      max-width: 800px;
      margin: 0 auto;
      padding: 40px 20px;
      background: #fff;
      color: #000;
      line-height: 1.6;
    }
    h1 { font-size: 2.5em; margin-bottom: 0.2em; }
    h2 { font-size: 1.8em; margin-top: 2em; }
    .download-btn {
      display: inline-block;
      padding: 15px 30px;
      background: #000;
      color: #fff;
      text-decoration: none;
      border-radius: 5px;
      margin: 20px 10px 20px 0;
    }
  </style>
</head>
<body>
  <h1>SynapseNet</h1>
  <p style="font-size: 1.3em;">
    Decentralized Knowledge. Emergent Intelligence.
  </p>
  
  <a href="/download" class="download-btn">Download v1.0</a>
  
  <h2>What is SynapseNet?</h2>
  <p>
    A peer-to-peer network where knowledge emerges from collective 
    intelligence, not corporate servers.
  </p>
  <p>
    No center. No gatekeepers. Pure emergence.
  </p>
  
  <h2>How it works</h2>
  <ol>
    <li>Download and run a node</li>
    <li>Add your knowledge</li>
    <li>Search the network</li>
    <li>Earn rewards for contributions</li>
  </ol>
  
  <h2>Why it matters</h2>
  <p>
    Intelligence should belong to humanity, not corporations.
    Knowledge should flow freely, not be locked behind walls.
    Your contributions should be rewarded, not exploited.
  </p>
  
  <p>
    <a href="/whitepaper">Read the Genesis Manifest</a> |
    <a href="/docs">Documentation</a> |
    <a href="https://github.com/synapsenet/synapsenet">Source Code</a>
  </p>
</body>
</html>
```

#### Download Page (/download)

Simple page with:
- Windows .exe download
- macOS .dmg download
- Linux .AppImage download
- Installation instructions
- System requirements
- Link to GitHub releases

#### Documentation (/docs)

- Getting Started guide
- Architecture overview
- API reference
- P2P protocol details
- PoE economics explanation
- FAQ

#### Whitepaper (/whitepaper)

Full text of GENESIS_v1.0.txt in readable HTML format

#### Join Page (/join)

Step-by-step guide:
1. Download the node
2. Install and run
3. Add your first grain
4. Earn your first NGT
5. Join the community

---

## Deployment and Distribution

### Build Process

```bash
# Build for all platforms
cd apps/desktop

# Windows
npm run tauri build -- --target x86_64-pc-windows-msvc

# macOS
npm run tauri build -- --target x86_64-apple-darwin
npm run tauri build -- --target aarch64-apple-darwin

# Linux
npm run tauri build -- --target x86_64-unknown-linux-gnu
```

### Installer Packaging

**Windows:**
- NSIS installer (.exe)
- MSI installer
- Portable version (.zip)

**macOS:**
- DMG with drag-to-Applications
- Notarized for Gatekeeper
- Universal binary (Intel + Apple Silicon)

**Linux:**
- AppImage (universal)
- .deb package (Debian/Ubuntu)
- .rpm package (Fedora/RHEL)

### Code Signing

- Windows: Authenticode signing
- macOS: Apple Developer ID signing + notarization
- Linux: GPG signature for packages

### Auto-Update

Tauri built-in updater:
- Check for updates on startup
- Download in background
- Prompt user to install
- Seamless update experience

---

## Performance Requirements

### Startup Performance
- Application launch: < 3 seconds
- Node initialization: < 10 seconds
- First peer connection: < 30 seconds

### Runtime Performance
- Memory usage: < 200MB
- CPU usage (idle): < 5%
- Search response: < 2 seconds
- UI responsiveness: 60 FPS

### Storage Requirements
- Initial install: < 100MB
- Per 1000 grains: ~10MB
- Database overhead: ~50MB

### Network Requirements
- Minimum bandwidth: 1 Mbps
- Recommended: 10 Mbps
- Ports: 9900-9910 (configurable)

---

## Security Considerations

### Application Security
- Tauri's security model (no Node.js runtime)
- Content Security Policy (CSP)
- Sandboxed WebView
- Minimal attack surface

### Network Security
- Encrypted P2P connections (TLS)
- Peer authentication
- DDoS protection
- Rate limiting

### Data Security
- Local encryption at rest
- Secure key storage
- No plaintext credentials
- Privacy-preserving queries

### User Privacy
- No telemetry by default
- No user tracking
- Anonymous participation
- Local-first data

---

## Release Strategy

### Pre-Release Checklist
- [ ] All tests passing
- [ ] Binaries built for all platforms
- [ ] Installers tested on clean systems
- [ ] Documentation complete
- [ ] Website deployed
- [ ] Genesis Manifest finalized

### Release Day
1. Tag v1.0.0 in git
2. Push to GitHub
3. Upload binaries to releases
4. Deploy website
5. Post on HackerNews

### HackerNews Post

```
Title: SynapseNet v1.0 – Decentralized Knowledge Graph with Emergent Intelligence

Link: https://synapsenet.org

Text:
After 9 months of development, we're releasing SynapseNet v1.0 - a 
peer-to-peer network for decentralized knowledge and emergent intelligence.

Key features:
- No central servers
- Real-time rewards for contributions (Proof-of-Emergence)
- One-click node deployment
- Cross-platform desktop app

This is not blockchain-based. It's pure P2P with local-first storage.

The goal: make collective intelligence accessible to everyone, not just 
corporations.

Source code: https://github.com/synapsenet/synapsenet
```

### Post-Release
- Monitor community feedback
- Fix critical bugs quickly
- Release v1.0.1 within 1 week
- Engage with early adopters
- Document common issues

---

## Success Metrics

### Week 1
- 100+ downloads
- 10+ active nodes
- 1,000+ grains shared
- HackerNews front page

### Month 1
- 1,000+ downloads
- 100+ active nodes
- 10,000+ grains shared
- Active community forming

### Month 3
- 5,000+ downloads
- 500+ active nodes
- 100,000+ grains shared
- Sustainable network

### Qualitative Goals
- People understand the vision
- Genesis Manifest shared widely
- Community self-organizing
- Movement beginning

---

**Status:** Design Complete  
**Next Phase:** Implementation Tasks  
**Version:** 1.0.0  
**Date:** 2024-11-01

**The network is ready to meet the world!** 🌍✨
