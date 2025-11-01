import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

function PeersScreen() {
  const [peers, setPeers] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadPeers();
    const interval = setInterval(loadPeers, 10000);
    return () => clearInterval(interval);
  }, []);

  const loadPeers = async () => {
    try {
      const data = await invoke<string[]>('syn_peers');
      setPeers(data);
    } catch (error) {
      console.error('Failed to load peers:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="screen peers-screen">
      <header>
        <h2>🌐 Network Peers</h2>
        <p className="subtitle">{peers.length} connected</p>
      </header>

      <div className="peers-list">
        {loading && <div className="loading">Loading peers...</div>}
        
        {!loading && peers.length === 0 && (
          <div className="empty-state">
            <p>No peers connected</p>
            <p className="hint">Connect to WiFi to discover peers</p>
          </div>
        )}

        {peers.map((peer) => (
          <div key={peer} className="peer-card">
            <div className="peer-icon">👤</div>
            <div className="peer-info">
              <div className="peer-id">{peer.substring(0, 16)}...</div>
              <div className="peer-status">🟢 Online</div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

export default PeersScreen;
