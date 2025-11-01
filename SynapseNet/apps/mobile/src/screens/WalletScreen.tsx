import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface WalletInfo {
  balance: number;
  pending_rewards: number;
  total_earned: number;
  grain_count: number;
  avg_poe_score: number;
}

interface RewardHistory {
  grain_id: string;
  timestamp: number;
  score: {
    novelty: number;
    coherence: number;
    reuse_count: number;
    total_score: number;
    ngt_reward: number;
  };
  synced: boolean;
}

interface PoEBreakdown {
  novelty_contribution: number;
  coherence_contribution: number;
  reuse_bonus: number;
  total_ngt: number;
}

function WalletScreen() {
  const [wallet, setWallet] = useState<WalletInfo | null>(null);
  const [history, setHistory] = useState<RewardHistory[]>([]);
  const [selectedGrain, setSelectedGrain] = useState<string | null>(null);
  const [breakdown, setBreakdown] = useState<PoEBreakdown | null>(null);
  const [loading, setLoading] = useState(true);
  const [syncing, setSyncing] = useState(false);

  useEffect(() => {
    loadWalletData();
  }, []);

  const loadWalletData = async () => {
    try {
      const [walletData, historyData] = await Promise.all([
        invoke<WalletInfo>('get_wallet_info'),
        invoke<RewardHistory[]>('get_reward_history', { limit: 20 }),
      ]);
      
      setWallet(walletData);
      setHistory(historyData);
    } catch (error) {
      console.error('Failed to load wallet data:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleSync = async () => {
    setSyncing(true);
    try {
      const result = await invoke('sync_rewards');
      console.log('Sync result:', result);
      await loadWalletData();
      alert('✅ Rewards synced successfully!');
    } catch (error) {
      console.error('Sync failed:', error);
      alert('❌ Failed to sync rewards');
    } finally {
      setSyncing(false);
    }
  };

  const handleExport = async () => {
    try {
      const data = await invoke<string>('export_rewards');
      // TODO: Save to file or share
      console.log('Export data:', data);
      alert('✅ Rewards exported!');
    } catch (error) {
      console.error('Export failed:', error);
      alert('❌ Failed to export rewards');
    }
  };

  const showBreakdown = async (grainId: string) => {
    try {
      const data = await invoke<PoEBreakdown>('get_poe_breakdown', { grainId });
      setBreakdown(data);
      setSelectedGrain(grainId);
    } catch (error) {
      console.error('Failed to load breakdown:', error);
    }
  };

  const formatDate = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleDateString();
  };

  if (loading) {
    return <div className="screen loading">Loading wallet...</div>;
  }

  return (
    <div className="screen wallet-screen">
      <header>
        <h2>💰 Wallet</h2>
      </header>

      {wallet && (
        <>
          <div className="balance-card">
            <div className="balance-label">NGT Balance</div>
            <div className="balance-amount">{wallet.balance.toFixed(2)}</div>
            <div className="balance-usd">
              {wallet.pending_rewards > 0 && (
                <span>+{wallet.pending_rewards.toFixed(2)} pending</span>
              )}
            </div>
          </div>

          <div className="wallet-stats">
            <div className="stat-item">
              <div className="stat-value">{wallet.total_earned.toFixed(2)}</div>
              <div className="stat-label">Total Earned</div>
            </div>
            <div className="stat-item">
              <div className="stat-value">{wallet.grain_count}</div>
              <div className="stat-label">Grains</div>
            </div>
            <div className="stat-item">
              <div className="stat-value">{(wallet.avg_poe_score * 100).toFixed(0)}%</div>
              <div className="stat-label">Avg PoE Score</div>
            </div>
          </div>

          <div className="wallet-actions">
            <button 
              className="btn-primary"
              onClick={handleSync}
              disabled={syncing}
            >
              {syncing ? '⏳ Syncing...' : '🔄 Sync Rewards'}
            </button>
            <button 
              className="btn-secondary"
              onClick={handleExport}
            >
              📤 Export
            </button>
          </div>
        </>
      )}

      <div className="rewards-section">
        <h3>Reward History</h3>
        
        {history.length === 0 ? (
          <div className="empty-state">
            <p>No rewards yet</p>
            <p className="hint">Add grains to start earning NGT!</p>
          </div>
        ) : (
          <div className="reward-list">
            {history.map((reward) => (
              <div 
                key={reward.grain_id} 
                className="reward-item"
                onClick={() => showBreakdown(reward.grain_id)}
              >
                <div className="reward-header">
                  <div className="reward-id">
                    {reward.grain_id.substring(0, 12)}...
                  </div>
                  <div className="reward-amount">
                    +{reward.score.ngt_reward.toFixed(2)} NGT
                  </div>
                </div>
                
                <div className="reward-details">
                  <div className="reward-date">{formatDate(reward.timestamp)}</div>
                  <div className="reward-scores">
                    <span className="score-badge">
                      N: {(reward.score.novelty * 100).toFixed(0)}%
                    </span>
                    <span className="score-badge">
                      C: {(reward.score.coherence * 100).toFixed(0)}%
                    </span>
                    {reward.score.reuse_count > 0 && (
                      <span className="score-badge reuse">
                        ♻️ {reward.score.reuse_count}
                      </span>
                    )}
                  </div>
                  <div className="sync-status">
                    {reward.synced ? '✅ Synced' : '⏳ Pending'}
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {breakdown && selectedGrain && (
        <div className="breakdown-modal" onClick={() => setBreakdown(null)}>
          <div className="breakdown-content" onClick={(e) => e.stopPropagation()}>
            <h3>PoE Breakdown</h3>
            <div className="breakdown-item">
              <span>Novelty Contribution</span>
              <span>{breakdown.novelty_contribution.toFixed(2)} NGT</span>
            </div>
            <div className="breakdown-item">
              <span>Coherence Contribution</span>
              <span>{breakdown.coherence_contribution.toFixed(2)} NGT</span>
            </div>
            <div className="breakdown-item">
              <span>Reuse Bonus</span>
              <span>{breakdown.reuse_bonus.toFixed(2)} NGT</span>
            </div>
            <div className="breakdown-total">
              <span>Total</span>
              <span>{breakdown.total_ngt.toFixed(2)} NGT</span>
            </div>
            <button className="btn-primary" onClick={() => setBreakdown(null)}>
              Close
            </button>
          </div>
        </div>
      )}
    </div>
  );
}

export default WalletScreen;
