//! Swarm Reflection Loop - Iterative consensus

use crate::com::ConsensusOfMeaning;
use crate::rov::ReinforcementOfValue;
use crate::schema::*;
use std::collections::HashMap;
use uuid::Uuid;

/// Swarm round state
#[derive(Debug, Clone)]
pub struct RoundState {
    pub round: u32,
    pub hypotheses: Vec<Hypothesis>,
    pub evidence_map: HashMap<Hash, Vec<Evidence>>,
    pub votes_map: HashMap<Hash, Vec<Vote>>,
    pub weights: Vec<MeaningWeight>,
}

impl RoundState {
    pub fn new(round: u32) -> Self {
        Self {
            round,
            hypotheses: Vec::new(),
            evidence_map: HashMap::new(),
            votes_map: HashMap::new(),
            weights: Vec::new(),
        }
    }
}

/// Swarm consensus result
#[derive(Debug, Clone)]
pub struct SwarmResult {
    pub goal_id: Uuid,
    pub best_hypothesis: Option<Hypothesis>,
    pub final_weight: Option<MeaningWeight>,
    pub rounds: u32,
    pub converged: bool,
    pub total_hypotheses: usize,
    pub total_votes: usize,
}

/// Swarm Reflection Loop
pub struct SwarmLoop {
    config: SwarmConfig,
    com: ConsensusOfMeaning,
    rov: ReinforcementOfValue,
    goal_id: Uuid,
    rounds: Vec<RoundState>,
}

impl SwarmLoop {
    /// Create new swarm loop
    pub fn new(goal_id: Uuid, config: SwarmConfig) -> Self {
        Self {
            com: ConsensusOfMeaning::new(config.clone()),
            rov: ReinforcementOfValue::default(),
            config,
            goal_id,
            rounds: Vec::new(),
        }
    }

    /// Start new round
    pub fn start_round(&mut self) -> u32 {
        let round_num = self.rounds.len() as u32 + 1;
        let state = RoundState::new(round_num);
        self.rounds.push(state);
        round_num
    }

    /// Add hypothesis to current round
    pub fn add_hypothesis(&mut self, hyp: Hypothesis) {
        if let Some(state) = self.rounds.last_mut() {
            state.hypotheses.push(hyp);
        }
    }

    /// Add evidence to current round
    pub fn add_evidence(&mut self, evidence: Evidence) {
        if let Some(state) = self.rounds.last_mut() {
            state.evidence_map
                .entry(evidence.hyp.clone())
                .or_insert_with(Vec::new)
                .push(evidence);
        }
    }

    /// Add vote to current round
    pub fn add_vote(&mut self, vote: Vote) {
        if let Some(state) = self.rounds.last_mut() {
            state.votes_map
                .entry(vote.hyp.clone())
                .or_insert_with(Vec::new)
                .push(vote);
        }
    }

    /// Execute current round
    pub fn execute_round(&mut self) -> Result<Vec<MeaningWeight>, String> {
        let state = self.rounds.last_mut()
            .ok_or("No active round")?;

        // Merge similar hypotheses
        self.com.merge_similar(&mut state.hypotheses);

        // Aggregate meaning
        let weights = self.com.aggregate_all(
            &state.hypotheses,
            &state.votes_map,
            state.round,
        );

        state.weights = weights.clone();
        Ok(weights)
    }

    /// Check if converged
    pub fn check_convergence(&self) -> bool {
        if self.rounds.len() < 2 {
            return false;
        }

        let current = &self.rounds[self.rounds.len() - 1].weights;
        let previous = &self.rounds[self.rounds.len() - 2].weights;

        self.com.check_convergence(current, previous)
    }

    /// Get best hypothesis
    pub fn get_best(&self) -> Option<(Hypothesis, MeaningWeight)> {
        let state = self.rounds.last()?;
        let best_weight = self.com.find_best(&state.weights)?;
        
        let best_hyp = state.hypotheses
            .iter()
            .find(|h| h.id == best_weight.hyp)?;

        Some((best_hyp.clone(), best_weight))
    }

    /// Run complete swarm consensus
    pub async fn run_consensus(&mut self) -> Result<SwarmResult, String> {
        let mut converged = false;

        for round_num in 1..=self.config.max_rounds {
            self.start_round();

            // TODO: Collect hypotheses, evidence, votes from P2P
            
            // Execute round
            let weights = self.execute_round()?;

            // Check for commit
            if let Some(best) = self.com.find_best(&weights) {
                if self.com.can_commit(&best) {
                    // Check convergence
                    if round_num > 1 && self.check_convergence() {
                        converged = true;
                        break;
                    }
                }
            }

            // Reflect and prepare for next round
            if round_num < self.config.max_rounds {
                self.reflect_and_counter().await?;
            }
        }

        self.finalize_result(converged)
    }

    /// Reflect and generate counter-proposals
    async fn reflect_and_counter(&mut self) -> Result<(), String> {
        // TODO: Implement reflection logic
        // - Analyze current weights
        // - Generate counter-hypotheses
        // - Broadcast reflection messages
        Ok(())
    }

    /// Finalize and return result
    fn finalize_result(&self, converged: bool) -> Result<SwarmResult, String> {
        let (best_hyp, final_weight) = self.get_best()
            .map(|(h, w)| (Some(h), Some(w)))
            .unwrap_or((None, None));

        let total_votes: usize = self.rounds
            .iter()
            .map(|r| r.votes_map.values().map(|v| v.len()).sum::<usize>())
            .sum();

        let total_hypotheses: usize = self.rounds
            .iter()
            .map(|r| r.hypotheses.len())
            .sum();

        Ok(SwarmResult {
            goal_id: self.goal_id,
            best_hypothesis: best_hyp,
            final_weight,
            rounds: self.rounds.len() as u32,
            converged,
            total_hypotheses,
            total_votes,
        })
    }

    /// Calculate rewards for final result
    pub fn calculate_rewards(&self) -> Result<(Vec<f64>, Vec<f64>), String> {
        let (best_hyp, final_weight) = self.get_best()
            .ok_or("No best hypothesis found")?;

        let state = self.rounds.last()
            .ok_or("No rounds executed")?;

        let votes = state.votes_map.get(&best_hyp.id)
            .cloned()
            .unwrap_or_default();

        let evidence_count = state.evidence_map.get(&best_hyp.id)
            .map(|e| e.len())
            .unwrap_or(0);

        let node_weights: HashMap<NodeId, f32> = HashMap::new(); // TODO: Get from CoM

        let (author_reward, voter_rewards) = self.rov.calculate_all_rewards(
            &best_hyp,
            &final_weight,
            &votes,
            evidence_count,
            &node_weights,
        );

        let author_rewards = vec![author_reward.reward];
        let voter_reward_values: Vec<f64> = voter_rewards.iter().map(|v| v.reward).collect();

        Ok((author_rewards, voter_reward_values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swarm_loop_creation() {
        let goal_id = Uuid::new_v4();
        let config = SwarmConfig::default();
        let swarm = SwarmLoop::new(goal_id, config);
        
        assert_eq!(swarm.goal_id, goal_id);
        assert_eq!(swarm.rounds.len(), 0);
    }

    #[test]
    fn test_start_round() {
        let goal_id = Uuid::new_v4();
        let config = SwarmConfig::default();
        let mut swarm = SwarmLoop::new(goal_id, config);
        
        let round1 = swarm.start_round();
        assert_eq!(round1, 1);
        
        let round2 = swarm.start_round();
        assert_eq!(round2, 2);
    }

    #[test]
    fn test_add_hypothesis() {
        let goal_id = Uuid::new_v4();
        let config = SwarmConfig::default();
        let mut swarm = SwarmLoop::new(goal_id, config);
        
        swarm.start_round();
        
        let hyp = Hypothesis::new(
            goal_id,
            "Test hypothesis".to_string(),
            vec![0.1; 384],
            "node1".to_string(),
        );
        
        swarm.add_hypothesis(hyp);
        
        assert_eq!(swarm.rounds[0].hypotheses.len(), 1);
    }

    #[test]
    fn test_add_vote() {
        let goal_id = Uuid::new_v4();
        let config = SwarmConfig::default();
        let mut swarm = SwarmLoop::new(goal_id, config);
        
        swarm.start_round();
        
        let vote = Vote::new(
            "hyp1".to_string(),
            0.8,
            0.9,
            0.7,
            0.6,
            "voter1".to_string(),
        );
        
        swarm.add_vote(vote);
        
        assert_eq!(swarm.rounds[0].votes_map.len(), 1);
    }

    #[test]
    fn test_execute_round() {
        let goal_id = Uuid::new_v4();
        let config = SwarmConfig::default();
        let mut swarm = SwarmLoop::new(goal_id, config);
        
        swarm.start_round();
        
        let hyp = Hypothesis::new(
            goal_id,
            "Test".to_string(),
            vec![0.1; 384],
            "node1".to_string(),
        );
        
        let vote = Vote::new(
            hyp.id.clone(),
            0.8,
            0.9,
            0.7,
            0.6,
            "voter1".to_string(),
        );
        
        swarm.add_hypothesis(hyp);
        swarm.add_vote(vote);
        
        let result = swarm.execute_round();
        assert!(result.is_ok());
        
        let weights = result.unwrap();
        assert_eq!(weights.len(), 1);
    }

    #[test]
    fn test_check_convergence() {
        let goal_id = Uuid::new_v4();
        let config = SwarmConfig::default();
        let mut swarm = SwarmLoop::new(goal_id, config);
        
        // Need at least 2 rounds
        assert!(!swarm.check_convergence());
        
        swarm.start_round();
        swarm.start_round();
        
        // Still false without weights
        assert!(!swarm.check_convergence());
    }
}
