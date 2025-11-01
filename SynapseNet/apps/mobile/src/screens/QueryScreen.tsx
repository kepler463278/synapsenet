import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface QueryResult {
  id: string;
  text: string;
  similarity: f32;
  tags: string[];
}

function QueryScreen() {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<QueryResult[]>([]);
  const [loading, setLoading] = useState(false);

  const handleSearch = async () => {
    if (!query.trim()) return;

    setLoading(true);
    try {
      const data = await invoke<QueryResult[]>('syn_query', {
        query,
        k: 10
      });
      setResults(data);
    } catch (error) {
      console.error('Search failed:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="screen query-screen">
      <header>
        <h2>🔍 Search Knowledge</h2>
      </header>

      <div className="search-box">
        <input
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
          placeholder="Search your knowledge..."
          disabled={loading}
        />
        <button onClick={handleSearch} disabled={loading || !query.trim()}>
          {loading ? '⏳' : '🔍'}
        </button>
      </div>

      <div className="results">
        {results.length === 0 && !loading && (
          <div className="empty-state">
            <p>No results yet. Try searching!</p>
          </div>
        )}

        {results.map((result) => (
          <div 
            key={result.id} 
            className="result-card"
            role="article"
            aria-label={`Result with ${(result.similarity * 100).toFixed(0)}% similarity. ${result.text}. Tags: ${result.tags.join(', ')}`}
            tabIndex={0}
          >
            <div 
              className="result-similarity"
              aria-label={`${(result.similarity * 100).toFixed(0)} percent similarity`}
            >
              {(result.similarity * 100).toFixed(0)}%
            </div>
            <div className="result-text">{result.text}</div>
            <div className="result-tags" role="list" aria-label="Tags">
              {result.tags.map((tag) => (
                <span 
                  key={tag} 
                  className="tag"
                  role="listitem"
                  aria-label={`Tag: ${tag}`}
                >
                  {tag}
                </span>
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

export default QueryScreen;
