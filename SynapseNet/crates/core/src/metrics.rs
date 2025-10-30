use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Node metrics and statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetrics {
    /// Total grains stored locally
    pub grains_total: usize,
    
    /// Grains created locally
    pub grains_local: usize,
    
    /// Grains received from peers
    pub grains_remote: usize,
    
    /// Number of connected peers
    pub peers_connected: usize,
    
    /// Average embedding generation time (ms)
    pub avg_embedding_time_ms: f64,
    
    /// Average query time (ms)
    pub avg_query_time_ms: f64,
    
    /// Total queries executed
    pub queries_total: usize,
    
    /// Node uptime (seconds)
    pub uptime_seconds: u64,
    
    /// Database size (bytes)
    pub db_size_bytes: u64,
}

impl Default for NodeMetrics {
    fn default() -> Self {
        Self {
            grains_total: 0,
            grains_local: 0,
            grains_remote: 0,
            peers_connected: 0,
            avg_embedding_time_ms: 0.0,
            avg_query_time_ms: 0.0,
            queries_total: 0,
            uptime_seconds: 0,
            db_size_bytes: 0,
        }
    }
}

impl NodeMetrics {
    /// Create new metrics
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Update embedding time
    pub fn record_embedding_time(&mut self, duration_ms: f64) {
        // Running average
        let total = self.avg_embedding_time_ms * self.grains_total as f64;
        self.avg_embedding_time_ms = (total + duration_ms) / (self.grains_total + 1) as f64;
    }
    
    /// Update query time
    pub fn record_query_time(&mut self, duration_ms: f64) {
        // Running average
        let total = self.avg_query_time_ms * self.queries_total as f64;
        self.queries_total += 1;
        self.avg_query_time_ms = (total + duration_ms) / self.queries_total as f64;
    }
    
    /// Format metrics for display
    pub fn format(&self) -> String {
        format!(
            r#"
📊 Node Statistics
==================

Grains:
  Total:          {}
  Local:          {}
  Remote:         {}

Network:
  Peers:          {}

Performance:
  Avg Embedding:  {:.2} ms
  Avg Query:      {:.2} ms
  Total Queries:  {}

System:
  Uptime:         {} seconds
  DB Size:        {:.2} MB
"#,
            self.grains_total,
            self.grains_local,
            self.grains_remote,
            self.peers_connected,
            self.avg_embedding_time_ms,
            self.avg_query_time_ms,
            self.queries_total,
            self.uptime_seconds,
            self.db_size_bytes as f64 / 1_048_576.0
        )
    }
}

/// Performance timer for metrics
pub struct MetricsTimer {
    start: Instant,
}

impl MetricsTimer {
    /// Start new timer
    pub fn start() -> Self {
        Self {
            start: Instant::now(),
        }
    }
    
    /// Get elapsed time in milliseconds
    pub fn elapsed_ms(&self) -> f64 {
        self.start.elapsed().as_secs_f64() * 1000.0
    }
}
