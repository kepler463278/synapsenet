//! P2P message handlers for swarm consensus

use crate::swarm_msgs::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use synapsenet_swarm::*;
use uuid::Uuid;

/// Rate limiting for swarm messages
#[derive(Debug, Clone)]
pub struct SwarmRateLimit {
    pub proposals_per_minute: u32,
    pub votes_per_minute: u32,
    pub evidence_per_minute: u32,
}

impl Default for SwarmRateLimit {
    fn default() -> Self {
        Self {
            proposals_per_minute: 5,
            votes_per_minute: 30,
            evidence_per_minute: 20,
        }
    }
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

    fn check_rate(&mut self, key: &str, limit: u32) -> bool {
        let now = chrono::Utc::now().timestamp();
        let window_start = now - 60;

        let timestamps = self.windows.entry(key.to_string()).or_insert_with(Vec::new);
        timestamps.retain(|&t| t > window_start);

        if timestamps.len() >= limit as usize {
            return false;
        }

        timestamps.push(now);
        true
    }
}

/// Swarm message handler
pub struct SwarmHandler {
    rate_limits: SwarmRateLimit,
    rate_limiter: Arc<Mutex<RateLimiter>>,
    validator: SwarmMessageValidator,
    active_goals: Arc<Mutex<HashMap<Uuid, SwarmGoalState>>>,
}

/// State for a goal's swarm consensus
#[derive(Debug)]
struct SwarmGoalState {
    goal_id: Uuid,
    current_round: u32,
    hypotheses: Vec<Hypothesis>,
    evidence_map: HashMap<Hash, Vec<Evidence>>,
    votes_map: HashMap<Hash, Vec<Vote>>,
    commits: Vec<CommitMessage>,
}

impl SwarmGoalState {
    fn new(goal_id: Uuid) -> Self {
        Self {
            goal_id,
            current_round: 0,
            hypotheses: Vec::new(),
            evidence_map: HashMap::new(),
            votes_map: HashMap::new(),
            commits: Vec::new(),
        }
    }
}

impl SwarmHandler {
    /// Create new swarm handler
    pub fn new(rate_limits: SwarmRateLimit) -> Self {
        Self {
            rate_limits,
            rate_limiter: Arc::new(Mutex::new(RateLimiter::new())),
            validator: SwarmMessageValidator::new(),
            active_goals: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Handle incoming swarm message
    pub async fn handle_message(&self, msg: SwarmMessage) -> Result<(), String> {
        // Validate message
        let validation = self.validator.validate(&msg);
        if !validation.valid {
            return Err(validation.reason.unwrap_or("Invalid message".to_string()));
        }

        // Check rate limits
        if !self.check_rate_limit(&msg) {
            return Err("Rate limit exceeded".to_string());
        }

        // Handle specific message types
        match msg {
            SwarmMessage::Propose(proposal) => self.handle_proposal(proposal).await,
            SwarmMessage::Evidence(submission) => self.handle_evidence(submission).await,
            SwarmMessage::Vote(submission) => self.handle_vote(submission).await,
            SwarmMessage::Commit(commit) => self.handle_commit(commit).await,
            SwarmMessage::Reflect(reflection) => self.handle_reflection(reflection).await,
        }
    }

    /// Handle hypothesis proposal
    async fn handle_proposal(&self, proposal: HypothesisProposal) -> Result<(), String> {
        tracing::info!("Received hypothesis proposal: {}", proposal.hypothesis.id);

        let mut goals = self.active_goals.lock().unwrap();
        let state = goals
            .entry(proposal.hypothesis.goal_id)
            .or_insert_with(|| SwarmGoalState::new(proposal.hypothesis.goal_id));

        // Check for duplicates
        if state.hypotheses.iter().any(|h| h.id == proposal.hypothesis.id) {
            return Ok(()); // Already have this hypothesis
        }

        // Check similarity with existing hypotheses
        for existing in &state.hypotheses {
            if existing.similarity(&proposal.hypothesis) > 0.9 {
                tracing::debug!("Merging similar hypothesis");
                return Ok(());
            }
        }

        state.hypotheses.push(proposal.hypothesis);
        Ok(())
    }

    /// Handle evidence submission
    async fn handle_evidence(&self, submission: EvidenceSubmission) -> Result<(), String> {
        tracing::info!("Received evidence for hypothesis: {}", submission.evidence.hyp);

        let mut goals = self.active_goals.lock().unwrap();

        // Find the goal this evidence belongs to
        for state in goals.values_mut() {
            if state.hypotheses.iter().any(|h| h.id == submission.evidence.hyp) {
                state
                    .evidence_map
                    .entry(submission.evidence.hyp.clone())
                    .or_insert_with(Vec::new)
                    .push(submission.evidence);
                return Ok(());
            }
        }

        Err("Hypothesis not found for evidence".to_string())
    }

    /// Handle vote submission
    async fn handle_vote(&self, submission: VoteSubmission) -> Result<(), String> {
        tracing::info!("Received vote for hypothesis: {}", submission.vote.hyp);

        let mut goals = self.active_goals.lock().unwrap();

        // Find the goal this vote belongs to
        for state in goals.values_mut() {
            if state.hypotheses.iter().any(|h| h.id == submission.vote.hyp) {
                state
                    .votes_map
                    .entry(submission.vote.hyp.clone())
                    .or_insert_with(Vec::new)
                    .push(submission.vote);
                return Ok(());
            }
        }

        Err("Hypothesis not found for vote".to_string())
    }

    /// Handle commit message
    async fn handle_commit(&self, commit: CommitMessage) -> Result<(), String> {
        tracing::info!("Received commit for hypothesis: {}", commit.weight.hyp);

        let mut goals = self.active_goals.lock().unwrap();

        // Find the goal this commit belongs to
        for state in goals.values_mut() {
            if state.hypotheses.iter().any(|h| h.id == commit.weight.hyp) {
                state.commits.push(commit);
                return Ok(());
            }
        }

        Err("Hypothesis not found for commit".to_string())
    }

    /// Handle reflection message
    async fn handle_reflection(&self, reflection: ReflectionMessage) -> Result<(), String> {
        tracing::info!("Received reflection for goal: {}", reflection.goal_id);

        // TODO: Process reflection and counter-proposals

        Ok(())
    }

    /// Check rate limit for message
    fn check_rate_limit(&self, msg: &SwarmMessage) -> bool {
        let mut limiter = self.rate_limiter.lock().unwrap();

        let (key, limit) = match msg {
            SwarmMessage::Propose(p) => (
                format!("propose_{}", p.hypothesis.author),
                self.rate_limits.proposals_per_minute,
            ),
            SwarmMessage::Vote(v) => (
                format!("vote_{}", v.vote.voter),
                self.rate_limits.votes_per_minute,
            ),
            SwarmMessage::Evidence(e) => (
                format!("evidence_{}", e.evidence.hyp),
                self.rate_limits.evidence_per_minute,
            ),
            _ => return true, // No rate limit for other types
        };

        limiter.check_rate(&key, limit)
    }

    /// Get state for a goal
    pub fn get_goal_state(&self, goal_id: &Uuid) -> Option<(usize, usize, usize)> {
        let goals = self.active_goals.lock().unwrap();
        goals.get(goal_id).map(|state| {
            (
                state.hypotheses.len(),
                state.votes_map.values().map(|v| v.len()).sum(),
                state.commits.len(),
            )
        })
    }

    /// Clear goal state
    pub fn clear_goal(&self, goal_id: &Uuid) {
        let mut goals = self.active_goals.lock().unwrap();
        goals.remove(goal_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let handler = SwarmHandler::new(SwarmRateLimit::default());
        assert_eq!(handler.rate_limits.proposals_per_minute, 5);
    }

    #[tokio::test]
    async fn test_handle_proposal() {
        let handler = SwarmHandler::new(SwarmRateLimit::default());

        let hyp = Hypothesis::new(
            Uuid::new_v4(),
            "Test hypothesis".to_string(),
            vec![0.1; 384],
            "node1".to_string(),
        );

        let proposal = HypothesisProposal {
            hypothesis: hyp.clone(),
            round: 1,
        };

        let result = handler.handle_proposal(proposal).await;
        assert!(result.is_ok());

        let state = handler.get_goal_state(&hyp.goal_id);
        assert!(state.is_some());
        assert_eq!(state.unwrap().0, 1); // 1 hypothesis
    }

    #[tokio::test]
    async fn test_rate_limiting() {
        let handler = SwarmHandler::new(SwarmRateLimit {
            proposals_per_minute: 2,
            votes_per_minute: 30,
            evidence_per_minute: 20,
        });

        let goal_id = Uuid::new_v4();

        // Send 3 proposals (limit is 2)
        for i in 0..3 {
            let hyp = Hypothesis::new(
                goal_id,
                format!("Test {}", i),
                vec![0.1; 384],
                "node1".to_string(),
            );

            let msg = SwarmMessage::Propose(HypothesisProposal {
                hypothesis: hyp,
                round: 1,
            });

            let result = handler.handle_message(msg).await;

            if i < 2 {
                assert!(result.is_ok());
            } else {
                assert!(result.is_err()); // Should be rate limited
            }
        }
    }

    #[tokio::test]
    async fn test_duplicate_detection() {
        let handler = SwarmHandler::new(SwarmRateLimit::default());

        let hyp = Hypothesis::new(
            Uuid::new_v4(),
            "Test".to_string(),
            vec![0.1; 384],
            "node1".to_string(),
        );

        // Send same hypothesis twice
        let proposal1 = HypothesisProposal {
            hypothesis: hyp.clone(),
            round: 1,
        };

        let proposal2 = HypothesisProposal {
            hypothesis: hyp.clone(),
            round: 1,
        };

        handler.handle_proposal(proposal1).await.unwrap();
        handler.handle_proposal(proposal2).await.unwrap();

        let state = handler.get_goal_state(&hyp.goal_id);
        assert_eq!(state.unwrap().0, 1); // Still only 1 hypothesis
    }
}
