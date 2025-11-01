import { useState } from 'react';
import { Settings, FileText, Info } from 'lucide-react';

function SettingsScreen() {
  const [showManifest, setShowManifest] = useState(false);

  const genesisManifest = `
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
  `;

  return (
    <div className="screen">
      <div className="card">
        <h2 className="card-title">
          <Settings size={20} style={{ display: 'inline', marginRight: '8px' }} />
          Settings
        </h2>

        <div style={{ marginBottom: '24px' }}>
          <h3 style={{ fontSize: '16px', fontWeight: '600', marginBottom: '12px' }}>
            Node Configuration
          </h3>
          <div style={{ fontSize: '14px', color: '#6b7280' }}>
            <p>Node configuration options coming soon...</p>
          </div>
        </div>

        <div style={{ marginBottom: '24px' }}>
          <h3 style={{ fontSize: '16px', fontWeight: '600', marginBottom: '12px' }}>
            Network Preferences
          </h3>
          <div style={{ fontSize: '14px', color: '#6b7280' }}>
            <p>Network preferences coming soon...</p>
          </div>
        </div>
      </div>

      <div className="card">
        <h2 className="card-title">
          <FileText size={20} style={{ display: 'inline', marginRight: '8px' }} />
          Genesis Manifest
        </h2>

        <p style={{ fontSize: '14px', color: '#6b7280', marginBottom: '16px' }}>
          The philosophical foundation of SynapseNet - our declaration of cognitive
          independence.
        </p>

        <button
          className="btn btn-primary"
          onClick={() => setShowManifest(!showManifest)}
        >
          {showManifest ? 'Hide Manifest' : 'Read Manifest'}
        </button>

        {showManifest && (
          <div
            style={{
              marginTop: '16px',
              padding: '16px',
              background: '#f9fafb',
              borderRadius: '8px',
              fontFamily: 'monospace',
              fontSize: '12px',
              lineHeight: '1.6',
              whiteSpace: 'pre-wrap',
              maxHeight: '500px',
              overflowY: 'auto',
            }}
          >
            {genesisManifest}
          </div>
        )}
      </div>

      <div className="card">
        <h2 className="card-title">
          <Info size={20} style={{ display: 'inline', marginRight: '8px' }} />
          About
        </h2>

        <div style={{ fontSize: '14px', lineHeight: '1.6' }}>
          <div style={{ marginBottom: '12px' }}>
            <strong>SynapseNet Desktop</strong>
          </div>
          <div style={{ marginBottom: '8px', color: '#6b7280' }}>
            Version: 1.0.0
          </div>
          <div style={{ marginBottom: '8px', color: '#6b7280' }}>
            Release Date: November 1, 2024
          </div>
          <div style={{ marginBottom: '16px', color: '#6b7280' }}>
            License: MIT
          </div>

          <div style={{ marginBottom: '12px' }}>
            <strong>What is SynapseNet?</strong>
          </div>
          <p style={{ color: '#374151', marginBottom: '12px' }}>
            SynapseNet is a decentralized knowledge network where intelligence emerges
            from collective wisdom. No central servers, no gatekeepers, pure peer-to-peer
            knowledge exchange.
          </p>

          <div style={{ marginBottom: '12px' }}>
            <strong>Core Principles</strong>
          </div>
          <ul style={{ paddingLeft: '20px', color: '#374151' }}>
            <li style={{ marginBottom: '8px' }}>
              Intelligence is commons, not property
            </li>
            <li style={{ marginBottom: '8px' }}>
              Knowledge belongs to humanity
            </li>
            <li style={{ marginBottom: '8px' }}>
              Privacy and autonomy are fundamental
            </li>
            <li style={{ marginBottom: '8px' }}>
              Networks grow through emergence
            </li>
          </ul>

          <div style={{ marginTop: '16px', paddingTop: '16px', borderTop: '1px solid #e5e7eb' }}>
            <a
              href="https://synapsenet.org"
              style={{ color: '#2563eb', textDecoration: 'none' }}
              target="_blank"
              rel="noopener noreferrer"
            >
              synapsenet.org
            </a>
            {' | '}
            <a
              href="https://github.com/synapsenet/synapsenet"
              style={{ color: '#2563eb', textDecoration: 'none' }}
              target="_blank"
              rel="noopener noreferrer"
            >
              GitHub
            </a>
          </div>
        </div>
      </div>
    </div>
  );
}

export default SettingsScreen;
