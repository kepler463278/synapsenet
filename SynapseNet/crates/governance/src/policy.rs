use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Policy classification for queries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyClass {
    /// Normal response allowed
    Ok,
    /// Only consequences/analysis, no step-by-step instructions
    AnalysisOnly,
    /// Requires curator review
    Curated,
}

/// Policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    /// Default policy class
    pub default: PolicyClass,
    /// Keyword-based rules
    pub rules: Vec<PolicyRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    /// Keywords to match (case-insensitive)
    pub keywords: Vec<String>,
    /// Policy class to apply
    pub class: PolicyClass,
    /// Optional explanation
    pub reason: Option<String>,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            default: PolicyClass::Ok,
            rules: vec![
                PolicyRule {
                    keywords: vec![
                        "bomb".to_string(),
                        "weapon".to_string(),
                        "explosive".to_string(),
                    ],
                    class: PolicyClass::AnalysisOnly,
                    reason: Some("Potential harm".to_string()),
                },
                PolicyRule {
                    keywords: vec![
                        "hack".to_string(),
                        "exploit".to_string(),
                        "vulnerability".to_string(),
                    ],
                    class: PolicyClass::AnalysisOnly,
                    reason: Some("Security concern".to_string()),
                },
            ],
        }
    }
}

/// Policy engine for query classification
pub struct PolicyEngine {
    policy: Policy,
}

impl PolicyEngine {
    pub fn new(policy: Policy) -> Self {
        Self { policy }
    }

    /// Classify query based on policy rules
    pub fn classify(&self, query: &str) -> PolicyClass {
        let query_lower = query.to_lowercase();

        for rule in &self.policy.rules {
            for keyword in &rule.keywords {
                if query_lower.contains(&keyword.to_lowercase()) {
                    return rule.class;
                }
            }
        }

        self.policy.default
    }

    /// Generate response based on policy class
    pub fn generate_response(&self, class: PolicyClass, query: &str, results: &[String]) -> String {
        match class {
            PolicyClass::Ok => {
                // Normal response
                format!("Query: {}\n\nResults:\n{}", query, results.join("\n"))
            }
            PolicyClass::AnalysisOnly => {
                // Consequences only, no instructions
                format!(
                    "Query: {}\n\n⚠️ ANALYSIS MODE\n\n\
                    This query involves potentially harmful content. \
                    Instead of providing step-by-step instructions, \
                    here are the consequences and risks:\n\n{}\n\n\
                    Legal implications: [varies by jurisdiction]\n\
                    Physical risks: [potential harm]\n\
                    Ethical considerations: [community impact]\n\n\
                    For legitimate research or educational purposes, \
                    consult appropriate authorities and experts.",
                    query,
                    results.join("\n")
                )
            }
            PolicyClass::Curated => {
                // Queue for review
                format!(
                    "Query: {}\n\n⏳ CURATOR REVIEW REQUIRED\n\n\
                    This query has been flagged for human review. \
                    A curator will assess the request and provide \
                    an appropriate response.\n\n\
                    Query ID: {}",
                    query,
                    blake3::hash(query.as_bytes()).to_hex()
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_classification() {
        let engine = PolicyEngine::new(Policy::default());

        assert_eq!(engine.classify("What is Rust?"), PolicyClass::Ok);
        assert_eq!(
            engine.classify("How to make a bomb?"),
            PolicyClass::AnalysisOnly
        );
        assert_eq!(
            engine.classify("Exploit vulnerability"),
            PolicyClass::AnalysisOnly
        );
    }
}
