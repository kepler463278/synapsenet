import { useState } from 'react';
import NotificationSettings from '../components/NotificationSettings';
import AccessibilitySettings from '../components/AccessibilitySettings';

function SettingsScreen() {
  const [airGapMode, setAirGapMode] = useState(false);
  const [biometric, setBiometric] = useState(true);

  return (
    <div className="screen settings-screen">
      <header>
        <h2>⚙️ Settings</h2>
      </header>

      <NotificationSettings />

      <AccessibilitySettings />

      <div className="settings-section">
        <h3>Privacy</h3>
        
        <div className="setting-item">
          <div className="setting-info">
            <div className="setting-label">Air-gap Mode</div>
            <div className="setting-desc">Disable all network connections</div>
          </div>
          <label className="toggle">
            <input
              type="checkbox"
              checked={airGapMode}
              onChange={(e) => setAirGapMode(e.target.checked)}
            />
            <span className="slider"></span>
          </label>
        </div>

        <div className="setting-item">
          <div className="setting-info">
            <div className="setting-label">Biometric Auth</div>
            <div className="setting-desc">Use Face ID / Fingerprint</div>
          </div>
          <label className="toggle">
            <input
              type="checkbox"
              checked={biometric}
              onChange={(e) => setBiometric(e.target.checked)}
            />
            <span className="slider"></span>
          </label>
        </div>
      </div>

      <div className="settings-section">
        <h3>AI Model</h3>
        <div className="setting-item">
          <div className="setting-info">
            <div className="setting-label">Current Model</div>
            <div className="setting-desc">all-MiniLM-L6-v2 (384-dim)</div>
          </div>
          <button className="btn-secondary">Change</button>
        </div>
      </div>

      <div className="settings-section">
        <h3>Backup</h3>
        <button className="btn-primary full-width">📤 Export Data</button>
        <button className="btn-secondary full-width">📥 Import Data</button>
        <button className="btn-secondary full-width">🔑 Show Recovery Phrase</button>
      </div>

      <div className="settings-section">
        <h3>About</h3>
        <div className="about-info">
          <p><strong>Version:</strong> 0.5.0</p>
          <p><strong>Build:</strong> Mobile Emergence</p>
          <p><strong>License:</strong> MIT / Apache-2.0</p>
        </div>
      </div>
    </div>
  );
}

export default SettingsScreen;
