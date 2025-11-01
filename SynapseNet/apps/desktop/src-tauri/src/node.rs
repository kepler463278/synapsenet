//! Node management and lifecycle

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info};

/// Node manager handles the lifecycle of the SynapseNet node
pub struct NodeManager {
    running: bool,
}

impl NodeManager {
    pub fn new() -> Self {
        Self { running: false }
    }

    pub async fn start(&mut self) -> Result<(), String> {
        if self.running {
            return Err("Node is already running".to_string());
        }

        info!("Starting SynapseNet node...");

        // TODO: Initialize actual node components
        // - Start P2P network
        // - Initialize storage
        // - Connect to peers
        // - Start PoE tracking

        self.running = true;
        info!("Node started successfully");

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), String> {
        if !self.running {
            return Err("Node is not running".to_string());
        }

        info!("Stopping SynapseNet node...");

        // TODO: Gracefully shutdown node components
        // - Disconnect from peers
        // - Flush storage
        // - Stop background tasks

        self.running = false;
        info!("Node stopped successfully");

        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl Default for NodeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global node manager instance
pub type SharedNodeManager = Arc<Mutex<NodeManager>>;

pub fn create_node_manager() -> SharedNodeManager {
    Arc::new(Mutex::new(NodeManager::new()))
}
