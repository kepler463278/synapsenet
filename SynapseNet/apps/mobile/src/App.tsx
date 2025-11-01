import { useState } from 'react';
import HomeScreen from './screens/HomeScreen';
import AddScreen from './screens/AddScreen';
import QueryScreen from './screens/QueryScreen';
import PeersScreen from './screens/PeersScreen';
import WalletScreen from './screens/WalletScreen';
import SettingsScreen from './screens/SettingsScreen';

type Screen = 'home' | 'add' | 'query' | 'peers' | 'wallet' | 'settings';

function App() {
  const [currentScreen, setCurrentScreen] = useState<Screen>('home');

  const renderScreen = () => {
    switch (currentScreen) {
      case 'home':
        return <HomeScreen />;
      case 'add':
        return <AddScreen />;
      case 'query':
        return <QueryScreen />;
      case 'peers':
        return <PeersScreen />;
      case 'wallet':
        return <WalletScreen />;
      case 'settings':
        return <SettingsScreen />;
    }
  };

  return (
    <div className="app">
      <div className="screen-container">
        {renderScreen()}
      </div>
      
      <nav className="bottom-nav">
        <button 
          className={currentScreen === 'home' ? 'active' : ''}
          onClick={() => setCurrentScreen('home')}
        >
          🏠 Home
        </button>
        <button 
          className={currentScreen === 'add' ? 'active' : ''}
          onClick={() => setCurrentScreen('add')}
        >
          ➕ Add
        </button>
        <button 
          className={currentScreen === 'query' ? 'active' : ''}
          onClick={() => setCurrentScreen('query')}
        >
          🔍 Search
        </button>
        <button 
          className={currentScreen === 'peers' ? 'active' : ''}
          onClick={() => setCurrentScreen('peers')}
        >
          🌐 Peers
        </button>
        <button 
          className={currentScreen === 'wallet' ? 'active' : ''}
          onClick={() => setCurrentScreen('wallet')}
        >
          💰 Wallet
        </button>
        <button 
          className={currentScreen === 'settings' ? 'active' : ''}
          onClick={() => setCurrentScreen('settings')}
        >
          ⚙️ Settings
        </button>
      </nav>
    </div>
  );
}

export default App;
