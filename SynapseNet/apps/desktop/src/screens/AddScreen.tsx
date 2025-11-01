import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Plus, CheckCircle } from 'lucide-react';

interface Props {
  onGrainAdded: () => void;
}

function AddScreen({ onGrainAdded }: Props) {
  const [content, setContent] = useState('');
  const [loading, setLoading] = useState(false);
  const [success, setSuccess] = useState<{
    grainId: string;
    reward: number;
  } | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!content.trim()) {
      alert('Please enter some content');
      return;
    }

    setLoading(true);
    setSuccess(null);

    try {
      const grainId = await invoke<string>('add_grain', {
        content: content.trim(),
      });

      // Simulate reward calculation (in real implementation, this comes from backend)
      const reward = 0.7; // This would be calculated based on novelty

      setSuccess({ grainId, reward });
      setContent('');
      onGrainAdded();

      // Clear success message after 5 seconds
      setTimeout(() => setSuccess(null), 5000);
    } catch (error) {
      console.error('Failed to add grain:', error);
      alert('Failed to add grain: ' + error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="screen">
      <div className="card">
        <h2 className="card-title">
          <Plus size={20} style={{ display: 'inline', marginRight: '8px' }} />
          Add Knowledge
        </h2>

        <p style={{ color: '#6b7280', marginBottom: '16px', fontSize: '14px' }}>
          Share your insights, ideas, or knowledge with the network. You'll earn NGT
          based on the novelty and value of your contribution.
        </p>

        <form onSubmit={handleSubmit}>
          <textarea
            className="textarea"
            placeholder="Enter your knowledge, insights, or ideas here..."
            value={content}
            onChange={(e) => setContent(e.target.value)}
            rows={10}
            disabled={loading}
          />

          <div style={{ marginTop: '16px', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <div style={{ fontSize: '13px', color: '#6b7280' }}>
              {content.length} characters
            </div>
            <button
              type="submit"
              className="btn btn-primary"
              disabled={loading || !content.trim()}
            >
              {loading ? 'Adding...' : 'Add to Network'}
            </button>
          </div>
        </form>
      </div>

      {success && (
        <div className="card" style={{ background: '#dcfce7', borderColor: '#22c55e' }}>
          <div style={{ display: 'flex', alignItems: 'flex-start', gap: '12px' }}>
            <CheckCircle size={24} color="#22c55e" />
            <div style={{ flex: 1 }}>
              <h3 style={{ fontSize: '16px', fontWeight: '600', color: '#166534', marginBottom: '8px' }}>
                Grain Added Successfully!
              </h3>
              <p style={{ fontSize: '14px', color: '#166534', marginBottom: '8px' }}>
                Your knowledge has been added to the network and is being broadcast to peers.
              </p>
              <div style={{ fontSize: '14px', color: '#166534' }}>
                <strong>Grain ID:</strong> {success.grainId.substring(0, 16)}...
              </div>
              <div style={{ fontSize: '16px', fontWeight: '600', color: '#22c55e', marginTop: '12px' }}>
                🎉 You earned +{success.reward.toFixed(2)} NGT!
              </div>
            </div>
          </div>
        </div>
      )}

      <div className="card">
        <h3 className="card-title">How Rewards Work</h3>
        <div style={{ fontSize: '14px', color: '#374151', lineHeight: '1.6' }}>
          <p style={{ marginBottom: '12px' }}>
            Your contributions are rewarded based on their novelty and value to the network:
          </p>
          <ul style={{ paddingLeft: '20px', marginBottom: '12px' }}>
            <li style={{ marginBottom: '8px' }}>
              <strong>Novel Knowledge:</strong> Unique insights earn more NGT
            </li>
            <li style={{ marginBottom: '8px' }}>
              <strong>Validation:</strong> When peers validate your grain, you earn additional rewards
            </li>
            <li style={{ marginBottom: '8px' }}>
              <strong>Network Growth:</strong> As the network grows, reward multipliers increase
            </li>
          </ul>
          <p style={{ color: '#6b7280', fontSize: '13px' }}>
            All rewards are calculated automatically using Proof-of-Emergence consensus.
          </p>
        </div>
      </div>
    </div>
  );
}

export default AddScreen;
