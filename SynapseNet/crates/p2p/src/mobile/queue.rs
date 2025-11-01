//! Operation queue for offline/background P2P operations

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

/// P2P operation for queuing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2POperation {
    /// Unique operation ID
    pub id: Uuid,
    
    /// Operation type
    pub op_type: OperationType,
    
    /// Creation timestamp
    pub created_at: SystemTime,
    
    /// Retry count
    pub retry_count: u32,
    
    /// Maximum retries
    pub max_retries: u32,
    
    /// Requires WiFi connection
    pub requires_wifi: bool,
    
    /// Priority (higher = more important)
    pub priority: u8,
}

/// Type of P2P operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    /// Publish a grain to network
    PublishGrain {
        grain_id: [u8; 32],
        data: Vec<u8>,
    },
    
    /// Query network for similar grains
    QueryNetwork {
        query: Vec<f32>,
        k: usize,
    },
    
    /// Sync PoE rewards
    SyncRewards,
    
    /// Update peer list
    UpdatePeerList,
    
    /// Sync grain with specific peer
    SyncGrain {
        grain_id: [u8; 32],
        peer_id: String,
    },
}

impl P2POperation {
    /// Create a new operation
    pub fn new(op_type: OperationType) -> Self {
        Self {
            id: Uuid::new_v4(),
            op_type,
            created_at: SystemTime::now(),
            retry_count: 0,
            max_retries: 3,
            requires_wifi: false,
            priority: 5,
        }
    }
    
    /// Create high-priority operation
    pub fn high_priority(op_type: OperationType) -> Self {
        let mut op = Self::new(op_type);
        op.priority = 10;
        op
    }
    
    /// Create WiFi-only operation
    pub fn wifi_only(op_type: OperationType) -> Self {
        let mut op = Self::new(op_type);
        op.requires_wifi = true;
        op
    }
    
    /// Check if operation should be retried
    pub fn should_retry(&self) -> bool {
        self.retry_count < self.max_retries
    }
    
    /// Increment retry count
    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
    }
    
    /// Check if operation is expired
    pub fn is_expired(&self, max_age: Duration) -> bool {
        self.created_at.elapsed().unwrap_or(Duration::ZERO) > max_age
    }
}

/// Operation queue manager
pub struct OperationQueue {
    queue: VecDeque<P2POperation>,
    max_size: usize,
    max_age: Duration,
}

impl OperationQueue {
    /// Create a new operation queue
    pub fn new(max_size: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            max_size,
            max_age: Duration::from_secs(86400), // 24 hours
        }
    }
    
    /// Add operation to queue
    pub fn enqueue(&mut self, operation: P2POperation) -> Result<()> {
        if self.queue.len() >= self.max_size {
            // Remove oldest low-priority operation
            if let Some(pos) = self.queue.iter().position(|op| op.priority < 5) {
                self.queue.remove(pos);
            } else {
                // Queue full with high-priority items
                tracing::warn!("Operation queue full, dropping operation");
                return Ok(());
            }
        }
        
        // Insert based on priority
        let insert_pos = self.queue
            .iter()
            .position(|op| op.priority < operation.priority)
            .unwrap_or(self.queue.len());
        
        self.queue.insert(insert_pos, operation);
        
        Ok(())
    }
    
    /// Get next operation to process
    pub fn dequeue(&mut self) -> Option<P2POperation> {
        self.queue.pop_front()
    }
    
    /// Peek at next operation without removing
    pub fn peek(&self) -> Option<&P2POperation> {
        self.queue.front()
    }
    
    /// Get operations that can be processed now
    pub fn get_processable(&mut self, wifi_available: bool) -> Vec<P2POperation> {
        let mut processable = Vec::new();
        let mut remaining = VecDeque::new();
        
        while let Some(op) = self.queue.pop_front() {
            if op.requires_wifi && !wifi_available {
                remaining.push_back(op);
            } else if op.is_expired(self.max_age) {
                tracing::debug!("Dropping expired operation: {:?}", op.id);
            } else {
                processable.push(op);
            }
        }
        
        self.queue = remaining;
        processable
    }
    
    /// Re-queue failed operation
    pub fn requeue(&mut self, mut operation: P2POperation) -> Result<()> {
        if operation.should_retry() {
            operation.increment_retry();
            self.enqueue(operation)?;
        } else {
            tracing::warn!("Operation {} exceeded max retries", operation.id);
        }
        Ok(())
    }
    
    /// Get queue size
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    
    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    
    /// Clear all operations
    pub fn clear(&mut self) {
        self.queue.clear();
    }
    
    /// Remove expired operations
    pub fn cleanup_expired(&mut self) {
        self.queue.retain(|op| !op.is_expired(self.max_age));
    }
    
    /// Get operations by type
    pub fn get_by_type(&self, op_type_filter: fn(&OperationType) -> bool) -> Vec<&P2POperation> {
        self.queue
            .iter()
            .filter(|op| op_type_filter(&op.op_type))
            .collect()
    }
    
    /// Save queue to disk (placeholder)
    pub fn save(&self) -> Result<()> {
        // TODO: Implement persistent queue storage
        tracing::debug!("Saving operation queue ({} items)", self.queue.len());
        Ok(())
    }
    
    /// Load queue from disk (placeholder)
    pub fn load(&mut self) -> Result<()> {
        // TODO: Implement queue loading
        tracing::debug!("Loading operation queue");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_operation_creation() {
        let op = P2POperation::new(OperationType::SyncRewards);
        assert_eq!(op.retry_count, 0);
        assert_eq!(op.priority, 5);
        assert!(!op.requires_wifi);
    }
    
    #[test]
    fn test_high_priority_operation() {
        let op = P2POperation::high_priority(OperationType::SyncRewards);
        assert_eq!(op.priority, 10);
    }
    
    #[test]
    fn test_wifi_only_operation() {
        let op = P2POperation::wifi_only(OperationType::UpdatePeerList);
        assert!(op.requires_wifi);
    }
    
    #[test]
    fn test_queue_enqueue_dequeue() {
        let mut queue = OperationQueue::new(10);
        
        let op = P2POperation::new(OperationType::SyncRewards);
        queue.enqueue(op.clone()).unwrap();
        
        assert_eq!(queue.len(), 1);
        
        let dequeued = queue.dequeue();
        assert!(dequeued.is_some());
        assert_eq!(queue.len(), 0);
    }
    
    #[test]
    fn test_queue_priority() {
        let mut queue = OperationQueue::new(10);
        
        let low = P2POperation::new(OperationType::UpdatePeerList);
        let high = P2POperation::high_priority(OperationType::SyncRewards);
        
        queue.enqueue(low).unwrap();
        queue.enqueue(high.clone()).unwrap();
        
        // High priority should be dequeued first
        let first = queue.dequeue().unwrap();
        assert_eq!(first.priority, 10);
    }
    
    #[test]
    fn test_queue_max_size() {
        let mut queue = OperationQueue::new(2);
        
        queue.enqueue(P2POperation::new(OperationType::SyncRewards)).unwrap();
        queue.enqueue(P2POperation::new(OperationType::UpdatePeerList)).unwrap();
        queue.enqueue(P2POperation::new(OperationType::SyncRewards)).unwrap();
        
        // Should not exceed max size
        assert!(queue.len() <= 2);
    }
    
    #[test]
    fn test_operation_retry() {
        let mut op = P2POperation::new(OperationType::SyncRewards);
        op.max_retries = 3;
        
        assert!(op.should_retry());
        
        op.increment_retry();
        assert_eq!(op.retry_count, 1);
        assert!(op.should_retry());
        
        op.retry_count = 3;
        assert!(!op.should_retry());
    }
    
    #[test]
    fn test_get_processable_wifi() {
        let mut queue = OperationQueue::new(10);
        
        let wifi_op = P2POperation::wifi_only(OperationType::UpdatePeerList);
        let normal_op = P2POperation::new(OperationType::SyncRewards);
        
        queue.enqueue(wifi_op).unwrap();
        queue.enqueue(normal_op).unwrap();
        
        // Without WiFi, only normal op should be processable
        let processable = queue.get_processable(false);
        assert_eq!(processable.len(), 1);
        assert!(!processable[0].requires_wifi);
        
        // WiFi op should still be in queue
        assert_eq!(queue.len(), 1);
    }
}
