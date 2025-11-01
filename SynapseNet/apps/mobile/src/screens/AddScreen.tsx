import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import VoiceButton from '../components/VoiceButton';
import FileImport from '../components/FileImport';

function AddScreen() {
  const [text, setText] = useState('');
  const [tags, setTags] = useState('');
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState('');

  const handleSubmit = async () => {
    if (!text.trim()) {
      setMessage('Please enter some text');
      return;
    }

    setLoading(true);
    setMessage('');

    try {
      const tagList = tags.split(',').map(t => t.trim()).filter(t => t);
      await invoke('syn_add', {
        grain: { text, tags: tagList }
      });
      
      setMessage('✅ Knowledge added successfully!');
      setText('');
      setTags('');
    } catch (error) {
      setMessage(`❌ Error: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="screen add-screen">
      <header>
        <h2>➕ Add Knowledge</h2>
      </header>

      <div className="form">
        <div className="form-group">
          <label>Your Knowledge</label>
          <textarea
            value={text}
            onChange={(e) => setText(e.target.value)}
            placeholder="Enter your knowledge here..."
            rows={8}
            disabled={loading}
          />
        </div>

        <div className="form-group">
          <label>Tags (comma-separated)</label>
          <input
            type="text"
            value={tags}
            onChange={(e) => setTags(e.target.value)}
            placeholder="ai, machine-learning, rust"
            disabled={loading}
          />
        </div>

        <button
          className="btn-primary"
          onClick={handleSubmit}
          disabled={loading || !text.trim()}
        >
          {loading ? 'Adding...' : 'Add Grain'}
        </button>

        {message && (
          <div className={`message ${message.startsWith('✅') ? 'success' : 'error'}`}>
            {message}
          </div>
        )}
      </div>

      <FileImport 
        onImportComplete={(result) => {
          if (result.success) {
            setMessage(`✅ Imported ${result.grains_created} grains from ${result.files_processed} files!`);
          }
        }}
      />

      <div className="tips">
        <h4>💡 Tips</h4>
        <ul>
          <li>Be specific and clear</li>
          <li>Add relevant tags for better search</li>
          <li>Your data is encrypted locally</li>
        </ul>
      </div>
    </div>
  );
}

export default AddScreen;
