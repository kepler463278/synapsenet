use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::embed::EmbeddingModel;
use crate::gpu_providers::GpuProvider;
use crate::onnx_embed::OnnxEmbedding;

/// Model size categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelSize {
    /// Small models (33MB, 384-dim) - phones/low-end devices
    Small,
    /// Medium models (120MB, 768-dim) - laptops
    Medium,
    /// Large models (500MB+, 1024-dim) - servers/GPU
    Large,
}

impl ModelSize {
    pub fn from_dimensions(dim: usize) -> Self {
        match dim {
            0..=512 => Self::Small,
            513..=896 => Self::Medium,
            _ => Self::Large,
        }
    }
}

impl std::fmt::Display for ModelSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Small => write!(f, "Small"),
            Self::Medium => write!(f, "Medium"),
            Self::Large => write!(f, "Large"),
        }
    }
}

/// Information about an embedding model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model name (e.g., "all-MiniLM-L6-v2")
    pub name: String,
    /// Model size category
    pub size: ModelSize,
    /// Embedding dimensions
    pub dimensions: usize,
    /// Approximate file size in MB
    pub file_size_mb: usize,
    /// Minimum RAM required in MB
    pub min_ram_mb: usize,
    /// Whether this model supports GPU acceleration
    pub supports_gpu: bool,
    /// Model description
    pub description: String,
}

impl ModelInfo {
    /// Create ModelInfo for all-MiniLM-L6-v2 (small model)
    pub fn mini_lm() -> Self {
        Self {
            name: "all-MiniLM-L6-v2".to_string(),
            size: ModelSize::Small,
            dimensions: 384,
            file_size_mb: 33,
            min_ram_mb: 256,
            supports_gpu: true,
            description: "Fast and efficient model for general-purpose embeddings".to_string(),
        }
    }

    /// Create ModelInfo for BERT-base (medium model)
    pub fn bert_base() -> Self {
        Self {
            name: "bert-base-uncased".to_string(),
            size: ModelSize::Medium,
            dimensions: 768,
            file_size_mb: 120,
            min_ram_mb: 512,
            supports_gpu: true,
            description: "Standard BERT model with good accuracy".to_string(),
        }
    }

    /// Create ModelInfo for Nomic Embed (large model)
    pub fn nomic_embed() -> Self {
        Self {
            name: "nomic-embed-text-v1".to_string(),
            size: ModelSize::Large,
            dimensions: 768,
            file_size_mb: 550,
            min_ram_mb: 1024,
            supports_gpu: true,
            description: "High-quality embeddings with strong performance".to_string(),
        }
    }

    /// Get all available models
    pub fn all_models() -> Vec<Self> {
        vec![Self::mini_lm(), Self::bert_base(), Self::nomic_embed()]
    }
}

/// Multi-model manager that can load and use multiple embedding models
pub struct MultiModelManager {
    /// Loaded models (name -> model)
    models: Arc<RwLock<HashMap<String, Arc<OnnxEmbedding>>>>,
    /// Currently active model name
    active_model: Arc<RwLock<String>>,
    /// GPU provider to use
    gpu_provider: GpuProvider,
    /// Data directory for models
    data_dir: PathBuf,
}

impl MultiModelManager {
    /// Create new multi-model manager
    pub fn new(data_dir: PathBuf, gpu_provider: GpuProvider) -> Self {
        Self {
            models: Arc::new(RwLock::new(HashMap::new())),
            active_model: Arc::new(RwLock::new(String::new())),
            gpu_provider,
            data_dir,
        }
    }

    /// Load a model by ModelInfo
    pub async fn load_model(&self, model_info: &ModelInfo) -> Result<()> {
        info!("Loading model: {} ({})", model_info.name, model_info.size);

        // Check if already loaded
        {
            let models = self.models.read().await;
            if models.contains_key(&model_info.name) {
                info!("Model {} already loaded", model_info.name);
                return Ok(());
            }
        }

        // Load the model
        let embedding = OnnxEmbedding::new_with_provider(
            self.data_dir.clone(),
            self.gpu_provider,
        )
        .await?;

        // Store in models map
        {
            let mut models = self.models.write().await;
            models.insert(model_info.name.clone(), Arc::new(embedding));
        }

        // Set as active if it's the first model
        {
            let active = self.active_model.read().await;
            if active.is_empty() {
                let mut active = self.active_model.write().await;
                *active = model_info.name.clone();
                info!("Set {} as active model", model_info.name);
            }
        }

        info!("✓ Model {} loaded successfully", model_info.name);
        Ok(())
    }

    /// Embed text using a specific model
    pub async fn embed_with_model(&self, text: &str, model_name: &str) -> Result<Vec<f32>> {
        let models = self.models.read().await;
        let model = models
            .get(model_name)
            .ok_or_else(|| anyhow::anyhow!("Model {} not loaded", model_name))?;

        model.embed(text)
    }

    /// Embed text using the active model
    pub async fn embed_auto(&self, text: &str) -> Result<Vec<f32>> {
        let active_name = self.active_model.read().await.clone();

        if active_name.is_empty() {
            return Err(anyhow::anyhow!("No active model set"));
        }

        self.embed_with_model(text, &active_name).await
    }

    /// List all loaded models
    pub async fn list_loaded_models(&self) -> Vec<String> {
        let models = self.models.read().await;
        models.keys().cloned().collect()
    }

    /// Get the active model name
    pub async fn get_active_model(&self) -> String {
        self.active_model.read().await.clone()
    }

    /// Set the active model
    pub async fn set_active_model(&self, model_name: &str) -> Result<()> {
        let models = self.models.read().await;
        if !models.contains_key(model_name) {
            return Err(anyhow::anyhow!("Model {} not loaded", model_name));
        }

        let mut active = self.active_model.write().await;
        *active = model_name.to_string();
        info!("Active model set to: {}", model_name);
        Ok(())
    }

    /// Get best model for current hardware
    pub fn get_best_model_for_hardware(&self) -> ModelInfo {
        // Simple heuristic based on available memory
        // In production, this would check actual system resources
        let available_ram_mb = 2048; // Placeholder

        if available_ram_mb >= 1024 {
            ModelInfo::nomic_embed()
        } else if available_ram_mb >= 512 {
            ModelInfo::bert_base()
        } else {
            ModelInfo::mini_lm()
        }
    }

    /// Unload a model to free memory
    pub async fn unload_model(&self, model_name: &str) -> Result<()> {
        let mut models = self.models.write().await;

        if !models.contains_key(model_name) {
            return Err(anyhow::anyhow!("Model {} not loaded", model_name));
        }

        // Check if it's the active model
        let active = self.active_model.read().await;
        if *active == model_name {
            warn!("Unloading active model {}", model_name);
            drop(active);
            let mut active = self.active_model.write().await;
            *active = String::new();
        }

        models.remove(model_name);
        info!("Model {} unloaded", model_name);
        Ok(())
    }

    /// Get model count
    pub async fn model_count(&self) -> usize {
        self.models.read().await.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_info() {
        let mini = ModelInfo::mini_lm();
        assert_eq!(mini.size, ModelSize::Small);
        assert_eq!(mini.dimensions, 384);

        let bert = ModelInfo::bert_base();
        assert_eq!(bert.size, ModelSize::Medium);
        assert_eq!(bert.dimensions, 768);
    }

    #[test]
    fn test_model_size_from_dimensions() {
        assert_eq!(ModelSize::from_dimensions(384), ModelSize::Small);
        assert_eq!(ModelSize::from_dimensions(768), ModelSize::Medium);
        assert_eq!(ModelSize::from_dimensions(1024), ModelSize::Large);
    }

    #[tokio::test]
    async fn test_multi_model_manager() {
        let manager = MultiModelManager::new(
            PathBuf::from("/tmp"),
            GpuProvider::Cpu,
        );

        assert_eq!(manager.model_count().await, 0);
        assert!(manager.get_active_model().await.is_empty());
    }
}
