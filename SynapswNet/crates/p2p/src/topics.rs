use serde::{Deserialize, Serialize};
use synapsenet_core::{Grain, Link};

/// P2P topic names
pub enum Topic {
    GrainsPut,
    GrainsAck,
    QueryKnn,
    QueryResp,
}

impl Topic {
    pub fn as_str(&self) -> &'static str {
        match self {
            Topic::GrainsPut => "grains.put",
            Topic::GrainsAck => "grains.ack",
            Topic::QueryKnn => "query.knn",
            Topic::QueryResp => "query.resp",
        }
    }
}

/// GossipSub message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GossipMessage {
    /// Publish new grain
    GrainPut { grain: Grain, links: Vec<Link> },

    /// Acknowledge grain receipt
    GrainAck { grain_id: [u8; 32], peer_id: String },

    /// KNN query request
    QueryKnn {
        query_id: String,
        vector: Vec<f32>,
        k: usize,
    },

    /// KNN query response
    QueryResp {
        query_id: String,
        results: Vec<QueryResult>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub grain_id: [u8; 32],
    pub similarity: f32,
    pub summary: Option<String>,
}
