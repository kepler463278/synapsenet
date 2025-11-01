//! Reflection - consistency checking and verification

use crate::episodes::Episode;
use serde::{Deserialize, Serialize};

/// Reflection result
#[derive(Debug, Serialize, Deserialize)]
pub struct ReflectionResult {
    pub consistent: bool,
    pub issues: Vec<String>,
    pub confidence_adjustment: f64,
}

/// Reflector for consistency checking
pub struct Reflector {
    min_confidence: f64,
    max_contradictions: usize,
}

impl Reflector {
    /// Create new reflector
    pub fn new() -> Self {
        Self {
            min_confidence: 0.5,
            max_contradictions: 2,
        }
    }

    /// Reflect on episodes
    pub fn reflect(&self, episodes: &[Episode]) -> Result<ReflectionResult, String> {
        let mut issues = Vec::new();
        
        // Check confidence levels
        let low_confidence = episodes.iter()
            .filter(|e| e.confidence < self.min_confidence)
            .count();
        
        if low_confidence > 0 {
            issues.push(format!("{} episodes with low confidence", low_confidence));
        }
        
        // Check for duplicates
        let duplicates = self.check_duplicates(episodes);
        if duplicates > 0 {
            issues.push(format!("{} duplicate episodes detected", duplicates));
        }
        
        // Check consistency
        let contradictions = self.check_contradictions(episodes);
        if contradictions > self.max_contradictions {
            issues.push(format!("{} contradictions found", contradictions));
        }
        
        let consistent = issues.is_empty();
        let confidence_adjustment = if consistent { 0.0 } else { -0.1 };
        
        Ok(ReflectionResult {
            consistent,
            issues,
            confidence_adjustment,
        })
    }

    /// Check for duplicate episodes
    fn check_duplicates(&self, episodes: &[Episode]) -> usize {
        let mut seen = std::collections::HashSet::new();
        let mut duplicates = 0;
        
        for episode in episodes {
            if !seen.insert(&episode.query) {
                duplicates += 1;
            }
        }
        
        duplicates
    }

    /// Check for contradictions (simplified)
    fn check_contradictions(&self, _episodes: &[Episode]) -> usize {
        // TODO: Implement actual contradiction detection
        0
    }
}

impl Default for Reflector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_reflector_creation() {
        let _reflector = Reflector::new();
    }

    #[test]
    fn test_reflect_empty() {
        let reflector = Reflector::new();
        let result = reflector.reflect(&[]).unwrap();
        assert!(result.consistent);
    }

    #[test]
    fn test_reflect_low_confidence() {
        let reflector = Reflector::new();
        
        let mut episode = Episode::new(Uuid::new_v4(), 1, "Test");
        episode.set_synthesis("Answer", 0.3); // Low confidence
        
        let result = reflector.reflect(&[episode]).unwrap();
        assert!(!result.consistent);
        assert!(!result.issues.is_empty());
    }
}
