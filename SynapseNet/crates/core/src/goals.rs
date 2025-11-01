//! Goal management for autonomous reasoning
//! 
//! Goals represent high-level objectives that the reasoning system
//! attempts to achieve through planning and execution.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

/// Status of a goal in the reasoning pipeline
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoalStatus {
    /// Goal is queued for processing
    Pending,
    /// Goal is currently being planned
    Planning,
    /// Plan is ready, executing reasoning steps
    Reasoning,
    /// Reasoning complete, reflecting on results
    Reflecting,
    /// Goal successfully completed
    Completed,
    /// Goal failed or was cancelled
    Failed,
}

/// Priority level for goal processing
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// A goal represents a high-level objective for the reasoning system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    /// Unique identifier
    pub id: Uuid,
    /// Natural language description of the goal
    pub text: String,
    /// Current status
    pub status: GoalStatus,
    /// Priority level
    pub priority: Priority,
    /// Who/what created this goal
    pub created_by: String,
    /// When the goal was created
    pub created_at: i64,
    /// When the goal was last updated
    pub updated_at: i64,
    /// Optional parent goal (for sub-goals)
    pub parent_id: Option<Uuid>,
    /// Metadata (tags, context, etc.)
    pub metadata: serde_json::Value,
}

impl Goal {
    /// Create a new goal
    pub fn new(text: impl Into<String>, created_by: impl Into<String>) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: Uuid::new_v4(),
            text: text.into(),
            status: GoalStatus::Pending,
            priority: Priority::Normal,
            created_by: created_by.into(),
            created_at: now,
            updated_at: now,
            parent_id: None,
            metadata: serde_json::json!({}),
        }
    }

    /// Create a sub-goal
    pub fn sub_goal(
        &self,
        text: impl Into<String>,
        created_by: impl Into<String>,
    ) -> Self {
        let mut goal = Self::new(text, created_by);
        goal.parent_id = Some(self.id);
        goal.priority = self.priority;
        goal
    }

    /// Update goal status
    pub fn set_status(&mut self, status: GoalStatus) {
        self.status = status;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    /// Check if goal is terminal (completed or failed)
    pub fn is_terminal(&self) -> bool {
        matches!(self.status, GoalStatus::Completed | GoalStatus::Failed)
    }

    /// Check if goal is active (being processed)
    pub fn is_active(&self) -> bool {
        matches!(
            self.status,
            GoalStatus::Planning | GoalStatus::Reasoning | GoalStatus::Reflecting
        )
    }
}

/// Goal queue with priority ordering
#[derive(Debug, Default)]
pub struct GoalQueue {
    queue: VecDeque<Goal>,
}

impl GoalQueue {
    /// Create a new empty queue
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    /// Add a goal to the queue
    pub fn push(&mut self, goal: Goal) {
        // Insert based on priority (higher priority first)
        let pos = self
            .queue
            .iter()
            .position(|g| g.priority < goal.priority)
            .unwrap_or(self.queue.len());
        
        self.queue.insert(pos, goal);
    }

    /// Get the next goal to process
    pub fn pop(&mut self) -> Option<Goal> {
        self.queue.pop_front()
    }

    /// Peek at the next goal without removing it
    pub fn peek(&self) -> Option<&Goal> {
        self.queue.front()
    }

    /// Get goal by ID
    pub fn get(&self, id: &Uuid) -> Option<&Goal> {
        self.queue.iter().find(|g| g.id == *id)
    }

    /// Get mutable goal by ID
    pub fn get_mut(&mut self, id: &Uuid) -> Option<&mut Goal> {
        self.queue.iter_mut().find(|g| g.id == *id)
    }

    /// Remove goal by ID
    pub fn remove(&mut self, id: &Uuid) -> Option<Goal> {
        if let Some(pos) = self.queue.iter().position(|g| g.id == *id) {
            self.queue.remove(pos)
        } else {
            None
        }
    }

    /// Get queue length
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Get all goals with a specific status
    pub fn by_status(&self, status: GoalStatus) -> Vec<&Goal> {
        self.queue.iter().filter(|g| g.status == status).collect()
    }

    /// Get all active goals
    pub fn active(&self) -> Vec<&Goal> {
        self.queue.iter().filter(|g| g.is_active()).collect()
    }

    /// Clear all terminal goals
    pub fn clear_terminal(&mut self) {
        self.queue.retain(|g| !g.is_terminal());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goal_creation() {
        let goal = Goal::new("Test goal", "user");
        assert_eq!(goal.text, "Test goal");
        assert_eq!(goal.status, GoalStatus::Pending);
        assert_eq!(goal.priority, Priority::Normal);
        assert_eq!(goal.created_by, "user");
    }

    #[test]
    fn test_sub_goal() {
        let parent = Goal::new("Parent goal", "user");
        let child = parent.sub_goal("Child goal", "user");
        
        assert_eq!(child.parent_id, Some(parent.id));
        assert_eq!(child.priority, parent.priority);
    }

    #[test]
    fn test_goal_status_update() {
        let mut goal = Goal::new("Test", "user");
        let initial_time = goal.updated_at;
        
        std::thread::sleep(std::time::Duration::from_millis(10));
        goal.set_status(GoalStatus::Planning);
        
        assert_eq!(goal.status, GoalStatus::Planning);
        assert!(goal.updated_at > initial_time);
    }

    #[test]
    fn test_goal_queue_priority() {
        let mut queue = GoalQueue::new();
        
        let mut low = Goal::new("Low priority", "user");
        low.priority = Priority::Low;
        
        let mut high = Goal::new("High priority", "user");
        high.priority = Priority::High;
        
        let normal = Goal::new("Normal priority", "user");
        
        queue.push(low);
        queue.push(normal);
        queue.push(high);
        
        // Should pop high priority first
        let first = queue.pop().unwrap();
        assert_eq!(first.priority, Priority::High);
        
        let second = queue.pop().unwrap();
        assert_eq!(second.priority, Priority::Normal);
        
        let third = queue.pop().unwrap();
        assert_eq!(third.priority, Priority::Low);
    }

    #[test]
    fn test_goal_queue_operations() {
        let mut queue = GoalQueue::new();
        let goal = Goal::new("Test", "user");
        let id = goal.id;
        
        queue.push(goal);
        assert_eq!(queue.len(), 1);
        
        let found = queue.get(&id);
        assert!(found.is_some());
        
        let removed = queue.remove(&id);
        assert!(removed.is_some());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_terminal_goals() {
        let mut completed = Goal::new("Completed", "user");
        completed.status = GoalStatus::Completed;
        assert!(completed.is_terminal());
        
        let mut failed = Goal::new("Failed", "user");
        failed.status = GoalStatus::Failed;
        assert!(failed.is_terminal());
        
        let pending = Goal::new("Pending", "user");
        assert!(!pending.is_terminal());
    }

    #[test]
    fn test_active_goals() {
        let mut planning = Goal::new("Planning", "user");
        planning.status = GoalStatus::Planning;
        assert!(planning.is_active());
        
        let mut reasoning = Goal::new("Reasoning", "user");
        reasoning.status = GoalStatus::Reasoning;
        assert!(reasoning.is_active());
        
        let pending = Goal::new("Pending", "user");
        assert!(!pending.is_active());
    }
}
