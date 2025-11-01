//! P2P message handlers for reasoning

use crate::reason_msgs::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Rate limiting configuration
#[derive(Debug, Clone)]
pub struct RateLimit {
    pub goals_per_minute: u32,
    pub queries_per_minute: u32,
    pub insights_per_minute: u32,
}

impl Default for RateLimit {
    fn default() -> Self {
        Self {
            goals_per_minute: 10,
            queries_per_minute: 60,
            insights_per_minute: 30,
        }
    }
}

/// Peer reputation
#[derive(Debug, Clone)]
pub struct PeerReputation {
    pub peer_id: String,
    pub score: f64,
    pub last_updated: i64,
    pub message_count: u64,
}

/// Rate limiter
#[derive(Debug)]
struct RateLimiter {
    windows: HashMap<String, Vec<i64>>,
}

impl RateLimiter {
    fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    fn check_rate(&mut self, peer_id: &str, limit: u32) -> bool {
        let now = chrono::Utc::now().timestamp();
        let window_start = now - 60;
        
        let timestamps = self.windows.entry(peer_id.to_string()).or_insert_with(Vec::new);
        
        timestamps.retain(|&t| t > window_start);
        
        if timestamps.len() >= limit as usize {
            return false;
        }
        
        timestamps.push(now);
        true
    }
}

/// Reasoning message handler
pub struct ReasoningHandler {
    rate_limits: RateLimit,
    rate_limiter: Arc<Mutex<RateLimiter>>,
    reputations: Arc<Mutex<HashMap<String, PeerReputation>>>,
    validator: MessageValidator,
}

impl ReasoningHandler {
    /// Create new handler
    pub fn new(rate_limits: RateLimit) -> Self {
        Self {
            rate_limits,
            rate_limiter: Arc::new(Mutex::new(RateLimiter::new())),
            reputations: Arc::new(Mutex::new(HashMap::new())),
            validator: MessageValidator::new(),
        }
    }

    /// Handle incoming message
    pub async fn handle_message(&self, msg: ReasoningMessage) -> Result<(), String> {
        let validation = self.validator.validate(&msg);
        if !validation.valid {
            return Err(validation.reason.unwrap_or("Invalid message".to_string()));
        }

        let peer_id = msg.sender();
        if self.get_reputation(peer_id) < -5.0 {
            return Err("Peer reputation too low".to_string());
        }

        if !self.check_rate_limit(&msg) {
            self.update_reputation(peer_id, -0.1, "Rate limit exceeded");
            return Err("Rate limit exceeded".to_string());
        }

        match msg {
            ReasoningMessage::GoalSubmit(goal) => self.handle_goal_submit(goal).await,
            ReasoningMessage::PlanShare(plan) => self.handle_plan_share(plan).await,
            ReasoningMessage::InsightBroadcast(insight) => self.handle_insight_broadcast(insight).await,
            ReasoningMessage::QueryRequest(query) => self.handle_query_request(query).await,
            ReasoningMessage::QueryResponse(response) => self.handle_query_response(response).await,
            ReasoningMessage::ReputationUpdate(update) => self.handle_reputation_update(update).await,
        }
    }

    async fn handle_goal_submit(&self, goal: GoalSubmit) -> Result<(), String> {
        tracing::info!("Received goal submission: {}", goal.goal_id);
        self.update_reputation(&goal.requester, 0.1, "Valid goal submission");
        Ok(())
    }

    async fn handle_plan_share(&self, plan: PlanShare) -> Result<(), String> {
        tracing::info!("Received plan share: {}", plan.plan_id);
        self.update_reputation(&plan.sharer, 0.1, "Valid plan share");
        Ok(())
    }

    async fn handle_insight_broadcast(&self, insight: InsightBroadcast) -> Result<(), String> {
        tracing::info!("Received insight broadcast: {}", insight.insight_id);
        self.update_reputation(&insight.broadcaster, 0.1, "Valid insight");
        Ok(())
    }

    async fn handle_query_request(&self, query: QueryRequest) -> Result<(), String> {
        tracing::info!("Received query request: {}", query.query_id);
        Ok(())
    }

    async fn handle_query_response(&self, response: QueryResponse) -> Result<(), String> {
        tracing::info!("Received query response: {}", response.query_id);
        Ok(())
    }

    async fn handle_reputation_update(&self, update: ReputationUpdate) -> Result<(), String> {
        tracing::info!("Received reputation update for: {}", update.peer_id);
        Ok(())
    }

    fn check_rate_limit(&self, msg: &ReasoningMessage) -> bool {
        let mut limiter = self.rate_limiter.lock().unwrap();
        let peer_id = msg.sender();
        
        match msg {
            ReasoningMessage::GoalSubmit(_) => {
                limiter.check_rate(peer_id, self.rate_limits.goals_per_minute)
            }
            ReasoningMessage::QueryRequest(_) => {
                limiter.check_rate(peer_id, self.rate_limits.queries_per_minute)
            }
            ReasoningMessage::InsightBroadcast(_) => {
                limiter.check_rate(peer_id, self.rate_limits.insights_per_minute)
            }
            _ => true,
        }
    }

    fn get_reputation(&self, peer_id: &str) -> f64 {
        let reputations = self.reputations.lock().unwrap();
        reputations.get(peer_id).map(|r| r.score).unwrap_or(0.0)
    }

    fn update_reputation(&self, peer_id: &str, delta: f64, reason: &str) {
        let mut reputations = self.reputations.lock().unwrap();
        let reputation = reputations.entry(peer_id.to_string()).or_insert_with(|| {
            PeerReputation {
                peer_id: peer_id.to_string(),
                score: 0.0,
                last_updated: chrono::Utc::now().timestamp(),
                message_count: 0,
            }
        });
        
        reputation.score += delta;
        reputation.score = reputation.score.clamp(-10.0, 10.0);
        reputation.last_updated = chrono::Utc::now().timestamp();
        reputation.message_count += 1;
        
        tracing::debug!("Updated reputation for {}: {} ({})", peer_id, reputation.score, reason);
    }

    pub fn get_all_reputations(&self) -> HashMap<String, PeerReputation> {
        self.reputations.lock().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handler_creation() {
        let handler = ReasoningHandler::new(RateLimit::default());
        assert_eq!(handler.get_reputation("test_peer"), 0.0);
    }

    #[tokio::test]
    async fn test_reputation_update() {
        let handler = ReasoningHandler::new(RateLimit::default());
        
        handler.update_reputation("peer_123", 0.5, "Test");
        assert_eq!(handler.get_reputation("peer_123"), 0.5);
        
        handler.update_reputation("peer_123", -0.2, "Test");
        assert_eq!(handler.get_reputation("peer_123"), 0.3);
    }

    #[tokio::test]
    async fn test_rate_limiting() {
        let mut limiter = RateLimiter::new();
        
        for _ in 0..5 {
            assert!(limiter.check_rate("peer_123", 10));
        }
        
        for _ in 0..5 {
            assert!(limiter.check_rate("peer_123", 10));
        }
        
        assert!(!limiter.check_rate("peer_123", 10));
    }
}
