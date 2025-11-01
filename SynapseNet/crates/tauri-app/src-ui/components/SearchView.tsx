import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface SearchResult {
  grain_id: string;
  similarity: number;
  title: string | null;
  summary: string | null;
  tags: string[];
  timestamp: number;
}

function SearchView() {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [searchTime, setSearchTime] = useState<number | null>(null);

  const handleSearch = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!query.trim()) {
      setError('Query cannot be empty');
      return;
    }

    setLoading(true);
    setError(null);
    const start = Date.now();

    try {
      const searchResults = await invoke<SearchResult[]>('search_grains', {
        query: query.trim(),
        k: 10,
      });

      setResults(searchResults);
      setSearchTime(Date.now() - start);
    } catch (err) {
      setError(`Search failed: ${err}`);
      setResults([]);
    } finally {
      setLoading(false);
    }
  };

  const formatDate = (timestamp: number) => {
    return new Date(timestamp).toLocaleString();
  };

  return (
    <div className="view-container">
      <h2>Semantic Search</h2>
      <p className="subtitle">Search your knowledge by meaning, not just keywords</p>

      <form onSubmit={handleSearch} className="search-form">
        <div className="search-input-group">
          <input
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="What are you looking for?"
            disabled={loading}
          />
          <button type="submit" className="btn-primary" disabled={loading}>
            {loading ? '⏳' : '🔍'} Search
          </button>
        </div>
      </form>

      {error && <div className="message error">{error}</div>}

      {searchTime !== null && (
        <div className="search-meta">
          Found {results.length} results in {searchTime}ms
        </div>
      )}

      <div className="results-list">
        {results.map((result) => (
          <div key={result.grain_id} className="result-card">
            <div className="result-header">
              <h3>{result.title || 'Untitled'}</h3>
              <span className="similarity-badge">
                {(result.similarity * 100).toFixed(1)}% match
              </span>
            </div>

            {result.summary && (
              <p className="result-summary">{result.summary}</p>
            )}

            <div className="result-footer">
              <div className="result-tags">
                {result.tags.map((tag) => (
                  <span key={tag} className="tag">
                    {tag}
                  </span>
                ))}
              </div>
              <span className="result-date">{formatDate(result.timestamp)}</span>
            </div>

            <div className="result-id">ID: {result.grain_id.substring(0, 16)}...</div>
          </div>
        ))}

        {results.length === 0 && !loading && !error && query && (
          <div className="no-results">
            <p>No results found. Try a different query.</p>
          </div>
        )}
      </div>
    </div>
  );
}

export default SearchView;
