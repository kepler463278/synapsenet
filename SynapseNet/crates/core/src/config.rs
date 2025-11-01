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

    /// Advanced network configuration (NEW in v0.4)
    #[serde(default)]
    pub network: NetworkConfig,

    /// AI/Embedding configuration
    pub ai: AiConfig,

    /// Storage configuration
    pub storage: StorageConfig,

    /// Economy/PoE configuration (NEW in v0.4)
    #[serde(default)]
    pub economy: EconomyConfig,

    /// UI configuration (NEW in v0.4)
    #[serde(default)]
    pub ui: UiConfig,
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
    /// ONNX model name (primary/default model)
    pub model_name: String,

    /// Embedding dimension
    pub embedding_dim: usize,

    /// Enable model download
    pub auto_download: bool,

    /// GPU provider: cpu, coreml, directml, cuda
    pub provider: String,

    /// Additional models to load (NEW in v0.4)
    #[serde(default)]
    pub additional_models: Vec<ModelConfig>,

    /// Enable multi-model support (NEW in v0.4)
    #[serde(default)]
    pub multi_model_enabled: bool,
}

/// Configuration for a single model (NEW in v0.4)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Model name
    pub name: String,

    /// Model file path (relative to models directory)
    pub path: String,

    /// Model size category: small, medium, large
    pub size: String,

    /// Auto-load this model on startup
    #[serde(default)]
    pub auto_load: bool,
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

/// Advanced network configuration (NEW in v0.4)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Enable DHT for peer discovery
    #[serde(default = "default_true")]
    pub dht_enabled: bool,

    /// DHT k-value (bucket size)
    #[serde(default = "default_dht_k")]
    pub dht_k: usize,

    /// Enable Circuit Relay v2
    #[serde(default = "default_true")]
    pub relay_enabled: bool,

    /// Enable AutoNAT for NAT detection
    #[serde(default = "default_true")]
    pub autonat_enabled: bool,

    /// Maximum number of peers to connect to
    #[serde(default = "default_max_peers")]
    pub max_peers: usize,

    /// Enable peer clustering by topic
    #[serde(default = "default_true")]
    pub clustering_enabled: bool,

    /// Cluster similarity threshold (0.0-1.0)
    #[serde(default = "default_cluster_threshold")]
    pub cluster_threshold: f32,

    /// Bootstrap node addresses (DNS seeds)
    #[serde(default)]
    pub bootstrap_nodes: Vec<String>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            dht_enabled: true,
            dht_k: 20,
            relay_enabled: true,
            autonat_enabled: true,
            max_peers: 50,
            clustering_enabled: true,
            cluster_threshold: 0.7,
            bootstrap_nodes: vec![
                "bootstrap1.synapsenet.io:9000".to_string(),
                "bootstrap2.synapsenet.io:9000".to_string(),
            ],
        }
    }
}

/// Economy/PoE configuration (NEW in v0.4)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomyConfig {
    /// Enable PoE v2 reward system
    #[serde(default = "default_true")]
    pub poe_enabled: bool,

    /// Novelty weight (0.0-1.0)
    #[serde(default = "default_novelty_weight")]
    pub novelty_weight: f32,

    /// Coherence weight (0.0-1.0)
    #[serde(default = "default_coherence_weight")]
    pub coherence_weight: f32,

    /// Reuse weight (0.0-1.0)
    #[serde(default = "default_reuse_weight")]
    pub reuse_weight: f32,

    /// Minimum novelty threshold for rewards
    #[serde(default = "default_min_novelty")]
    pub min_novelty_threshold: f32,

    /// Enable access tracking for reuse calculation
    #[serde(default = "default_true")]
    pub track_access: bool,

    /// Access event retention period (days)
    #[serde(default = "default_access_retention")]
    pub access_retention_days: u32,
}

impl Default for EconomyConfig {
    fn default() -> Self {
        Self {
            poe_enabled: true,
            novelty_weight: 0.4,
            coherence_weight: 0.3,
            reuse_weight: 0.3,
            min_novelty_threshold: 0.1,
            track_access: true,
            access_retention_days: 90,
        }
    }
}

/// UI configuration (NEW in v0.4)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// UI theme: light, dark, auto
    #[serde(default = "default_theme")]
    pub theme: String,

    /// Default view on startup: search, add, graph, stats
    #[serde(default = "default_view")]
    pub default_view: String,

    /// Enable graph visualization
    #[serde(default = "default_true")]
    pub graph_enabled: bool,

    /// Maximum nodes to display in graph
    #[serde(default = "default_graph_max_nodes")]
    pub graph_max_nodes: usize,

    /// Search results per page
    #[serde(default = "default_results_per_page")]
    pub results_per_page: usize,

    /// Enable animations
    #[serde(default = "default_true")]
    pub animations_enabled: bool,

    /// Show PoE scores in UI
    #[serde(default = "default_true")]
    pub show_poe_scores: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "auto".to_string(),
            default_view: "search".to_string(),
            graph_enabled: true,
            graph_max_nodes: 100,
            results_per_page: 10,
            animations_enabled: true,
            show_poe_scores: true,
        }
    }
}

// Default value functions for serde
fn default_true() -> bool {
    true
}

fn default_dht_k() -> usize {
    20
}

fn default_max_peers() -> usize {
    50
}

fn default_cluster_threshold() -> f32 {
    0.7
}

fn default_novelty_weight() -> f32 {
    0.4
}

fn default_coherence_weight() -> f32 {
    0.3
}

fn default_reuse_weight() -> f32 {
    0.3
}

fn default_min_novelty() -> f32 {
    0.1
}

fn default_access_retention() -> u32 {
    90
}

fn default_theme() -> String {
    "auto".to_string()
}

fn default_view() -> String {
    "search".to_string()
}

fn default_graph_max_nodes() -> usize {
    100
}

fn default_results_per_page() -> usize {
    10
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
            network: NetworkConfig::default(),
            ai: AiConfig {
                model_name: "all-MiniLM-L6-v2".to_string(),
                embedding_dim: 384,
                auto_download: false,
                provider: "cpu".to_string(), // cpu, coreml, directml, cuda
                additional_models: vec![],
                multi_model_enabled: false,
            },
            storage: StorageConfig {
                db_file: "synapsenet.db".to_string(),
                hnsw_max_elements: 1_000_000,
                hnsw_m: 16,
                hnsw_ef_construction: 200,
            },
            economy: EconomyConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

impl Config {
    /// Load configuration from TOML file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_ref = path.as_ref();
        let content = std::fs::read_to_string(path_ref)
            .map_err(|e| anyhow::anyhow!("Failed to read config file {:?}: {}", path_ref, e))?;
        
        let config: Config = toml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse config file {:?}: {}", path_ref, e))?;
        
        config.validate()
            .map_err(|e| anyhow::anyhow!("Configuration validation failed: {}", e))?;
        
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

    /// Migrate v0.3 configuration to v0.4
    /// 
    /// This handles backward compatibility by:
    /// - Adding default values for new fields (network, economy, ui)
    /// - Preserving existing settings
    /// - Validating the migrated configuration
    pub fn migrate_from_v03<P: AsRef<Path>>(old_path: P, new_path: P) -> Result<Self> {
        // Load old config (will use #[serde(default)] for missing fields)
        let content = std::fs::read_to_string(&old_path)?;
        let mut config: Config = toml::from_str(&content)?;

        // Ensure new fields have proper defaults
        if config.network.bootstrap_nodes.is_empty() {
            config.network = NetworkConfig::default();
        }

        // Validate migrated config
        config.validate()?;

        // Save to new location
        config.save(&new_path)?;

        Ok(config)
    }

    /// Check if configuration needs migration
    pub fn needs_migration<P: AsRef<Path>>(path: P) -> Result<bool> {
        if !path.as_ref().exists() {
            return Ok(false);
        }

        let content = std::fs::read_to_string(&path)?;
        
        // Check if new v0.4 fields are present
        let has_network = content.contains("[network]");
        let has_economy = content.contains("[economy]");
        let has_ui = content.contains("[ui]");

        // If any of the new sections are missing, migration is needed
        Ok(!(has_network && has_economy && has_ui))
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

        // Validate network config
        if self.network.max_peers == 0 {
            anyhow::bail!("Network max_peers must be > 0");
        }

        if self.network.cluster_threshold < 0.0 || self.network.cluster_threshold > 1.0 {
            anyhow::bail!("Network cluster_threshold must be between 0.0 and 1.0");
        }

        // Validate economy config (PoE weights)
        let weight_sum = self.economy.novelty_weight 
            + self.economy.coherence_weight 
            + self.economy.reuse_weight;
        
        if (weight_sum - 1.0).abs() > 0.01 {
            anyhow::bail!(
                "Economy weights must sum to 1.0, got {} (novelty: {}, coherence: {}, reuse: {})",
                weight_sum,
                self.economy.novelty_weight,
                self.economy.coherence_weight,
                self.economy.reuse_weight
            );
        }

        if self.economy.min_novelty_threshold < 0.0 || self.economy.min_novelty_threshold > 1.0 {
            anyhow::bail!("Economy min_novelty_threshold must be between 0.0 and 1.0");
        }

        // Validate UI config
        if !["light", "dark", "auto"].contains(&self.ui.theme.as_str()) {
            anyhow::bail!("UI theme must be 'light', 'dark', or 'auto'");
        }

        if !["search", "add", "graph", "stats"].contains(&self.ui.default_view.as_str()) {
            anyhow::bail!("UI default_view must be 'search', 'add', 'graph', or 'stats'");
        }

        if self.ui.graph_max_nodes == 0 {
            anyhow::bail!("UI graph_max_nodes must be > 0");
        }

        if self.ui.results_per_page == 0 {
            anyhow::bail!("UI results_per_page must be > 0");
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
