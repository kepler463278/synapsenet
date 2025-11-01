import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface PerformanceMetrics {
  embedding_avg_ms: number;
  query_avg_ms: number;
  batch_avg_ms: number;
  poe_calc_avg_ms: number;
}

interface NetworkHealth {
  peers_connected: number;
  messages_sent: number;
  messages_received: number;
  clusters: number;
  status: 'healthy' | 'degraded' | 'offline';
}

interface SystemMetrics {
  total_grains: number;
  models_loaded: number;
  storage_size_mb: number;
  uptime_seconds: number;
}

export default function MonitoringView() {
  const [performance, setPerformance] = useState<PerformanceMetrics | null>(null);
  const [network, setNetwork] = useState<NetworkHealth | null>(null);
  const [system, setSystem] = useState<SystemMetrics | null>(null);
  const [autoRefresh, setAutoRefresh] = useState(true);
  const [refreshInterval, setRefreshInterval] = useState(5000); // 5 seconds

  const loadMetrics = async () => {
    try {
      // TODO: Implement actual metrics endpoints
      // For now, use mock data
      setPerformance({
        embedding_avg_ms: 45.2,
        query_avg_ms: 12.8,
        batch_avg_ms: 234.5,
        poe_calc_avg_ms: 89.3,
      });

      setNetwork({
        peers_connected: 5,
        messages_sent: 1234,
        messages_received: 1189,
        clusters: 3,
        status: 'healthy',
      });

      setSystem({
        total_grains: 1523,
        models_loaded: 2,
        storage_size_mb: 245.8,
        uptime_seconds: 3600,
      });
    } catch (err) {
      console.error('Failed to load metrics:', err);
    }
  };

  useEffect(() => {
    loadMetrics();

    if (autoRefresh) {
      const interval = setInterval(loadMetrics, refreshInterval);
      return () => clearInterval(interval);
    }
  }, [autoRefresh, refreshInterval]);

  const formatUptime = (seconds: number): string => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    return `${hours}h ${minutes}m`;
  };

  const getHealthColor = (status: string): string => {
    switch (status) {
      case 'healthy':
        return '#10b981';
      case 'degraded':
        return '#fbbf24';
      case 'offline':
        return '#ef4444';
      default:
        return '#6b7280';
    }
  };

  return (
    <div className="monitoring-view">
      <div className="monitoring-header">
        <h2>📊 System Monitoring</h2>
        <div className="monitoring-controls">
          <label>
            <input
              type="checkbox"
              checked={autoRefresh}
              onChange={(e) => setAutoRefresh(e.target.checked)}
            />
            Auto-refresh
          </label>
          <select
            value={refreshInterval}
            onChange={(e) => setRefreshInterval(Number(e.target.value))}
            disabled={!autoRefresh}
          >
            <option value={1000}>1s</option>
            <option value={5000}>5s</option>
            <option value={10000}>10s</option>
            <option value={30000}>30s</option>
          </select>
          <button onClick={loadMetrics} className="btn-secondary">
            🔄 Refresh
          </button>
        </div>
      </div>

      <div className="metrics-grid">
        {/* Performance Metrics */}
        <div className="metric-card">
          <h3>⚡ Performance</h3>
          {performance && (
            <div className="metric-list">
              <div className="metric-item">
                <span className="metric-label">Embedding</span>
                <span className="metric-value">{performance.embedding_avg_ms.toFixed(1)}ms</span>
              </div>
              <div className="metric-item">
                <span className="metric-label">Query</span>
                <span className="metric-value">{performance.query_avg_ms.toFixed(1)}ms</span>
              </div>
              <div className="metric-item">
                <span className="metric-label">Batch</span>
                <span className="metric-value">{performance.batch_avg_ms.toFixed(1)}ms</span>
              </div>
              <div className="metric-item">
                <span className="metric-label">PoE Calc</span>
                <span className="metric-value">{performance.poe_calc_avg_ms.toFixed(1)}ms</span>
              </div>
            </div>
          )}
        </div>

        {/* Network Health */}
        <div className="metric-card">
          <h3>🌐 Network</h3>
          {network && (
            <>
              <div className="health-indicator">
                <span
                  className="health-dot"
                  style={{ backgroundColor: getHealthColor(network.status) }}
                />
                <span className="health-status">{network.status.toUpperCase()}</span>
              </div>
              <div className="metric-list">
                <div className="metric-item">
                  <span className="metric-label">Peers</span>
                  <span className="metric-value">{network.peers_connected}</span>
                </div>
                <div className="metric-item">
                  <span className="metric-label">Clusters</span>
                  <span className="metric-value">{network.clusters}</span>
                </div>
                <div className="metric-item">
                  <span className="metric-label">Messages Sent</span>
                  <span className="metric-value">{network.messages_sent.toLocaleString()}</span>
                </div>
                <div className="metric-item">
                  <span className="metric-label">Messages Received</span>
                  <span className="metric-value">{network.messages_received.toLocaleString()}</span>
                </div>
              </div>
            </>
          )}
        </div>

        {/* System Metrics */}
        <div className="metric-card">
          <h3>💾 System</h3>
          {system && (
            <div className="metric-list">
              <div className="metric-item">
                <span className="metric-label">Total Grains</span>
                <span className="metric-value">{system.total_grains.toLocaleString()}</span>
              </div>
              <div className="metric-item">
                <span className="metric-label">Models Loaded</span>
                <span className="metric-value">{system.models_loaded}</span>
              </div>
              <div className="metric-item">
                <span className="metric-label">Storage Size</span>
                <span className="metric-value">{system.storage_size_mb.toFixed(1)} MB</span>
              </div>
              <div className="metric-item">
                <span className="metric-label">Uptime</span>
                <span className="metric-value">{formatUptime(system.uptime_seconds)}</span>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Performance Graphs Placeholder */}
      <div className="graphs-section">
        <h3>📈 Performance Trends</h3>
        <div className="graph-placeholder">
          <p>Real-time performance graphs coming soon...</p>
          <small>Will show embedding speed, query latency, and throughput over time</small>
        </div>
      </div>
    </div>
  );
}
