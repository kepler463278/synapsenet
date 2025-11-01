import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface NodeStats {
  total_grains: number;
  network_peers: number;
  storage_path: string;
  embedding_model: string;
}

function StatsView() {
  const [stats, setStats] = useState<NodeStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadStats = async () => {
    setLoading(true);
    setError(null);

    try {
      const nodeStats = await invoke<NodeStats>('get_stats');
      setStats(nodeStats);
    } catch (err) {
      setError(`Failed to load stats: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadStats();
  }, []);

  if (loading) {
    return (
      <div className="view-container">
        <h2>Node Statistics</h2>
        <p>Loading...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="view-container">
        <h2>Node Statistics</h2>
        <div className="message error">{error}</div>
        <button onClick={loadStats} className="btn-primary">
          Retry
        </button>
      </div>
    );
  }

  return (
    <div className="view-container">
      <h2>Node Statistics</h2>
      <p className="subtitle">Overview of your SynapseNet node</p>

      <div className="stats-grid">
        <div className="stat-card">
          <div className="stat-icon">📦</div>
          <div className="stat-content">
            <div className="stat-value">{stats?.total_grains || 0}</div>
            <div className="stat-label">Knowledge Grains</div>
          </div>
        </div>

        <div className="stat-card">
          <div className="stat-icon">🌐</div>
          <div className="stat-content">
            <div className="stat-value">{stats?.network_peers || 0}</div>
            <div className="stat-label">Network Peers</div>
          </div>
        </div>

        <div className="stat-card">
          <div className="stat-icon">🤖</div>
          <div className="stat-content">
            <div className="stat-value">{stats?.embedding_model || 'N/A'}</div>
            <div className="stat-label">Embedding Model</div>
          </div>
        </div>

        <div className="stat-card full-width">
          <div className="stat-icon">💾</div>
          <div className="stat-content">
            <div className="stat-value small">{stats?.storage_path || 'N/A'}</div>
            <div className="stat-label">Storage Path</div>
          </div>
        </div>
      </div>

      <button onClick={loadStats} className="btn-secondary">
        🔄 Refresh
      </button>
    </div>
  );
}

export default StatsView;
