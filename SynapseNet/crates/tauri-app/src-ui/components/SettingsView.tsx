import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface Config {
  node: {
    name: string;
    data_dir: string;
  };
  p2p: {
    enabled: boolean;
    port: number;
    mdns_enabled: boolean;
    bootstrap_peers: string[];
  };
  network: {
    dht_enabled: boolean;
    dht_k: number;
    relay_enabled: boolean;
    autonat_enabled: boolean;
    max_peers: number;
    clustering_enabled: boolean;
    cluster_threshold: number;
    bootstrap_nodes: string[];
  };
  ai: {
    model_name: string;
    embedding_dim: number;
    auto_download: boolean;
    provider: string;
    additional_models: any[];
    multi_model_enabled: boolean;
  };
  storage: {
    db_file: string;
    hnsw_max_elements: number;
    hnsw_m: number;
    hnsw_ef_construction: number;
  };
  economy: {
    poe_enabled: boolean;
    novelty_weight: number;
    coherence_weight: number;
    reuse_weight: number;
    min_novelty_threshold: number;
    track_access: boolean;
    access_retention_days: number;
  };
  ui: {
    theme: string;
    default_view: string;
    graph_enabled: boolean;
    graph_max_nodes: number;
    results_per_page: number;
    animations_enabled: boolean;
    show_poe_scores: boolean;
  };
}

export default function SettingsView() {
  const [config, setConfig] = useState<Config | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<'node' | 'network' | 'ai' | 'economy' | 'ui'>('node');

  useEffect(() => {
    loadConfig();
  }, []);

  const loadConfig = async () => {
    try {
      setLoading(true);
      setError(null);
      const cfg = await invoke<Config>('get_config');
      setConfig(cfg);
    } catch (err) {
      setError(`Failed to load configuration: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const saveConfig = async () => {
    if (!config) return;

    try {
      setSaving(true);
      setError(null);
      setSuccess(null);

      // Validate first
      await invoke('validate_config', { config });

      // Save
      await invoke('update_config', { newConfig: config });
      
      setSuccess('Configuration saved successfully!');
      setTimeout(() => setSuccess(null), 3000);
    } catch (err) {
      setError(`Failed to save configuration: ${err}`);
    } finally {
      setSaving(false);
    }
  };

  const resetConfig = async () => {
    if (!confirm('Are you sure you want to reset all settings to defaults?')) {
      return;
    }

    try {
      setSaving(true);
      setError(null);
      const defaultConfig = await invoke<Config>('reset_config');
      setConfig(defaultConfig);
      setSuccess('Configuration reset to defaults!');
      setTimeout(() => setSuccess(null), 3000);
    } catch (err) {
      setError(`Failed to reset configuration: ${err}`);
    } finally {
      setSaving(false);
    }
  };

  const updateConfig = (section: keyof Config, field: string, value: any) => {
    if (!config) return;
    setConfig({
      ...config,
      [section]: {
        ...config[section],
        [field]: value,
      },
    });
  };

  if (loading) {
    return (
      <div className="settings-view">
        <div className="loading">Loading configuration...</div>
      </div>
    );
  }

  if (!config) {
    return (
      <div className="settings-view">
        <div className="error">Failed to load configuration</div>
      </div>
    );
  }

  return (
    <div className="settings-view">
      <div className="settings-header">
        <h2>⚙️ Settings</h2>
        <div className="settings-actions">
          <button onClick={resetConfig} disabled={saving} className="btn-secondary">
            Reset to Defaults
          </button>
          <button onClick={saveConfig} disabled={saving} className="btn-primary">
            {saving ? 'Saving...' : 'Save Changes'}
          </button>
        </div>
      </div>

      {error && <div className="alert alert-error">{error}</div>}
      {success && <div className="alert alert-success">{success}</div>}

      <div className="settings-tabs">
        <button
          className={activeTab === 'node' ? 'tab-active' : ''}
          onClick={() => setActiveTab('node')}
        >
          Node
        </button>
        <button
          className={activeTab === 'network' ? 'tab-active' : ''}
          onClick={() => setActiveTab('network')}
        >
          Network
        </button>
        <button
          className={activeTab === 'ai' ? 'tab-active' : ''}
          onClick={() => setActiveTab('ai')}
        >
          AI/Models
        </button>
        <button
          className={activeTab === 'economy' ? 'tab-active' : ''}
          onClick={() => setActiveTab('economy')}
        >
          Economy
        </button>
        <button
          className={activeTab === 'ui' ? 'tab-active' : ''}
          onClick={() => setActiveTab('ui')}
        >
          UI
        </button>
      </div>

      <div className="settings-content">
        {activeTab === 'node' && (
          <div className="settings-section">
            <h3>Node Configuration</h3>
            
            <div className="form-group">
              <label>Node Name</label>
              <input
                type="text"
                value={config.node.name}
                onChange={(e) => updateConfig('node', 'name', e.target.value)}
              />
            </div>

            <div className="form-group">
              <label>Data Directory</label>
              <input
                type="text"
                value={config.node.data_dir}
                onChange={(e) => updateConfig('node', 'data_dir', e.target.value)}
              />
              <small>Location where grains and indexes are stored</small>
            </div>

            <h3>P2P Configuration</h3>
            
            <div className="form-group">
              <label>
                <input
                  type="checkbox"
                  checked={config.p2p.enabled}
                  onChange={(e) => updateConfig('p2p', 'enabled', e.target.checked)}
                />
                Enable P2P Networking
              </label>
            </div>

            <div className="form-group">
              <label>P2P Port</label>
              <input
                type="number"
                value={config.p2p.port}
                onChange={(e) => updateConfig('p2p', 'port', parseInt(e.target.value))}
              />
            </div>

            <div className="form-group">
              <label>
                <input
                  type="checkbox"
                  checked={config.p2p.mdns_enabled}
                  onChange={(e) => updateConfig('p2p', 'mdns_enabled', e.target.checked)}
                />
                Enable mDNS Discovery
              </label>
            </div>
          </div>
        )}

        {activeTab === 'network' && (
          <div className="settings-section">
            <h3>Advanced Network Settings</h3>
            
            <div className="form-group">
              <label>
                <input
                  type="checkbox"
                  checked={config.network.dht_enabled}
                  onChange={(e) => updateConfig('network', 'dht_enabled', e.target.checked)}
                />
                Enable DHT (Distributed Hash Table)
              </label>
            </div>

            <div className="form-group">
              <label>
                <input
                  type="checkbox"
                  checked={config.network.relay_enabled}
                  onChange={(e) => updateConfig('network', 'relay_enabled', e.target.checked)}
                />
                Enable Circuit Relay
              </label>
              <small>Helps with NAT traversal</small>
            </div>

            <div className="form-group">
              <label>
                <input
                  type="checkbox"
                  checked={config.network.clustering_enabled}
                  onChange={(e) => updateConfig('network', 'clustering_enabled', e.target.checked)}
                />
                Enable Peer Clustering
              </label>
            </div>

            <div className="form-group">
              <label>Maximum Peers</label>
              <input
                type="number"
                value={config.network.max_peers}
                onChange={(e) => updateConfig('network', 'max_peers', parseInt(e.target.value))}
              />
            </div>

            <div className="form-group">
              <label>Cluster Threshold (0.0-1.0)</label>
              <input
                type="number"
                step="0.1"
                min="0"
                max="1"
                value={config.network.cluster_threshold}
                onChange={(e) => updateConfig('network', 'cluster_threshold', parseFloat(e.target.value))}
              />
            </div>
          </div>
        )}

        {activeTab === 'ai' && (
          <div className="settings-section">
            <h3>AI/Embedding Configuration</h3>
            
            <div className="form-group">
              <label>Primary Model</label>
              <input
                type="text"
                value={config.ai.model_name}
                onChange={(e) => updateConfig('ai', 'model_name', e.target.value)}
              />
            </div>

            <div className="form-group">
              <label>Embedding Dimensions</label>
              <input
                type="number"
                value={config.ai.embedding_dim}
                onChange={(e) => updateConfig('ai', 'embedding_dim', parseInt(e.target.value))}
              />
            </div>

            <div className="form-group">
              <label>Provider</label>
              <select
                value={config.ai.provider}
                onChange={(e) => updateConfig('ai', 'provider', e.target.value)}
              >
                <option value="cpu">CPU</option>
                <option value="coreml">CoreML (macOS)</option>
                <option value="directml">DirectML (Windows)</option>
                <option value="cuda">CUDA (NVIDIA)</option>
              </select>
            </div>

            <div className="form-group">
              <label>
                <input
                  type="checkbox"
                  checked={config.ai.multi_model_enabled}
                  onChange={(e) => updateConfig('ai', 'multi_model_enabled', e.target.checked)}
                />
                Enable Multi-Model Support
              </label>
            </div>
          </div>
        )}

        {activeTab === 'economy' && (
          <div className="settings-section">
            <h3>Economy/PoE Configuration</h3>
            
            <div className="form-group">
              <label>
                <input
                  type="checkbox"
                  checked={config.economy.poe_enabled}
                  onChange={(e) => updateConfig('economy', 'poe_enabled', e.target.checked)}
                />
                Enable Proof of Emergence (PoE)
              </label>
            </div>

            <h4>Reward Weights (must sum to 1.0)</h4>
            
            <div className="form-group">
              <label>Novelty Weight</label>
              <input
                type="number"
                step="0.1"
                min="0"
                max="1"
                value={config.economy.novelty_weight}
                onChange={(e) => updateConfig('economy', 'novelty_weight', parseFloat(e.target.value))}
              />
            </div>

            <div className="form-group">
              <label>Coherence Weight</label>
              <input
                type="number"
                step="0.1"
                min="0"
                max="1"
                value={config.economy.coherence_weight}
                onChange={(e) => updateConfig('economy', 'coherence_weight', parseFloat(e.target.value))}
              />
            </div>

            <div className="form-group">
              <label>Reuse Weight</label>
              <input
                type="number"
                step="0.1"
                min="0"
                max="1"
                value={config.economy.reuse_weight}
                onChange={(e) => updateConfig('economy', 'reuse_weight', parseFloat(e.target.value))}
              />
            </div>

            <div className="form-group">
              <label>Minimum Novelty Threshold</label>
              <input
                type="number"
                step="0.1"
                min="0"
                max="1"
                value={config.economy.min_novelty_threshold}
                onChange={(e) => updateConfig('economy', 'min_novelty_threshold', parseFloat(e.target.value))}
              />
            </div>

            <div className="form-group">
              <label>
                <input
                  type="checkbox"
                  checked={config.economy.track_access}
                  onChange={(e) => updateConfig('economy', 'track_access', e.target.checked)}
                />
                Track Grain Access
              </label>
            </div>
          </div>
        )}

        {activeTab === 'ui' && (
          <div className="settings-section">
            <h3>UI Configuration</h3>
            
            <div className="form-group">
              <label>Theme</label>
              <select
                value={config.ui.theme}
                onChange={(e) => updateConfig('ui', 'theme', e.target.value)}
              >
                <option value="light">Light</option>
                <option value="dark">Dark</option>
                <option value="auto">Auto</option>
              </select>
            </div>

            <div className="form-group">
              <label>Default View</label>
              <select
                value={config.ui.default_view}
                onChange={(e) => updateConfig('ui', 'default_view', e.target.value)}
              >
                <option value="search">Search</option>
                <option value="add">Add Grain</option>
                <option value="graph">Graph</option>
                <option value="stats">Stats</option>
              </select>
            </div>

            <div className="form-group">
              <label>
                <input
                  type="checkbox"
                  checked={config.ui.graph_enabled}
                  onChange={(e) => updateConfig('ui', 'graph_enabled', e.target.checked)}
                />
                Enable Graph Visualization
              </label>
            </div>

            <div className="form-group">
              <label>Graph Max Nodes</label>
              <input
                type="number"
                value={config.ui.graph_max_nodes}
                onChange={(e) => updateConfig('ui', 'graph_max_nodes', parseInt(e.target.value))}
              />
            </div>

            <div className="form-group">
              <label>Results Per Page</label>
              <input
                type="number"
                value={config.ui.results_per_page}
                onChange={(e) => updateConfig('ui', 'results_per_page', parseInt(e.target.value))}
              />
            </div>

            <div className="form-group">
              <label>
                <input
                  type="checkbox"
                  checked={config.ui.show_poe_scores}
                  onChange={(e) => updateConfig('ui', 'show_poe_scores', e.target.checked)}
                />
                Show PoE Scores
              </label>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
