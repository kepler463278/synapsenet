//! Database schema v6 for reasoning system

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Goal record in database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalRecord {
    pub id: String,
    pub text: String,
    pub status: String,
    pub priority: i32,
    pub created_by: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub parent_id: Option<String>,
    pub metadata: String, // JSON
}

/// Plan record in database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanRecord {
    pub id: String,
    pub goal_id: String,
    pub dag_json: String,
    pub created_at: i64,
}

/// Episode record in database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeRecord {
    pub id: String,
    pub goal_id: String,
    pub step: i32,
    pub query: String,
    pub synthesis: String,
    pub confidence: f64,
    pub vec: Vec<u8>, // BLOB
    pub meta: String, // JSON
    pub timestamp: i64,
}

/// Reasoning statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonStats {
    pub key: String,
    pub value: String,
}

/// SQL schema creation
pub const CREATE_GOALS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS goals (
    id TEXT PRIMARY KEY,
    text TEXT NOT NULL,
    status TEXT NOT NULL,
    priority INTEGER NOT NULL DEFAULT 1,
    created_by TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    parent_id TEXT,
    metadata TEXT NOT NULL DEFAULT '{}',
    FOREIGN KEY (parent_id) REFERENCES goals(id)
);
"#;

pub const CREATE_PLANS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS plans (
    id TEXT PRIMARY KEY,
    goal_id TEXT NOT NULL,
    dag_json TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (goal_id) REFERENCES goals(id)
);
"#;

pub const CREATE_EPISODES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS episodes (
    id TEXT PRIMARY KEY,
    goal_id TEXT NOT NULL,
    step INTEGER NOT NULL,
    query TEXT NOT NULL,
    synthesis TEXT NOT NULL,
    confidence REAL NOT NULL,
    vec BLOB,
    meta TEXT NOT NULL DEFAULT '{}',
    timestamp INTEGER NOT NULL,
    FOREIGN KEY (goal_id) REFERENCES goals(id)
);
"#;

pub const CREATE_REASON_STATS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS reason_stats (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
"#;

/// Create indexes for performance
pub const CREATE_INDEXES: &[&str] = &[
    "CREATE INDEX IF NOT EXISTS idx_goals_status ON goals(status);",
    "CREATE INDEX IF NOT EXISTS idx_goals_priority ON goals(priority);",
    "CREATE INDEX IF NOT EXISTS idx_goals_created_at ON goals(created_at);",
    "CREATE INDEX IF NOT EXISTS idx_plans_goal_id ON plans(goal_id);",
    "CREATE INDEX IF NOT EXISTS idx_episodes_goal_id ON episodes(goal_id);",
    "CREATE INDEX IF NOT EXISTS idx_episodes_step ON episodes(step);",
    "CREATE INDEX IF NOT EXISTS idx_episodes_timestamp ON episodes(timestamp);",
];

/// Database operations
pub struct ReasoningDb {
    // TODO: Add actual database connection
}

impl ReasoningDb {
    /// Initialize database with schema
    pub async fn init() -> Result<Self, String> {
        // TODO: Implement SQLite connection
        Ok(Self {})
    }

    /// Create all tables
    pub async fn create_tables(&self) -> Result<(), String> {
        // TODO: Execute CREATE TABLE statements
        Ok(())
    }

    /// Insert goal
    pub async fn insert_goal(&self, _goal: &GoalRecord) -> Result<(), String> {
        // TODO: INSERT INTO goals
        Ok(())
    }

    /// Get goal by ID
    pub async fn get_goal(&self, _id: &str) -> Result<Option<GoalRecord>, String> {
        // TODO: SELECT FROM goals WHERE id = ?
        Ok(None)
    }

    /// Insert plan
    pub async fn insert_plan(&self, _plan: &PlanRecord) -> Result<(), String> {
        // TODO: INSERT INTO plans
        Ok(())
    }

    /// Insert episode
    pub async fn insert_episode(&self, _episode: &EpisodeRecord) -> Result<(), String> {
        // TODO: INSERT INTO episodes
        Ok(())
    }

    /// Get episodes for goal
    pub async fn get_episodes(&self, _goal_id: &str) -> Result<Vec<EpisodeRecord>, String> {
        // TODO: SELECT FROM episodes WHERE goal_id = ?
        Ok(Vec::new())
    }

    /// Update stats
    pub async fn update_stat(&self, _key: &str, _value: &str) -> Result<(), String> {
        // TODO: INSERT OR REPLACE INTO reason_stats
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db_init() {
        let _db = ReasoningDb::init().await.unwrap();
    }

    #[test]
    fn test_goal_record() {
        let goal = GoalRecord {
            id: Uuid::new_v4().to_string(),
            text: "Test goal".to_string(),
            status: "pending".to_string(),
            priority: 1,
            created_by: "user".to_string(),
            created_at: 1234567890,
            updated_at: 1234567890,
            parent_id: None,
            metadata: "{}".to_string(),
        };
        
        assert_eq!(goal.text, "Test goal");
    }
}
