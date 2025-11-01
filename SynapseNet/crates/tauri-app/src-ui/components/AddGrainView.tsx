import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface AddGrainResponse {
  grain_id: string;
  embedding_time_ms: number;
  storage_time_ms: number;
}

function AddGrainView() {
  const [text, setText] = useState('');
  const [tags, setTags] = useState('');
  const [title, setTitle] = useState('');
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState<{ type: 'success' | 'error'; text: string } | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!text.trim()) {
      setMessage({ type: 'error', text: 'Text cannot be empty' });
      return;
    }

    setLoading(true);
    setMessage(null);

    try {
      const response = await invoke<AddGrainResponse>('add_grain', {
        request: {
          text: text.trim(),
          tags: tags.split(',').map(t => t.trim()).filter(t => t.length > 0),
          title: title.trim() || null,
        },
      });

      setMessage({
        type: 'success',
        text: `✓ Grain added successfully! ID: ${response.grain_id.substring(0, 8)}... (${response.embedding_time_ms}ms)`,
      });

      // Clear form
      setText('');
      setTags('');
      setTitle('');
    } catch (error) {
      setMessage({
        type: 'error',
        text: `Failed to add grain: ${error}`,
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="view-container">
      <h2>Add Knowledge Grain</h2>
      <p className="subtitle">Store a new piece of knowledge in your semantic memory</p>

      <form onSubmit={handleSubmit} className="add-grain-form">
        <div className="form-group">
          <label htmlFor="title">Title (optional)</label>
          <input
            id="title"
            type="text"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            placeholder="A short title for this knowledge..."
            disabled={loading}
          />
        </div>

        <div className="form-group">
          <label htmlFor="text">
            Text <span className="char-count">({text.length} characters)</span>
          </label>
          <textarea
            id="text"
            value={text}
            onChange={(e) => setText(e.target.value)}
            placeholder="Enter your knowledge here..."
            rows={10}
            disabled={loading}
            required
          />
        </div>

        <div className="form-group">
          <label htmlFor="tags">Tags (comma-separated)</label>
          <input
            id="tags"
            type="text"
            value={tags}
            onChange={(e) => setTags(e.target.value)}
            placeholder="ai, machine-learning, rust"
            disabled={loading}
          />
        </div>

        {message && (
          <div className={`message ${message.type}`}>
            {message.text}
          </div>
        )}

        <button type="submit" className="btn-primary" disabled={loading}>
          {loading ? '⏳ Adding...' : '➕ Add Grain'}
        </button>
      </form>
    </div>
  );
}

export default AddGrainView;
