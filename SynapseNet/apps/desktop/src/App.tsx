import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { Activity, Database, Coins } from 'lucide-react';

import HomeScreen from './screens/HomeScreen';
import KnowledgeScreen from './screens/KnowledgeScreen';
import AddScreen from './screens/AddScreen';
import RewardsScreen from './screens/RewardsScreen';
import SettingsScreen from './screens/SettingsScreen';

type Screen = 'home' | 'knowledge' | 'add' | 'rewards' | 'settings';

interface NodeStatus {
  running: boolean;
  peers: number;
  grains: number;
  uptime: number;
}

interface AppEvent {
  type: string;
  peers?: number;
  grains?: number;
  amount?: number;
  reason?: string;
  running?: boolean;
}

function App() {
  const [activeScreen, setActiveScreen] = useState<Screen>('home');
  const [nodeStatus, setNodeStatus] = useState<NodeStatus | null>(null);
  const [balance, setBalance] = useState<number>(0);
  const [notification, setNotification] = useState<{
    title: string;
    message: string;
  } | null>(null);

  useEffect(() => {
    // Load initial data
    loadNodeStatus();
    loadBalance();

    // Listen for real-time events
    const unlisten = listen<AppEvent>('app-event', (event) => {
      const payload = event.payload;

      switch (payload.type) {
        case 'NetworkUpdate':
          loadNodeStatus();
          break;
        case 'RewardEarned':
          loadBalance();
          showNotification(
            '🎉 Reward Earned!',
            `+${payload.amount?.toFixed(2)} NGT - ${payload.reason}`
          );
          break;
        case 'NodeStatusChanged':
          loadNodeStatus();
          break;
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const loadNodeStatus = async () => {
    try {
      const status = await invoke<NodeStatus>('get_node_status');
      setNodeStatus(status);
    } catch (error) {
      console.error('Failed to load node status:', error);
    }
  };

  const loadBalance = async () => {
    try {
      const bal = await invoke<number>('get_balance');
      setBalance(bal);
    } catch (error) {
      console.error('Failed to load balance:', error);
    }
  };

  const showNotification = (title: string, message: string) => {
    setNotification({ title, message });
    setTimeout(() => setNotification(null), 5000);
  };

  return (
    <div className="app">
      {/* Header */}
      <header className="header">
        <div className="header-title">
          <div className="logo">S</div>
          <h1>SynapseNet v1.0</h1>
        </div>
        <div className="header-stats">
          <div className="stat">
            <Activity size={16} />
            <span className="stat-value">{nodeStatus?.peers || 0}</span>
            <span>Peers</span>
          </div>
          <div className="stat">
            <Database size={16} />
            <span className="stat-value">{nodeStatus?.grains || 0}</span>
            <span>Grains</span>
          </div>
          <div className="stat">
            <Coins size={16} />
            <span className="stat-value">{balance.toFixed(2)}</span>
            <span>NGT</span>
          </div>
        </div>
      </header>

      {/* Navigation */}
      <nav className="nav">
        <button
          className={`nav-tab ${activeScreen === 'home' ? 'active' : ''}`}
          onClick={() => setActiveScreen('home')}
        >
          Home
        </button>
        <button
          className={`nav-tab ${activeScreen === 'knowledge' ? 'active' : ''}`}
          onClick={() => setActiveScreen('knowledge')}
        >
          Knowledge
        </button>
        <button
          className={`nav-tab ${activeScreen === 'add' ? 'active' : ''}`}
          onClick={() => setActiveScreen('add')}
        >
          Add
        </button>
        <button
          className={`nav-tab ${activeScreen === 'rewards' ? 'active' : ''}`}
          onClick={() => setActiveScreen('rewards')}
        >
          Rewards
        </button>
        <button
          className={`nav-tab ${activeScreen === 'settings' ? 'active' : ''}`}
          onClick={() => setActiveScreen('settings')}
        >
          Settings
        </button>
      </nav>

      {/* Main Content */}
      <main className="main">
        {activeScreen === 'home' && (
          <HomeScreen
            nodeStatus={nodeStatus}
            balance={balance}
            onRefresh={loadNodeStatus}
          />
        )}
        {activeScreen === 'knowledge' && <KnowledgeScreen />}
        {activeScreen === 'add' && (
          <AddScreen onGrainAdded={() => {
            loadNodeStatus();
            loadBalance();
          }} />
        )}
        {activeScreen === 'rewards' && <RewardsScreen balance={balance} />}
        {activeScreen === 'settings' && <SettingsScreen />}
      </main>

      {/* Notification */}
      {notification && (
        <div className="notification">
          <div className="notification-header">
            <div className="notification-title">{notification.title}</div>
            <button
              className="notification-close"
              onClick={() => setNotification(null)}
            >
              ✕
            </button>
          </div>
          <div className="notification-body">{notification.message}</div>
        </div>
      )}
    </div>
  );
}

export default App;
