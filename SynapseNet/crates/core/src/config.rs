use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// SynapseNet configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Node configuration
    pub node: NodeConfig,

    /// P2P network configuration
    pub p2p: P2pConfig,

    /// AI/Embedding configuration
    pub ai: AiConfig,

    /// Storage configuration
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Node name/identifier
    pub name: String,

    /// Data directory
    pub data_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2pConfig {
    /// Enable P2P networking
    pub enabled: bool,

    /// P2P listen port
    pub port: u16,

    /// Enable mDNS peer discovery
    pub mdns_enabled: bool,

    /// Bootstrap peer addresses
    pub bootstrap_peers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    /// ONNX model name
    pub model_name: String,

    /// Embedding dimension
    pub embedding_dim: usize,

    /// Enable model download
    pub auto_download: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Database file name
    pub db_file: String,

    /// HNSW index max elements
    pub hnsw_max_elements: usize,

    /// HNSW M parameter (connections per layer)
    pub hnsw_m: usize,

    /// HNSW ef_construction parameter
    pub hnsw_ef_construction: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            node: NodeConfig {
                name: "synapsenet-node".to_string(),
                data_dir: ".synapsenet".to_string(),
            },
            p2p: P2pConfig {
                enabled: false,
                port: 9000,
                mdns_enabled: true,
                bootstrap_peers: Vec::new(),
            },
            ai: AiConfig {
                model_name: "all-MiniLM-L6-v2".to_string(),
                embedding_dim: 384,
                auto_download: false,
            },
            storage: StorageConfig {
                db_file: "synapsenet.db".to_string(),
                hnsw_max_elements: 1_000_000,
                hnsw_m: 16,
                hnsw_ef_construction: 200,
            },
        }
    }
}

impl Config {
    /// Load configuration from TOML file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    /// Save configuration to TOML file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Load or create default configuration
    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Result<Self> {
        if path.as_ref().exists() {
            Self::load(path)
        } else {
            let config = Self::default();
            config.save(&path)?;
            Ok(config)
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Validate P2P port
        if self.p2p.port == 0 {
            anyhow::bail!("P2P port cannot be 0");
        }

        // Validate embedding dimension
        if self.ai.embedding_dim == 0 {
            anyhow::bail!("Embedding dimension must be > 0");
        }

        // Validate HNSW parameters
        if self.storage.hnsw_max_elements == 0 {
            anyhow::bail!("HNSW max_elements must be > 0");
        }

        if self.storage.hnsw_m == 0 {
            anyhow::bail!("HNSW M parameter must be > 0");
        }

        if self.storage.hnsw_ef_construction == 0 {
            anyhow::bail!("HNSW ef_construction must be > 0");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();

        // Test invalid port
        config.p2p.port = 0;
        assert!(config.validate().is_err());

        // Reset and test invalid embedding dim
        config = Config::default();
        config.ai.embedding_dim = 0;
        assert!(config.validate().is_err());
    }
}
