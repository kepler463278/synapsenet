//! Mobile app state management

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use synapsenet_core::mobile::{MemoryCapsule, HardwareCapabilities, BatteryMonitor};
use synapsenet_ai::mobile::MobileModelManager;
use synapsenet_p2p::mobile::MobileP2PManager;

/// Mobile application state
pub struct MobileAppState {
    /// Hardware capabilities
    pub capabilities: HardwareCapabilities,
    
    /// Battery monitor
    pub battery: Arc<RwLock<BatteryMonitor>>,
    
    /// Encrypted memory capsule
    pub capsule: Arc<RwLock<MemoryCapsule>>,
    
    /// AI model manager
    pub ai_manager: Arc<RwLock<MobileModelManager>>,
    
    /// P2P manager
    pub p2p_manager: Arc<RwLock<MobileP2PManager>>,
}

impl MobileAppState {
    /// Create new mobile app state
    pub fn new() -> Result<Self> {
        // Detect hardware capabilities
        let capabilities = HardwareCapabilities::detect();
        tracing::info!("Hardware: {:?}", capabilities);
        
        // Initialize battery monitor
        let battery = Arc::new(RwLock::new(BatteryMonitor::new()));
        
        // Initialize encrypted capsule
        let data_dir = Self::get_data_dir()?;
        let db_path = data_dir.join("capsule.db");
        let keystore = synapsenet_core::mobile::keystore::create_keystore();
        let capsule = MemoryCapsule::new(db_path, keystore)?;
        let capsule = Arc::new(RwLock::new(capsule));
        
        // Initialize AI manager
        let models_dir = data_dir.join("models");
        std::fs::create_dir_all(&models_dir)?;
        let ai_manager = MobileModelManager::new(models_dir)?;
        let ai_manager = Arc::new(RwLock::new(ai_manager));
        
        // Initialize P2P manager
        let p2p_config = synapsenet_p2p::mobile::MobileP2PConfig::default();
        let p2p_manager = MobileP2PManager::new(p2p_config)?;
        let p2p_manager = Arc::new(RwLock::new(p2p_manager));
        
        Ok(Self {
            capabilities,
            battery,
            capsule,
            ai_manager,
            p2p_manager,
        })
    }
    
    /// Get app data directory
    fn get_data_dir() -> Result<std::path::PathBuf> {
        let data_dir = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?
            .join("synapsenet-mobile");
        
        std::fs::create_dir_all(&data_dir)?;
        Ok(data_dir)
    }
}
