//! P2P messages for swarm consensus

use serde::{Deserialize, Serialize};
use synapsenet_swarm::*;

/// Hypothesis proposal message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypothesisProposal {
    pub hypothesis: Hypothesis,
    pub round: u32,
}

/// Evidence submission message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceSubmission {
    pub evidence: Evidence,
    pub round: u32,
}

/// Vote submission message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteSubmission {
    pub vote: Vote,
    pub round: u32,
}

/// Commit message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitMessage {
    pub weight: MeaningWeight,
    pub round: u32,
    pub node_id: NodeId,
    pub signature: Signature,
}

/// Reflection message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionMessage {
    pub goal_id: uuid::Uuid,
    pub round: u32,
    pub analysis: String,
    pub counter_proposals: Vec<Hash>,
}

/// Swarm message envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SwarmMessage {
    Propose(HypothesisProposal),
    Evidence(EvidenceSubmission),
    Vote(VoteSubmission),
    Commit(CommitMessage),
    Reflect(ReflectionMessage),
}

impl SwarmMessage {
    /// Get message round
    pub fn round(&self) -> u32 {
        match self {
            Self::Propose(msg) => msg.round,
            Self::Evidence(msg) => msg.round,
            Self::Vote(msg) => msg.round,
            Self::Commit(msg) => msg.round,
            Self::Reflect(msg) => msg.round,
        }
    }

    /// Get message type name
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Propose(_) => "propose",
            Self::Evidence(_) => "evidence",
            Self::Vote(_) => "vote",
            Self::Commit(_) => "commit",
            Self::Reflect(_) => "reflect",
        }
    }

    /// Check if message is signed
    pub fn is_signed(&self) -> bool {
        match self {
            Self::Propose(msg) => !msg.hypothesis.sig.is_empty(),
            Self::Evidence(msg) => !msg.evidence.sig.is_empty(),
            Self::Vote(msg) => !msg.vote.sig.is_empty(),
            Self::Commit(msg) => !msg.signature.is_empty(),
            Self::Reflect(_) => false,
        }
    }
}

/// Message validation result
#[derive(Debug)]
pub struct MessageValidation {
    pub valid: bool,
    pub reason: Option<String>,
}

/// Message validator
pub struct SwarmMessageValidator {
    max_message_age: i64,
    max_content_length: usize,
}

impl SwarmMessageValidator {
    pub fn new() -> Self {
        Self {
            max_message_age: 3600, // 1 hour
            max_content_length: 512,
        }
    }

    /// Validate message
    pub fn validate(&self, msg: &SwarmMessage) -> MessageValidation {
        match msg {
            SwarmMessage::Propose(proposal) => self.validate_hypothesis(&proposal.hypothesis),
            SwarmMessage::Evidence(submission) => self.validate_evidence(&submission.evidence),
            SwarmMessage::Vote(submission) => self.validate_vote(&submission.vote),
            SwarmMessage::Commit(commit) => self.validate_commit(commit),
            SwarmMessage::Reflect(reflection) => self.validate_reflection(reflection),
        }
    }

    fn validate_hypothesis(&self, hyp: &Hypothesis) -> MessageValidation {
        let now = chrono::Utc::now().timestamp();
        let age = now - hyp.timestamp;

        if age > self.max_message_age {
            return MessageValidation {
                valid: false,
                reason: Some("Hypothesis too old".to_string()),
            };
        }

        if hyp.content.len() > self.max_content_length {
            return MessageValidation {
                valid: false,
                reason: Some("Content too long".to_string()),
            };
        }

        if hyp.vec.len() != 384 {
            return MessageValidation {
                valid: false,
                reason: Some("Invalid embedding size".to_string()),
            };
        }

        if !hyp.is_valid() {
            return MessageValidation {
                valid: false,
                reason: Some("Invalid hypothesis".to_string()),
            };
        }

        MessageValidation {
            valid: true,
            reason: None,
        }
    }

    fn validate_evidence(&self, evidence: &Evidence) -> MessageValidation {
        if !evidence.is_valid() {
            return MessageValidation {
                valid: false,
                reason: Some("Invalid evidence".to_string()),
            };
        }

        MessageValidation {
            valid: true,
            reason: None,
        }
    }

    fn validate_vote(&self, vote: &Vote) -> MessageValidation {
        if !vote.is_valid() {
            return MessageValidation {
                valid: false,
                reason: Some("Invalid vote".to_string()),
            };
        }

        MessageValidation {
            valid: true,
            reason: None,
        }
    }

    fn validate_commit(&self, _commit: &CommitMessage) -> MessageValidation {
        // TODO: Verify signature
        MessageValidation {
            valid: true,
            reason: None,
        }
    }

    fn validate_reflection(&self, _reflection: &ReflectionMessage) -> MessageValidation {
        MessageValidation {
            valid: true,
            reason: None,
        }
    }
}

impl Default for SwarmMessageValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_hypothesis_proposal() {
        let hyp = Hypothesis::new(
            Uuid::new_v4(),
            "Test".to_string(),
            vec![0.1; 384],
            "node1".to_string(),
        );

        let proposal = HypothesisProposal {
            hypothesis: hyp,
            round: 1,
        };

        assert_eq!(proposal.round, 1);
    }

    #[test]
    fn test_swarm_message_envelope() {
        let hyp = Hypothesis::new(
            Uuid::new_v4(),
            "Test".to_string(),
            vec![0.1; 384],
            "node1".to_string(),
        );

        let msg = SwarmMessage::Propose(HypothesisProposal {
            hypothesis: hyp,
            round: 1,
        });

        assert_eq!(msg.round(), 1);
        assert_eq!(msg.type_name(), "propose");
    }

    #[test]
    fn test_message_validation() {
        let validator = SwarmMessageValidator::new();

        let hyp = Hypothesis::new(
            Uuid::new_v4(),
            "Test hypothesis".to_string(),
            vec![0.1; 384],
            "node1".to_string(),
        );

        let msg = SwarmMessage::Propose(HypothesisProposal {
            hypothesis: hyp,
            round: 1,
        });

        let result = validator.validate(&msg);
        assert!(result.valid);
    }

    #[test]
    fn test_invalid_hypothesis_length() {
        let validator = SwarmMessageValidator::new();

        let long_content = "a".repeat(600);
        let hyp = Hypothesis::new(
            Uuid::new_v4(),
            long_content,
            vec![0.1; 384],
            "node1".to_string(),
        );

        let msg = SwarmMessage::Propose(HypothesisProposal {
            hypothesis: hyp,
            round: 1,
        });

        let result = validator.validate(&msg);
        assert!(!result.valid);
    }

    #[test]
    fn test_invalid_embedding_size() {
        let validator = SwarmMessageValidator::new();

        let hyp = Hypothesis {
            id: "h1".to_string(),
            goal_id: Uuid::new_v4(),
            content: "Test".to_string(),
            vec: vec![0.1; 100], // Wrong size
            author: "node1".to_string(),
            sig: String::new(),
            timestamp: chrono::Utc::now().timestamp(),
        };

        let msg = SwarmMessage::Propose(HypothesisProposal {
            hypothesis: hyp,
            round: 1,
        });

        let result = validator.validate(&msg);
        assert!(!result.valid);
    }
}
