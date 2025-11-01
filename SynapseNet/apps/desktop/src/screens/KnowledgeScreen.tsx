import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Search } from 'lucide-react';

interface Grain {
  id: string;
  content: string;
  author: string;
  timestamp: number;
  confidence: number;
}

function KnowledgeScreen() {
  const [query, setQuery] = useState('');
  const [grains, setGrains] = useState<Grain[]>([]);
  const [loading, setLoading] = useState(false);
  const [searched, setSearched] = useState(false);

  const handleSearch = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!query.trim()) {
      return;
    }

    setLoading(true);
    setSearched(true);

    try {
      const results = await invoke<Grain[]>('search_grains', {
        query: query.trim(),
        limit: 20,
      });
      setGrains(results);
    } catch (error) {
      console.error('Search failed:', error);
      alert('Search failed: ' + error);
    } finally {
      setLoading(false);
    }
  };

  const formatTimestamp = (timestamp: number) => {
    const date = new Date(timestamp * 1000);
    return date.toLocaleString();
  };

  const getSourceBadge = (author: string) => {
    if (author === 'local') {
      return (
        <span
          style={{
            padding: '2px 8px',
            borderRadius: '4px',
            fontSize: '11px',
            fontWeight: '500',
            background: '#dbeafe',
            color: '#1e40af',
          }}
        >
          Local
        </span>
      );
    }
    return (
      <span
        style={{
          padding: '2px 8px',
          borderRadius: '4px',
          fontSize: '11px',
          fontWeight: '500',
          background: '#f3e8ff',
          color: '#6b21a8',
        }}
      >
        Network
      </span>
    );
  };

  return (
    <div className="screen">
      <div className="card">
        <h2 className="card-title">
          <Search size={20} style={{ display: 'inline', marginRight: '8px' }} />
          Search Knowledge
        </h2>

        <form onSubmit={handleSearch}>
          <div className="flex gap-2">
            <input
              type="text"
              className="input"
              placeholder="Search across the network..."
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              style={{ flex: 1 }}
            />
            <button type="submit" className="btn btn-primary" disabled={loading}>
              {loading ? 'Searching...' : 'Search'}
            </button>
          </div>
        </form>
      </div>

      {loading && (
        <div className="loading">
          <div>Searching network...</div>
        </div>
      )}

      {!loading && searched && grains.length === 0 && (
        <div className="card">
          <div className="empty-state">
            <div className="empty-state-icon">🔍</div>
            <div className="empty-state-text">No results found</div>
          </div>
        </div>
      )}

      {!loading && grains.length > 0 && (
        <div className="card">
          <h3 className="card-title">
            Results ({grains.length})
          </h3>

          <div>
            {grains.map((grain) => (
              <div key={grain.id} className="grain-item">
                <div className="grain-header">
                  <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                    <span className="grain-author">{grain.author}</span>
                    {getSourceBadge(grain.author)}
                  </div>
                  <span className="grain-confidence">
                    {(grain.confidence * 100).toFixed(0)}% confidence
                  </span>
                </div>
                <div className="grain-content">{grain.content}</div>
                <div className="grain-footer">{formatTimestamp(grain.timestamp)}</div>
              </div>
            ))}
          </div>
        </div>
      )}

      {!searched && (
        <div className="card">
          <div className="empty-state">
            <div className="empty-state-icon">💡</div>
            <div className="empty-state-text">
              Enter a query to search the knowledge network
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default KnowledgeScreen;
