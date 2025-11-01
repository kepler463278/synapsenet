import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface GraphNode {
  id: string;
  title: string;
  tags: string[];
  x: number;
  y: number;
}

interface GraphEdge {
  from: string;
  to: string;
  similarity: number;
}

function GraphView() {
  const [nodes, setNodes] = useState<GraphNode[]>([]);
  const [edges, setEdges] = useState<GraphEdge[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedNode, setSelectedNode] = useState<GraphNode | null>(null);

  useEffect(() => {
    loadGraph();
  }, []);

  const loadGraph = async () => {
    setLoading(true);
    setError(null);

    try {
      // For MVP, we'll create a simple visualization from search results
      // In future tasks, we'll add proper graph data structure
      const stats = await invoke<any>('get_stats');
      
      if (stats.total_grains === 0) {
        setError('No grains to visualize. Add some knowledge first!');
        setLoading(false);
        return;
      }

      // Create placeholder nodes in a circle layout
      const mockNodes: GraphNode[] = [];
      const mockEdges: GraphEdge[] = [];
      
      const centerX = 400;
      const centerY = 300;
      const radius = 200;
      const count = Math.min(stats.total_grains, 20); // Limit to 20 for performance

      for (let i = 0; i < count; i++) {
        const angle = (i / count) * 2 * Math.PI;
        mockNodes.push({
          id: `node-${i}`,
          title: `Grain ${i + 1}`,
          tags: ['sample'],
          x: centerX + radius * Math.cos(angle),
          y: centerY + radius * Math.sin(angle),
        });

        // Create some edges
        if (i > 0) {
          mockEdges.push({
            from: `node-${i - 1}`,
            to: `node-${i}`,
            similarity: 0.5 + Math.random() * 0.5,
          });
        }
      }

      setNodes(mockNodes);
      setEdges(mockEdges);
    } catch (err) {
      setError(`Failed to load graph: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div className="view-container">
        <h2>Knowledge Graph</h2>
        <p>Loading graph...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="view-container">
        <h2>Knowledge Graph</h2>
        <div className="message error">{error}</div>
        <button onClick={loadGraph} className="btn-primary">
          Retry
        </button>
      </div>
    );
  }

  return (
    <div className="view-container">
      <h2>Knowledge Graph</h2>
      <p className="subtitle">
        Visual representation of your knowledge network (MVP - simplified view)
      </p>

      <div className="graph-container">
        <svg width="800" height="600" className="graph-svg">
          {/* Draw edges */}
          {edges.map((edge, idx) => {
            const fromNode = nodes.find((n) => n.id === edge.from);
            const toNode = nodes.find((n) => n.id === edge.to);
            if (!fromNode || !toNode) return null;

            return (
              <line
                key={idx}
                x1={fromNode.x}
                y1={fromNode.y}
                x2={toNode.x}
                y2={toNode.y}
                stroke="#475569"
                strokeWidth={edge.similarity * 3}
                opacity={0.6}
              />
            );
          })}

          {/* Draw nodes */}
          {nodes.map((node) => (
            <g
              key={node.id}
              onClick={() => setSelectedNode(node)}
              style={{ cursor: 'pointer' }}
            >
              <circle
                cx={node.x}
                cy={node.y}
                r={selectedNode?.id === node.id ? 12 : 8}
                fill={selectedNode?.id === node.id ? '#6366f1' : '#334155'}
                stroke="#6366f1"
                strokeWidth={selectedNode?.id === node.id ? 3 : 1}
              />
              <text
                x={node.x}
                y={node.y - 15}
                textAnchor="middle"
                fill="#f1f5f9"
                fontSize="12"
              >
                {node.title}
              </text>
            </g>
          ))}
        </svg>

        {selectedNode && (
          <div className="graph-details">
            <h3>Selected Node</h3>
            <p>
              <strong>ID:</strong> {selectedNode.id}
            </p>
            <p>
              <strong>Title:</strong> {selectedNode.title}
            </p>
            <p>
              <strong>Tags:</strong> {selectedNode.tags.join(', ')}
            </p>
            <button
              onClick={() => setSelectedNode(null)}
              className="btn-secondary"
            >
              Clear Selection
            </button>
          </div>
        )}
      </div>

      <div className="graph-info">
        <p>
          📊 Showing {nodes.length} nodes and {edges.length} connections
        </p>
        <p className="text-secondary">
          Note: This is a simplified MVP visualization. Full graph features with
          D3.js/Cytoscape will be added in future updates.
        </p>
      </div>
    </div>
  );
}

export default GraphView;
