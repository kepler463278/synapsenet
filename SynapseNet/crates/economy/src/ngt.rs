use std::collections::HashMap;
use synapsenet_core::poe::Credit;
use synapsenet_core::ProofOfEmergence;

/// NGT (Neural Graph Token) ledger
#[derive(Debug, Default)]
pub struct NgtLedger {
    /// Node balances
    balances: HashMap<[u8; 32], f64>,
    /// PoE calculator
    pub poe: ProofOfEmergence,
}

impl NgtLedger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_poe(poe: ProofOfEmergence) -> Self {
        Self {
            balances: HashMap::new(),
            poe,
        }
    }

    /// Award NGT for grain contribution
    pub fn award(
        &mut self,
        node_pk: [u8; 32],
        novelty: f32,
        coherence: f32,
        reuse_count: u32,
    ) -> f64 {
        let ngt = self.poe.calculate_ngt(novelty, coherence, reuse_count);

        if ngt > 0.0 {
            *self.balances.entry(node_pk).or_insert(0.0) += ngt;
        }

        ngt
    }

    /// Get node balance
    pub fn balance(&self, node_pk: &[u8; 32]) -> f64 {
        self.balances.get(node_pk).copied().unwrap_or(0.0)
    }

    /// Get total supply
    pub fn total_supply(&self) -> f64 {
        self.balances.values().sum()
    }

    /// Get top holders
    pub fn top_holders(&self, n: usize) -> Vec<([u8; 32], f64)> {
        let mut holders: Vec<_> = self.balances.iter().map(|(k, v)| (*k, *v)).collect();
        holders.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        holders.truncate(n);
        holders
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ngt_award() {
        let mut ledger = NgtLedger::new();
        let node_pk = [1u8; 32];

        // High novelty, medium coherence
        let ngt = ledger.award(node_pk, 0.8, 0.5, 0);
        assert!(ngt > 0.0);

        assert_eq!(ledger.balance(&node_pk), ngt);
        assert_eq!(ledger.total_supply(), ngt);
    }
}
