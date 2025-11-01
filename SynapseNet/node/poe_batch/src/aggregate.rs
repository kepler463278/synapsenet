//! PoE batch aggregation

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

pub type NodeId = String;
pub type PubKey = String;
pub type Signature = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoeBatch {
    pub epoch: u64,
    pub chain_id: String,
    pub root: [u8; 32],
    pub items: Vec<PoeItem>,
    pub participants: Vec<NodeId>,
    pub signers: Vec<PubKey>,
    pub sigs: Vec<Signature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoeItem {
    pub node: NodeId,
    pub goal: Uuid,
    pub novelty: f32,
    pub coherence: f32,
    pub reuse: f32,
    pub weight: f32,
    pub leaf_hash: [u8; 32],
}

impl PoeItem {
    pub fn new(
        node: NodeId,
        goal: Uuid,
        novelty: f32,
        coherence: f32,
        reuse: f32,
        weight: f32,
    ) -> Self {
        let leaf_hash = Self::compute_leaf_hash(&node, &goal, novelty, coherence, reuse, weight);
        Self {
            node,
            goal,
            novelty,
            coherence,
            reuse,
            weight,
            leaf_hash,
        }
    }

    fn compute_leaf_hash(
        node: &str,
        goal: &Uuid,
        novelty: f32,
        coherence: f32,
        reuse: f32,
        weight: f32,
    ) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(node.as_bytes());
        hasher.update(goal.as_bytes());
        hasher.update(&novelty.to_le_bytes());
        hasher.update(&coherence.to_le_bytes());
        hasher.update(&reuse.to_le_bytes());
        hasher.update(&weight.to_le_bytes());
        hasher.finalize().into()
    }
}

pub struct BatchAggregator {
    chain_id: String,
}

impl BatchAggregator {
    pub fn new(chain_id: String) -> Self {
        Self { chain_id }
    }

    pub fn create_batch(&self, epoch: u64, items: Vec<PoeItem>) -> PoeBatch {
        let root = self.compute_merkle_root(&items);
        let participants: Vec<NodeId> = items.iter().map(|i| i.node.clone()).collect();

        PoeBatch {
            epoch,
            chain_id: self.chain_id.clone(),
            root,
            items,
            participants,
            signers: Vec::new(),
            sigs: Vec::new(),
        }
    }

    fn compute_merkle_root(&self, items: &[PoeItem]) -> [u8; 32] {
        if items.is_empty() {
            return [0u8; 32];
        }

        let mut hashes: Vec<[u8; 32]> = items.iter().map(|item| item.leaf_hash).collect();

        while hashes.len() > 1 {
            let mut next_level = Vec::new();

            for chunk in hashes.chunks(2) {
                let mut hasher = Sha256::new();
                hasher.update(&chunk[0]);
                if chunk.len() > 1 {
                    hasher.update(&chunk[1]);
                } else {
                    hasher.update(&chunk[0]);
                }
                next_level.push(hasher.finalize().into());
            }

            hashes = next_level;
        }

        hashes[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poe_item_creation() {
        let item = PoeItem::new(
            "node1".to_string(),
            Uuid::new_v4(),
            0.8,
            0.9,
            0.7,
            0.85,
        );

        assert_eq!(item.novelty, 0.8);
        assert_eq!(item.coherence, 0.9);
        assert_ne!(item.leaf_hash, [0u8; 32]);
    }

    #[test]
    fn test_merkle_root() {
        let aggregator = BatchAggregator::new("test-chain".to_string());

        let items = vec![
            PoeItem::new("node1".to_string(), Uuid::new_v4(), 0.8, 0.9, 0.7, 0.85),
            PoeItem::new("node2".to_string(), Uuid::new_v4(), 0.7, 0.8, 0.6, 0.75),
        ];

        let batch = aggregator.create_batch(1, items);
        assert_ne!(batch.root, [0u8; 32]);
        assert_eq!(batch.participants.len(), 2);
    }
}
