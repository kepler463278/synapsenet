//! Episodes v2 - Extended with action support

use crate::episodes::Episode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Action log within an episode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeAction {
    pub id: Uuid,
    pub tool_name: String,
    pub input: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
    pub timestamp: i64,
    pub execution_time_ms: u64,
    pub success: bool,
}

/// Episode with action support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeV2 {
    /// Base episode data
    #[serde(flatten)]
    pub base: Episode,
    
    /// Actions performed in this episode
    pub actions: Vec<EpisodeAction>,
    
    /// Whether actions were used
    pub used_tools: bool,
}

impl EpisodeV2 {
    /// Create new episode v2 from base episode
    pub fn from_episode(episode: Episode) -> Self {
        Self {
            base: episode,
            actions: Vec::new(),
            used_tools: false,
        }
    }

    /// Add action to episode
    pub fn add_action(&mut self, action: EpisodeAction) {
        self.actions.push(action);
        self.used_tools = true;
    }

    /// Get all successful actions
    pub fn successful_actions(&self) -> Vec<&EpisodeAction> {
        self.actions.iter().filter(|a| a.success).collect()
    }

    /// Get all failed actions
    pub fn failed_actions(&self) -> Vec<&EpisodeAction> {
        self.actions.iter().filter(|a| !a.success).collect()
    }

    /// Total execution time for all actions
    pub fn total_action_time_ms(&self) -> u64 {
        self.actions.iter().map(|a| a.execution_time_ms).sum()
    }

    /// Count actions by tool
    pub fn actions_by_tool(&self) -> std::collections::HashMap<String, usize> {
        let mut counts = std::collections::HashMap::new();
        for action in &self.actions {
            *counts.entry(action.tool_name.clone()).or_insert(0) += 1;
        }
        counts
    }

    /// Get action success rate
    pub fn action_success_rate(&self) -> f64 {
        if self.actions.is_empty() {
            return 1.0;
        }
        
        let successful = self.successful_actions().len() as f64;
        let total = self.actions.len() as f64;
        successful / total
    }
}

/// Episode trace with actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeTrace {
    pub goal_id: Uuid,
    pub episodes: Vec<EpisodeV2>,
    pub total_actions: usize,
    pub total_execution_time_ms: u64,
}

impl EpisodeTrace {
    /// Create new trace
    pub fn new(goal_id: Uuid) -> Self {
        Self {
            goal_id,
            episodes: Vec::new(),
            total_actions: 0,
            total_execution_time_ms: 0,
        }
    }

    /// Add episode to trace
    pub fn add_episode(&mut self, episode: EpisodeV2) {
        self.total_actions += episode.actions.len();
        self.total_execution_time_ms += episode.total_action_time_ms();
        self.episodes.push(episode);
    }

    /// Get all actions across episodes
    pub fn all_actions(&self) -> Vec<&EpisodeAction> {
        self.episodes.iter().flat_map(|e| &e.actions).collect()
    }

    /// Get tools used
    pub fn tools_used(&self) -> Vec<String> {
        let mut tools: Vec<String> = self
            .all_actions()
            .iter()
            .map(|a| a.tool_name.clone())
            .collect();
        tools.sort();
        tools.dedup();
        tools
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_episode_v2_creation() {
        let base = Episode::new(Uuid::new_v4(), 1, "Test query");
        let episode = EpisodeV2::from_episode(base);
        
        assert!(!episode.used_tools);
        assert_eq!(episode.actions.len(), 0);
    }

    #[test]
    fn test_add_action() {
        let base = Episode::new(Uuid::new_v4(), 1, "Test query");
        let mut episode = EpisodeV2::from_episode(base);
        
        let action = EpisodeAction {
            id: Uuid::new_v4(),
            tool_name: "web_fetch".to_string(),
            input: serde_json::json!({"url": "https://example.com"}),
            output: Some(serde_json::json!({"status": 200})),
            error: None,
            timestamp: chrono::Utc::now().timestamp(),
            execution_time_ms: 100,
            success: true,
        };
        
        episode.add_action(action);
        
        assert!(episode.used_tools);
        assert_eq!(episode.actions.len(), 1);
    }

    #[test]
    fn test_action_success_rate() {
        let base = Episode::new(Uuid::new_v4(), 1, "Test query");
        let mut episode = EpisodeV2::from_episode(base);
        
        // Add successful action
        episode.add_action(EpisodeAction {
            id: Uuid::new_v4(),
            tool_name: "test".to_string(),
            input: serde_json::json!({}),
            output: Some(serde_json::json!({})),
            error: None,
            timestamp: chrono::Utc::now().timestamp(),
            execution_time_ms: 100,
            success: true,
        });
        
        // Add failed action
        episode.add_action(EpisodeAction {
            id: Uuid::new_v4(),
            tool_name: "test".to_string(),
            input: serde_json::json!({}),
            output: None,
            error: Some("Error".to_string()),
            timestamp: chrono::Utc::now().timestamp(),
            execution_time_ms: 50,
            success: false,
        });
        
        assert_eq!(episode.action_success_rate(), 0.5);
    }

    #[test]
    fn test_episode_trace() {
        let mut trace = EpisodeTrace::new(Uuid::new_v4());
        
        let base = Episode::new(trace.goal_id, 1, "Test");
        let mut episode = EpisodeV2::from_episode(base);
        
        episode.add_action(EpisodeAction {
            id: Uuid::new_v4(),
            tool_name: "web_fetch".to_string(),
            input: serde_json::json!({}),
            output: Some(serde_json::json!({})),
            error: None,
            timestamp: chrono::Utc::now().timestamp(),
            execution_time_ms: 100,
            success: true,
        });
        
        trace.add_episode(episode);
        
        assert_eq!(trace.total_actions, 1);
        assert_eq!(trace.tools_used(), vec!["web_fetch"]);
    }
}
