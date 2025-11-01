//! Submit PoE batch to chain

use crate::aggregate::PoeBatch;
use serde_json::json;

pub struct ChainSubmitter {
    rpc_url: String,
    contract_address: String,
}

impl ChainSubmitter {
    pub fn new(rpc_url: String, contract_address: String) -> Self {
        Self {
            rpc_url,
            contract_address,
        }
    }

    pub async fn submit_batch(&self, batch: &PoeBatch) -> Result<String, String> {
        // Convert root to base64
        let root_b64 = base64::encode(&batch.root);

        // Build execute message
        let execute_msg = json!({
            "submit_batch": {
                "epoch": batch.epoch,
                "root": root_b64,
                "participants": batch.participants,
                "signers": batch.signers,
                "sigs": batch.sigs,
            }
        });

        // TODO: Sign and broadcast transaction
        tracing::info!("Submitting batch for epoch {}", batch.epoch);
        tracing::debug!("Execute msg: {}", execute_msg);

        // Placeholder response
        Ok("tx_hash_placeholder".to_string())
    }

    pub async fn claim_reward(&self, node: &str) -> Result<String, String> {
        let execute_msg = json!({
            "claim_reward": {
                "node": node
            }
        });

        tracing::info!("Claiming reward for node: {}", node);
        tracing::debug!("Execute msg: {}", execute_msg);

        Ok("tx_hash_placeholder".to_string())
    }

    pub async fn query_accrual(&self, node: &str) -> Result<u128, String> {
        let query_msg = json!({
            "get_accrual": {
                "node": node
            }
        });

        tracing::debug!("Querying accrual for node: {}", node);

        // TODO: Query contract
        Ok(0)
    }
}

// Base64 encoding helper
mod base64 {
    pub fn encode(data: &[u8]) -> String {
        use std::fmt::Write;
        let mut result = String::new();
        for byte in data {
            write!(&mut result, "{:02x}", byte).unwrap();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aggregate::{BatchAggregator, PoeItem};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_submit_batch() {
        let submitter = ChainSubmitter::new(
            "http://localhost:26657".to_string(),
            "contract_addr".to_string(),
        );

        let aggregator = BatchAggregator::new("test-chain".to_string());
        let items = vec![PoeItem::new(
            "node1".to_string(),
            Uuid::new_v4(),
            0.8,
            0.9,
            0.7,
            0.85,
        )];

        let batch = aggregator.create_batch(1, items);
        let result = submitter.submit_batch(&batch).await;

        assert!(result.is_ok());
    }
}
