import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Play, Square, Activity, TrendingUp } from 'lucide-react';

interface NodeStatus {
  running: boolean;
  peers: number;
  grains: number;
  uptime: number;
}

interface NetworkStats {
  total_peers: number;
  total_grains: number;
  network_health: number;
  sync_progress: number;
}

interface Reward {
  id: string;
  amount: number;
  reason: string;
  timestamp: number;
}

interface Props {
  nodeStatus: NodeStatus | null;
  balance: number;
  onRefresh: () => void;
}

function HomeScreen({ nodeStatus, balance, onRefresh }: Props) {
  const [loading, setLoading] = useState(false);
  const [networkStats, setNetworkStats] = useState<NetworkStats | null>(null);
  const [recentRewards, setRecentRewards] = useState<Reward[]>([]);
  const [todayEarnings, setTodayEarnings] = useState(0);

  useEffect(() => {
    loadNetworkStats();
    loadRecentRewards();
    loadTodayEarnings();
  }, []);

  const loadNetworkStats = async () => {
    try {
      const stats = await invoke<NetworkStats>('get_network_stats');
      setNetworkStats(stats);
    } catch (error) {
      console.error('Failed to load network stats:', error);
    }
  };

  const loadRecentRewards = async () => {
    try {
      const rewards = await invoke<Reward[]>('get_rewards', { limit: 5 });
      setRecentRewards(rewards);
    } catch (error) {
      console.error('Failed to load rewards:', error);
    }
  };

  const loadTodayEarnings = async () => {
    try {
      const earnings = await invoke<number>('get_today_earnings');
      setTodayEarnings(earnings);
    } catch (error) {
      console.error('Failed to load today earnings:', error);
    }
  };

  const handleStartNode = async () => {
    setLoading(true);
    try {
      await invoke('start_node');
      onRefresh();
      await loadNetworkStats();
    } catch (error) {
      alert('Failed to start node: ' + error);
    } finally {
      setLoading(false);
    }
  };

  const handleStopNode = async () => {
    setLoading(true);
    try {
      await invoke('stop_node');
      onRefresh();
      await loadNetworkStats();
    } catch (error) {
      alert('Failed to stop node: ' + error);
    } finally {
      setLoading(false);
    }
  };

  const formatUptime = (seconds: number) => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    return `${hours}h ${minutes}m`;
  };

  const formatTimestamp = (timestamp: number) => {
    const now = Date.now() / 1000;
    const diff = now - timestamp;

    if (diff < 60) return 'just now';
    if (diff < 3600) return `${Math.floor(diff / 60)} min ago`;
    if (diff < 86400) return `${Math.floor(diff / 3600)} hours ago`;
    return `${Math.floor(diff / 86400)} days ago`;
  };

  return (
    <div className="screen">
      {/* Node Status Card */}
      <div className="card">
        <div className="flex" style={{ justifyContent: 'space-between', alignItems: 'center' }}>
          <div>
            <h2 className="card-title">Node Status</h2>
            <div className="flex gap-2" style={{ alignItems: 'center' }}>
              {nodeStatus?.running ? (
                <span className="status status-running">
                  <span className="status-dot"></span>
                  Running
                </span>
              ) : (
                <span className="status status-stopped">
                  <span className="status-dot"></span>
                  Stopped
                </span>
              )}
              {nodeStatus?.running && (
                <span style={{ fontSize: '14px', color: '#6b7280' }}>
                  Uptime: {formatUptime(nodeStatus.uptime)}
                </span>
              )}
            </div>
          </div>
          <div className="flex gap-2">
            {!nodeStatus?.running ? (
              <button
                className="btn btn-primary"
                onClick={handleStartNode}
                disabled={loading}
              >
                <Play size={16} style={{ marginRight: '8px', display: 'inline' }} />
                Start Node
              </button>
            ) : (
              <button
                className="btn btn-danger"
                onClick={handleStopNode}
                disabled={loading}
              >
                <Square size={16} style={{ marginRight: '8px', display: 'inline' }} />
                Stop Node
              </button>
            )}
          </div>
        </div>
      </div>

      {/* Network Stats Card */}
      <div className="card">
        <h2 className="card-title">
          <Activity size={20} style={{ display: 'inline', marginRight: '8px' }} />
          Network Statistics
        </h2>
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(2, 1fr)', gap: '16px' }}>
          <div>
            <div style={{ fontSize: '13px', color: '#6b7280', marginBottom: '4px' }}>
              Local Grains
            </div>
            <div style={{ fontSize: '24px', fontWeight: '600' }}>
              {nodeStatus?.grains.toLocaleString() || 0}
            </div>
          </div>
          <div>
            <div style={{ fontSize: '13px', color: '#6b7280', marginBottom: '4px' }}>
              Network Grains
            </div>
            <div style={{ fontSize: '24px', fontWeight: '600' }}>
              {networkStats?.total_grains.toLocaleString() || 0}
            </div>
          </div>
          <div>
            <div style={{ fontSize: '13px', color: '#6b7280', marginBottom: '4px' }}>
              Network Health
            </div>
            <div style={{ fontSize: '24px', fontWeight: '600' }}>
              {((networkStats?.network_health || 0) * 100).toFixed(0)}%
            </div>
          </div>
          <div>
            <div style={{ fontSize: '13px', color: '#6b7280', marginBottom: '4px' }}>
              Sync Progress
            </div>
            <div style={{ fontSize: '24px', fontWeight: '600' }}>
              {((networkStats?.sync_progress || 0) * 100).toFixed(0)}%
            </div>
          </div>
        </div>
      </div>

      {/* Today's Earnings Card */}
      <div className="card">
        <h2 className="card-title">
          <TrendingUp size={20} style={{ display: 'inline', marginRight: '8px' }} />
          Today's Earnings
        </h2>
        <div style={{ fontSize: '32px', fontWeight: '600', color: '#22c55e' }}>
          +{todayEarnings.toFixed(2)} NGT
        </div>
        <div style={{ fontSize: '14px', color: '#6b7280', marginTop: '8px' }}>
          Total Balance: {balance.toFixed(2)} NGT
        </div>
      </div>

      {/* Recent Activity Card */}
      <div className="card">
        <h2 className="card-title">Recent Activity</h2>
        {recentRewards.length === 0 ? (
          <div className="empty-state">
            <div className="empty-state-text">No recent activity</div>
          </div>
        ) : (
          <div className="list">
            {recentRewards.map((reward) => (
              <div key={reward.id} className="reward-item">
                <div className="reward-info">
                  <div className="reward-reason">{reward.reason}</div>
                  <div className="reward-time">{formatTimestamp(reward.timestamp)}</div>
                </div>
                <div className="reward-amount">+{reward.amount.toFixed(2)} NGT</div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

export default HomeScreen;
