import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface Stats {
  grain_count: number;
  peer_count: number;
  storage_mb: number;
  battery_level: number;
  network_state: string;
}

function HomeScreen() {
  const [stats, setStats] = useState<Stats | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadStats();
    const interval = setInterval(loadStats, 5000);
    return () => clearInterval(interval);
  }, []);

  const loadStats = async () => {
    try {
      const data = await invoke<Stats>('syn_stats');
      setStats(data);
    } catch (error) {
      console.error('Failed to load stats:', error);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return <div className="screen loading">Loading...</div>;
  }

  return (
    <div className="screen home-screen">
      <header>
        <h1>🧠 SynapseNet</h1>
        <p className="subtitle">Your Personal Knowledge Network</p>
      </header>

      <div className="stats-grid">
        <div 
          className="stat-card"
          role="button"
          aria-label={`${stats?.grain_count || 0} grains stored`}
          tabIndex={0}
        >
          <div className="stat-icon" aria-hidden="true">📚</div>
          <div className="stat-value">{stats?.grain_count || 0}</div>
          <div className="stat-label">Grains</div>
        </div>

        <div 
          className="stat-card"
          role="button"
          aria-label={`${stats?.peer_count || 0} peers connected`}
          tabIndex={0}
        >
          <div className="stat-icon" aria-hidden="true">🌐</div>
          <div className="stat-value">{stats?.peer_count || 0}</div>
          <div className="stat-label">Peers</div>
        </div>

        <div 
          className="stat-card"
          role="button"
          aria-label={`${stats?.storage_mb.toFixed(1) || 0} megabytes used`}
          tabIndex={0}
        >
          <div className="stat-icon" aria-hidden="true">💾</div>
          <div className="stat-value">{stats?.storage_mb.toFixed(1) || 0}</div>
          <div className="stat-label">MB Used</div>
        </div>

        <div 
          className="stat-card"
          role="button"
          aria-label={`Battery level ${((stats?.battery_level || 0) * 100).toFixed(0)} percent`}
          tabIndex={0}
        >
          <div className="stat-icon" aria-hidden="true">🔋</div>
          <div className="stat-value">{((stats?.battery_level || 0) * 100).toFixed(0)}%</div>
          <div className="stat-label">Battery</div>
        </div>
      </div>

      <div className="network-status">
        <span className="status-indicator"></span>
        {stats?.network_state || 'Offline'}
      </div>

      <div className="quick-actions">
        <h3>Quick Actions</h3>
        <button className="action-btn primary">➕ Add Knowledge</button>
        <button className="action-btn">🔍 Search</button>
      </div>
    </div>
  );
}

export default HomeScreen;
