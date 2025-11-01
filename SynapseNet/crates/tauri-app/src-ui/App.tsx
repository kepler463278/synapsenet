import { useState } from 'react';
import AddGrainView from './components/AddGrainView';
import SearchView from './components/SearchView';
import StatsView from './components/StatsView';
import GraphView from './components/GraphView';
import SettingsView from './components/SettingsView';
import { UpdateNotification } from './components/UpdateNotification';

type View = 'add' | 'search' | 'graph' | 'stats' | 'settings';

function App() {
  const [currentView, setCurrentView] = useState<View>('add');

  return (
    <div className="app">
      <UpdateNotification />
      <nav className="navbar">
        <div className="nav-brand">
          <h1>🧠 SynapseNet</h1>
          <span className="version">v0.4.0</span>
        </div>
        <div className="nav-links">
          <button
            className={currentView === 'add' ? 'active' : ''}
            onClick={() => setCurrentView('add')}
          >
            ➕ Add
          </button>
          <button
            className={currentView === 'search' ? 'active' : ''}
            onClick={() => setCurrentView('search')}
          >
            🔍 Search
          </button>
          <button
            className={currentView === 'graph' ? 'active' : ''}
            onClick={() => setCurrentView('graph')}
          >
            🕸️ Graph
          </button>
          <button
            className={currentView === 'stats' ? 'active' : ''}
            onClick={() => setCurrentView('stats')}
          >
            📊 Stats
          </button>
          <button
            className={currentView === 'settings' ? 'active' : ''}
            onClick={() => setCurrentView('settings')}
          >
            ⚙️ Settings
          </button>
        </div>
      </nav>

      <main className="main-content">
        {currentView === 'add' && <AddGrainView />}
        {currentView === 'search' && <SearchView />}
        {currentView === 'graph' && <GraphView />}
        {currentView === 'stats' && <StatsView />}
        {currentView === 'settings' && <SettingsView />}
      </main>
    </div>
  );
}

export default App;
