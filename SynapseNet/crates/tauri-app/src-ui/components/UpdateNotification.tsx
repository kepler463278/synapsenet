import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface UpdateInfo {
  available: boolean;
  version: string;
  current_version: string;
  download_url?: string;
  release_notes?: string;
}

export const UpdateNotification: React.FC = () => {
  const [updateInfo, setUpdateInfo] = useState<UpdateInfo | null>(null);
  const [checking, setChecking] = useState(false);
  const [installing, setInstalling] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [dismissed, setDismissed] = useState(false);

  const checkForUpdates = async () => {
    setChecking(true);
    setError(null);
    
    try {
      const info = await invoke<UpdateInfo>('check_for_updates');
      setUpdateInfo(info);
      
      // Auto-check on mount, but don't show if no update
      if (!info.available) {
        setDismissed(true);
      }
    } catch (err) {
      console.error('Failed to check for updates:', err);
      setError(err instanceof Error ? err.message : 'Failed to check for updates');
    } finally {
      setChecking(false);
    }
  };

  const installUpdate = async () => {
    setInstalling(true);
    setError(null);
    
    try {
      await invoke('install_update');
      // App will restart after update
    } catch (err) {
      console.error('Failed to install update:', err);
      setError(err instanceof Error ? err.message : 'Failed to install update');
      setInstalling(false);
    }
  };

  useEffect(() => {
    // Check for updates on mount
    checkForUpdates();
    
    // Check every 6 hours
    const interval = setInterval(checkForUpdates, 6 * 60 * 60 * 1000);
    
    return () => clearInterval(interval);
  }, []);

  if (dismissed || !updateInfo || !updateInfo.available) {
    return null;
  }

  return (
    <div className="update-notification">
      <div className="update-content">
        <div className="update-icon">🎉</div>
        <div className="update-text">
          <h3>Update Available</h3>
          <p>
            Version {updateInfo.version} is now available. You're currently on version{' '}
            {updateInfo.current_version}.
          </p>
          {updateInfo.release_notes && (
            <details className="release-notes">
              <summary>What's new?</summary>
              <div className="release-notes-content">
                {updateInfo.release_notes}
              </div>
            </details>
          )}
          {error && (
            <div className="error-message">
              ⚠️ {error}
            </div>
          )}
        </div>
        <div className="update-actions">
          <button
            onClick={installUpdate}
            disabled={installing || checking}
            className="btn-primary"
          >
            {installing ? 'Installing...' : 'Update Now'}
          </button>
          <button
            onClick={() => setDismissed(true)}
            disabled={installing}
            className="btn-secondary"
          >
            Later
          </button>
        </div>
      </div>
      
      <style>{`
        .update-notification {
          position: fixed;
          top: 20px;
          right: 20px;
          max-width: 400px;
          background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
          color: white;
          border-radius: 12px;
          box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
          padding: 20px;
          z-index: 1000;
          animation: slideIn 0.3s ease-out;
        }
        
        @keyframes slideIn {
          from {
            transform: translateX(100%);
            opacity: 0;
          }
          to {
            transform: translateX(0);
            opacity: 1;
          }
        }
        
        .update-content {
          display: flex;
          flex-direction: column;
          gap: 15px;
        }
        
        .update-icon {
          font-size: 32px;
          text-align: center;
        }
        
        .update-text h3 {
          margin: 0 0 8px 0;
          font-size: 18px;
          font-weight: 600;
        }
        
        .update-text p {
          margin: 0;
          font-size: 14px;
          opacity: 0.95;
          line-height: 1.5;
        }
        
        .release-notes {
          margin-top: 10px;
          font-size: 13px;
        }
        
        .release-notes summary {
          cursor: pointer;
          font-weight: 500;
          padding: 5px 0;
          user-select: none;
        }
        
        .release-notes summary:hover {
          opacity: 0.8;
        }
        
        .release-notes-content {
          margin-top: 8px;
          padding: 10px;
          background: rgba(255, 255, 255, 0.1);
          border-radius: 6px;
          max-height: 150px;
          overflow-y: auto;
          white-space: pre-wrap;
        }
        
        .error-message {
          margin-top: 10px;
          padding: 8px;
          background: rgba(255, 0, 0, 0.2);
          border-radius: 4px;
          font-size: 13px;
        }
        
        .update-actions {
          display: flex;
          gap: 10px;
          margin-top: 5px;
        }
        
        .update-actions button {
          flex: 1;
          padding: 10px 16px;
          border: none;
          border-radius: 6px;
          font-size: 14px;
          font-weight: 500;
          cursor: pointer;
          transition: all 0.2s;
        }
        
        .update-actions button:disabled {
          opacity: 0.6;
          cursor: not-allowed;
        }
        
        .btn-primary {
          background: white;
          color: #667eea;
        }
        
        .btn-primary:hover:not(:disabled) {
          transform: translateY(-1px);
          box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
        }
        
        .btn-secondary {
          background: rgba(255, 255, 255, 0.2);
          color: white;
        }
        
        .btn-secondary:hover:not(:disabled) {
          background: rgba(255, 255, 255, 0.3);
        }
      `}</style>
    </div>
  );
};
