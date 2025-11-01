import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface NotificationSettings {
  enabled: boolean;
  sync_notifications: boolean;
  peer_notifications: boolean;
  reward_notifications: boolean;
  reminder_notifications: boolean;
}

function NotificationSettings() {
  const [settings, setSettings] = useState<NotificationSettings>({
    enabled: true,
    sync_notifications: true,
    peer_notifications: true,
    reward_notifications: true,
    reminder_notifications: true,
  });
  const [permissionGranted, setPermissionGranted] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadSettings();
  }, []);

  const loadSettings = async () => {
    try {
      const data = await invoke<NotificationSettings>('get_notification_settings');
      setSettings(data);
    } catch (error) {
      console.error('Failed to load notification settings:', error);
    } finally {
      setLoading(false);
    }
  };

  const requestPermission = async () => {
    try {
      const granted = await invoke<boolean>('request_notification_permission');
      setPermissionGranted(granted);
      
      if (granted) {
        alert('✅ Notification permission granted!');
      } else {
        alert('❌ Notification permission denied');
      }
    } catch (error) {
      console.error('Failed to request permission:', error);
      alert('Failed to request notification permission');
    }
  };

  const updateSetting = async (key: keyof NotificationSettings, value: boolean) => {
    const newSettings = { ...settings, [key]: value };
    setSettings(newSettings);
    
    try {
      await invoke('update_notification_settings', { settings: newSettings });
    } catch (error) {
      console.error('Failed to update settings:', error);
      // Revert on error
      setSettings(settings);
    }
  };

  const sendTestNotification = async () => {
    try {
      await invoke('send_notification', {
        notification: {
          id: 'test_' + Date.now(),
          title: 'Test Notification',
          body: 'This is a test notification from SynapseNet',
          category: 'custom',
          data: {},
        },
      });
      alert('Test notification sent!');
    } catch (error) {
      console.error('Failed to send test notification:', error);
      alert('Failed to send test notification');
    }
  };

  if (loading) {
    return <div className="loading">Loading settings...</div>;
  }

  return (
    <div className="notification-settings">
      <div className="settings-header">
        <h3>🔔 Notifications</h3>
        <p>Manage your notification preferences</p>
      </div>

      {!permissionGranted && (
        <div className="permission-banner">
          <p>📱 Enable notifications to stay updated</p>
          <button className="btn-primary" onClick={requestPermission}>
            Enable Notifications
          </button>
        </div>
      )}

      <div className="setting-group">
        <div className="setting-item">
          <div>
            <div className="setting-label">Enable Notifications</div>
            <div className="setting-desc">Master switch for all notifications</div>
          </div>
          <label className="toggle">
            <input
              type="checkbox"
              checked={settings.enabled}
              onChange={(e) => updateSetting('enabled', e.target.checked)}
            />
            <span className="slider"></span>
          </label>
        </div>

        <div className="setting-item">
          <div>
            <div className="setting-label">Sync Notifications</div>
            <div className="setting-desc">Get notified when sync completes</div>
          </div>
          <label className="toggle">
            <input
              type="checkbox"
              checked={settings.sync_notifications}
              onChange={(e) => updateSetting('sync_notifications', e.target.checked)}
              disabled={!settings.enabled}
            />
            <span className="slider"></span>
          </label>
        </div>

        <div className="setting-item">
          <div>
            <div className="setting-label">Peer Notifications</div>
            <div className="setting-desc">Get notified about new peer connections</div>
          </div>
          <label className="toggle">
            <input
              type="checkbox"
              checked={settings.peer_notifications}
              onChange={(e) => updateSetting('peer_notifications', e.target.checked)}
              disabled={!settings.enabled}
            />
            <span className="slider"></span>
          </label>
        </div>

        <div className="setting-item">
          <div>
            <div className="setting-label">Reward Notifications</div>
            <div className="setting-desc">Get notified when you earn rewards</div>
          </div>
          <label className="toggle">
            <input
              type="checkbox"
              checked={settings.reward_notifications}
              onChange={(e) => updateSetting('reward_notifications', e.target.checked)}
              disabled={!settings.enabled}
            />
            <span className="slider"></span>
          </label>
        </div>

        <div className="setting-item">
          <div>
            <div className="setting-label">Reminder Notifications</div>
            <div className="setting-desc">Get reminders for backups and updates</div>
          </div>
          <label className="toggle">
            <input
              type="checkbox"
              checked={settings.reminder_notifications}
              onChange={(e) => updateSetting('reminder_notifications', e.target.checked)}
              disabled={!settings.enabled}
            />
            <span className="slider"></span>
          </label>
        </div>
      </div>

      <button 
        className="btn-secondary full-width"
        onClick={sendTestNotification}
        disabled={!settings.enabled}
      >
        Send Test Notification
      </button>
    </div>
  );
}

export default NotificationSettings;
