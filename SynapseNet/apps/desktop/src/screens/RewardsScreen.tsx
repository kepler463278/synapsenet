import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Coins, TrendingUp, Download } from 'lucide-react';

interface Reward {
  id: string;
  amount: number;
  reason: string;
  timestamp: number;
}

interface Props {
  balance: number;
}

function RewardsScreen({ balance }: Props) {
  const [rewards, setRewards] = useState<Reward[]>([]);
  const [loading, setLoading] = useState(true);
  const [todayEarnings, setTodayEarnings] = useState(0);

  useEffect(() => {
    loadRewards();
    loadTodayEarnings();
  }, []);

  const loadRewards = async () => {
    setLoading(true);
    try {
      const data = await invoke<Reward[]>('get_rewards', { limit: 100 });
      setRewards(data);
    } catch (error) {
      console.error('Failed to load rewards:', error);
    } finally {
      setLoading(false);
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

  const handleExport = async (format: 'json' | 'csv') => {
    try {
      const filename = await invoke<string>('export_data', { format });
      alert(`Data exported to ${filename}`);
    } catch (error) {
      console.error('Export failed:', error);
      alert('Export failed: ' + error);
    }
  };

  const formatTimestamp = (timestamp: number) => {
    const date = new Date(timestamp * 1000);
    return date.toLocaleString();
  };

  const getRewardsByType = () => {
    const byType: Record<string, number> = {};
    rewards.forEach((reward) => {
      const type = reward.reason.includes('grain')
        ? 'Novel Grains'
        : reward.reason.includes('validat')
        ? 'Validations'
        : reward.reason.includes('query')
        ? 'Query Answers'
        : reward.reason.includes('swarm')
        ? 'Swarm Consensus'
        : 'Other';

      byType[type] = (byType[type] || 0) + reward.amount;
    });
    return byType;
  };

  const rewardsByType = getRewardsByType();

  return (
    <div className="screen">
      {/* Balance Card */}
      <div className="card">
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
          <div>
            <h2 className="card-title">
              <Coins size={20} style={{ display: 'inline', marginRight: '8px' }} />
              NGT Balance
            </h2>
            <div style={{ fontSize: '48px', fontWeight: '600', color: '#2563eb', marginTop: '8px' }}>
              {balance.toFixed(2)}
            </div>
            <div style={{ fontSize: '14px', color: '#6b7280', marginTop: '8px' }}>
              Network Growth Tokens
            </div>
          </div>
          <div style={{ display: 'flex', gap: '8px' }}>
            <button
              className="btn btn-secondary"
              onClick={() => handleExport('json')}
              title="Export as JSON"
            >
              <Download size={16} />
            </button>
            <button
              className="btn btn-secondary"
              onClick={() => handleExport('csv')}
              title="Export as CSV"
            >
              <Download size={16} />
            </button>
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
      </div>

      {/* Earnings Breakdown Card */}
      {Object.keys(rewardsByType).length > 0 && (
        <div className="card">
          <h3 className="card-title">Earnings Breakdown</h3>
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(2, 1fr)', gap: '16px' }}>
            {Object.entries(rewardsByType).map(([type, amount]) => (
              <div key={type}>
                <div style={{ fontSize: '13px', color: '#6b7280', marginBottom: '4px' }}>
                  {type}
                </div>
                <div style={{ fontSize: '20px', fontWeight: '600', color: '#22c55e' }}>
                  +{amount.toFixed(2)} NGT
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Reward History Card */}
      <div className="card">
        <h3 className="card-title">Reward History</h3>

        {loading && (
          <div className="loading">
            <div>Loading rewards...</div>
          </div>
        )}

        {!loading && rewards.length === 0 && (
          <div className="empty-state">
            <div className="empty-state-icon">💰</div>
            <div className="empty-state-text">
              No rewards yet. Start contributing to earn NGT!
            </div>
          </div>
        )}

        {!loading && rewards.length > 0 && (
          <div className="list">
            {rewards.map((reward) => (
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

      {/* Info Card */}
      <div className="card">
        <h3 className="card-title">About NGT Rewards</h3>
        <div style={{ fontSize: '14px', color: '#374151', lineHeight: '1.6' }}>
          <p style={{ marginBottom: '12px' }}>
            Network Growth Tokens (NGT) are earned through Proof-of-Emergence:
          </p>
          <ul style={{ paddingLeft: '20px', marginBottom: '12px' }}>
            <li style={{ marginBottom: '8px' }}>
              <strong>Novel Grains:</strong> 0.1-1.0 NGT per grain based on novelty
            </li>
            <li style={{ marginBottom: '8px' }}>
              <strong>Validations:</strong> 0.01-0.1 NGT per validation
            </li>
            <li style={{ marginBottom: '8px' }}>
              <strong>Query Answers:</strong> 0.05-0.5 NGT per answer
            </li>
            <li style={{ marginBottom: '8px' }}>
              <strong>Swarm Consensus:</strong> 0.1-2.0 NGT per participation
            </li>
          </ul>
          <p style={{ color: '#6b7280', fontSize: '13px' }}>
            Rewards are calculated automatically and distributed in real-time.
          </p>
        </div>
      </div>
    </div>
  );
}

export default RewardsScreen;
