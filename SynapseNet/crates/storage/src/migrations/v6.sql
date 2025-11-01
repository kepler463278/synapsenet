-- Migration v6: Add reasoning tables

-- Goals table
CREATE TABLE IF NOT EXISTS goals (
    id TEXT PRIMARY KEY,
    text TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('pending', 'planning', 'reasoning', 'reflecting', 'completed', 'failed')),
    priority INTEGER NOT NULL DEFAULT 1 CHECK (priority >= 0 AND priority <= 3),
    created_by TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    parent_id TEXT,
    metadata TEXT NOT NULL DEFAULT '{}',
    FOREIGN KEY (parent_id) REFERENCES goals(id) ON DELETE CASCADE
);

-- Plans table
CREATE TABLE IF NOT EXISTS plans (
    id TEXT PRIMARY KEY,
    goal_id TEXT NOT NULL,
    dag_json TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (goal_id) REFERENCES goals(id) ON DELETE CASCADE
);

-- Episodes table
CREATE TABLE IF NOT EXISTS episodes (
    id TEXT PRIMARY KEY,
    goal_id TEXT NOT NULL,
    step INTEGER NOT NULL,
    query TEXT NOT NULL,
    synthesis TEXT NOT NULL,
    confidence REAL NOT NULL CHECK (confidence >= 0.0 AND confidence <= 1.0),
    vec BLOB,
    meta TEXT NOT NULL DEFAULT '{}',
    timestamp INTEGER NOT NULL,
    FOREIGN KEY (goal_id) REFERENCES goals(id) ON DELETE CASCADE
);

-- Reasoning statistics
CREATE TABLE IF NOT EXISTS reason_stats (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- Performance indexes
CREATE INDEX IF NOT EXISTS idx_goals_status ON goals(status);
CREATE INDEX IF NOT EXISTS idx_goals_priority ON goals(priority DESC);
CREATE INDEX IF NOT EXISTS idx_goals_created_at ON goals(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_goals_parent ON goals(parent_id);

CREATE INDEX IF NOT EXISTS idx_plans_goal_id ON plans(goal_id);
CREATE INDEX IF NOT EXISTS idx_plans_created_at ON plans(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_episodes_goal_id ON episodes(goal_id);
CREATE INDEX IF NOT EXISTS idx_episodes_step ON episodes(step);
CREATE INDEX IF NOT EXISTS idx_episodes_timestamp ON episodes(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_episodes_confidence ON episodes(confidence DESC);

-- Initialize stats
INSERT OR IGNORE INTO reason_stats (key, value) VALUES 
    ('total_goals', '0'),
    ('total_episodes', '0'),
    ('avg_confidence', '0.0'),
    ('schema_version', '6');
