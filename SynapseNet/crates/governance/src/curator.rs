use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Curator review queue
#[derive(Debug, Default)]
pub struct CuratorQueue {
    queue: VecDeque<CuratorTask>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuratorTask {
    pub id: String,
    pub query: String,
    pub grain_ids: Vec<[u8; 32]>,
    pub submitted_at: i64,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InReview,
    Approved,
    Rejected,
}

impl CuratorQueue {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add task to queue
    pub fn submit(&mut self, task: CuratorTask) {
        self.queue.push_back(task);
    }

    /// Get next pending task
    pub fn next(&mut self) -> Option<CuratorTask> {
        self.queue.pop_front()
    }

    /// Queue length
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
