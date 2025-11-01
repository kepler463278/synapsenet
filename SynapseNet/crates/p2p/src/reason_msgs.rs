//! P2P messages for reasoning system

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Goal submission message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalSubmit {
    pub goal_id: Uuid,
    pub goal_text: String,
    pub priority: u8,
    pub requester: String,
    pub timestamp: i64,
    pub signature: Option<String>,
}

/// Plan sharing message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanShare {
    pub plan_id: Uuid,
    pub goal_id: Uuid,
    pub dag_json: String,
    pub complexity: f64,
    pub sharer: String,
    pub timestamp: i64,
    pub signature: Option<String>,
}

/// Insight broadcast message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsightBroadcast {
    pub insight_id: Uuid,
    pub goal_id: Option<Uuid>,
    pub content_hash: String,
    pub confidence: f64,
    pub grain_refs: Vec<String>,
    pub broadcaster: String,
    pub timestamp: i64,
    pub signature: Option<String>,
}

/// Query request message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequest {
    pub query_id: Uuid,
    pub query_text: String,
    pub query_vec: Vec<f32>,
    pub max_results: usize,
    pub min_similarity: f64,
    pub requester: String,
    pub timestamp: i64,
}

/// Query response message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResponse {
    pub query_id: Uuid,
    pub results: Vec<QueryResult>,
    pub responder: String,
    pub timestamp: i64,
}

/// Single query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub grain_id: String,
    pub similarity: f64,
    pub snippet: Option<String>,
    pub metadata: serde_json::Value,
}

/// Reputation update message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationUpdate {
    pub peer_id: String,
    pub delta: f64,
    pub reason: String,
    pub evidence: Option<String>,
    pub updater: String,
    pub timestamp: i64,
}

/// Message envelope for all reasoning messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ReasoningMessage {
    GoalSubmit(GoalSubmit),
    PlanShare(PlanShare),
    InsightBroadcast(InsightBroadcast),
    QueryRequest(QueryRequest),
    QueryResponse(QueryResponse),
    ReputationUpdate(ReputationUpdate),
}

impl ReasoningMessage {
    /// Get message timestamp
    pub fn timestamp(&self) -> i64 {
        match self {
            Self::GoalSubmit(msg) => msg.timestamp,
            Self::PlanShare(msg) => msg.timestamp,
            Self::InsightBroadcast(msg) => msg.timestamp,
            Self::QueryRequest(msg) => msg.timestamp,
            Self::QueryResponse(msg) => msg.timestamp,
            Self::ReputationUpdate(msg) => msg.timestamp,
        }
    }

    /// Get sender ID
    pub fn sender(&self) -> &str {
        match self {
            Self::GoalSubmit(msg) => &msg.requester,
            Self::PlanShare(msg) => &msg.sharer,
            Self::InsightBroadcast(msg) => &msg.broadcaster,
            Self::QueryRequest(msg) => &msg.requester,
            Self::QueryResponse(msg) => &msg.responder,
            Self::ReputationUpdate(msg) => &msg.updater,
        }
    }

    /// Check if message has signature
    pub fn is_signed(&self) -> bool {
        match self {
            Self::GoalSubmit(msg) => msg.signature.is_some(),
            Self::PlanShare(msg) => msg.signature.is_some(),
            Self::InsightBroadcast(msg) => msg.signature.is_some(),
            Self::QueryRequest(_) => false,
            Self::QueryResponse(_) => false,
            Self::ReputationUpdate(_) => true,
        }
    }
}

/// Message validation result
#[derive(Debug)]
pub struct ValidationResult {
    pub valid: bool,
    pub reason: Option<String>,
}

/// Message validator
pub struct MessageValidator {
    max_message_age: i64,
}

impl MessageValidator {
    pub fn new() -> Self {
        Self {
            max_message_age: 3600, // 1 hour
        }
    }

    /// Validate message
    pub fn validate(&self, msg: &ReasoningMessage) -> ValidationResult {
        let now = chrono::Utc::now().timestamp();
        let age = now - msg.timestamp();
        
        if age > self.max_message_age {
            return ValidationResult {
                valid: false,
                reason: Some("Message too old".to_string()),
            };
        }
        
        if age < -300 {
            return ValidationResult {
                valid: false,
                reason: Some("Message from future".to_string()),
            };
        }
        
        match msg {
            ReasoningMessage::GoalSubmit(goal) => {
                if goal.goal_text.is_empty() {
                    return ValidationResult {
                        valid: false,
                        reason: Some("Empty goal text".to_string()),
                    };
                }
            }
            ReasoningMessage::QueryRequest(query) => {
                if query.query_vec.len() != 384 {
                    return ValidationResult {
                        valid: false,
                        reason: Some("Invalid query vector size".to_string()),
                    };
                }
            }
            _ => {}
        }
        
        ValidationResult {
            valid: true,
            reason: None,
        }
    }
}

impl Default for MessageValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goal_submit() {
        let msg = GoalSubmit {
            goal_id: Uuid::new_v4(),
            goal_text: "Test goal".to_string(),
            priority: 1,
            requester: "peer_123".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            signature: None,
        };
        
        assert_eq!(msg.goal_text, "Test goal");
        assert_eq!(msg.priority, 1);
    }

    #[test]
    fn test_message_envelope() {
        let goal = GoalSubmit {
            goal_id: Uuid::new_v4(),
            goal_text: "Test".to_string(),
            priority: 1,
            requester: "peer_123".to_string(),
            timestamp: 1234567890,
            signature: None,
        };
        
        let msg = ReasoningMessage::GoalSubmit(goal);
        assert_eq!(msg.timestamp(), 1234567890);
        assert_eq!(msg.sender(), "peer_123");
        assert!(!msg.is_signed());
    }

    #[test]
    fn test_message_validation() {
        let validator = MessageValidator::new();
        
        let goal = GoalSubmit {
            goal_id: Uuid::new_v4(),
            goal_text: "Test goal".to_string(),
            priority: 1,
            requester: "peer_123".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            signature: None,
        };
        
        let msg = ReasoningMessage::GoalSubmit(goal);
        let result = validator.validate(&msg);
        assert!(result.valid);
    }

    #[test]
    fn test_old_message_validation() {
        let validator = MessageValidator::new();
        
        let goal = GoalSubmit {
            goal_id: Uuid::new_v4(),
            goal_text: "Test goal".to_string(),
            priority: 1,
            requester: "peer_123".to_string(),
            timestamp: chrono::Utc::now().timestamp() - 7200,
            signature: None,
        };
        
        let msg = ReasoningMessage::GoalSubmit(goal);
        let result = validator.validate(&msg);
        assert!(!result.valid);
    }
}
