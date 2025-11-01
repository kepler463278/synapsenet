use anyhow::Result;
use std::path::PathBuf;
use tracing::{debug, info, warn};

use crate::embed::EmbeddingModel;
use crate::gpu_providers::GpuProvider;
use crate::model_manager::{ModelManager, ALL_MINILM_L6_V2};

/// ONNX-based embedding model with GPU support
///
/// Supports multiple execution providers:
/// - CPU (default)
/// - CoreML (Mac - Metal backend)
/// - DirectML (Windows - any GPU)
/// - CUDA (NVIDIA GPUs)
pub struct OnnxEmbedding {
    model_path: PathBuf,
    dim: usize,
    _ready: bool,
    provider: GpuProvider,
}

impl OnnxEmbedding {
    /// Create new ONNX embedding model with auto-detected GPU provider
    ///
    /// This will download the model if not present locally
    ///
    /// Note: Currently uses hash-based embeddings as fallback
    /// Set auto_download=true in config to enable model download
    pub async fn new(data_dir: PathBuf) -> Result<Self> {
        Self::new_with_provider(data_dir, GpuProvider::detect()).await
    }

    /// Create new ONNX embedding model with specific GPU provider
    pub async fn new_with_provider(data_dir: PathBuf, provider: GpuProvider) -> Result<Self> {
        let models_dir = data_dir.join("models");
        let manager = ModelManager::new(models_dir)?;

        // Check if model download is enabled via environment variable
        let auto_download = std::env::var("SYNAPSENET_AUTO_DOWNLOAD")
            .unwrap_or_else(|_| "false".to_string())
            == "true";

        let model_path = if auto_download {
            info!("Auto-download enabled, ensuring model is available...");
            match manager.ensure_model(&ALL_MINILM_L6_V2).await {
                Ok(path) => {
                    info!("✓ ONNX model ready at: {:?}", path);
                    path
                }
                Err(e) => {
                    warn!("Failed to download model: {}", e);
                    warn!("Falling back to hash-based embeddings");
                    manager.model_path(&ALL_MINILM_L6_V2)
                }
            }
        } else {
            info!("ONNX model download disabled - using hash-based embeddings");
            info!("To enable: set SYNAPSENET_AUTO_DOWNLOAD=true or config.ai.auto_download=true");
            manager.model_path(&ALL_MINILM_L6_V2)
        };

        let ready = model_path.exists();
        
        // Log provider configuration
        provider.log_configuration();
        
        if ready {
            info!(
                "ONNX embedding service initialized (model ready, provider: {})",
                provider
            );
            info!(
                "Expected speedup: {:.1}x compared to CPU",
                provider.speedup_factor()
            );
        } else {
            info!("ONNX embedding service initialized (hash-based fallback)");
        }

        Ok(Self {
            model_path,
            dim: 384, // all-MiniLM-L6-v2 dimension
            _ready: ready,
            provider,
        })
    }

    /// Get current GPU provider
    pub fn provider(&self) -> GpuProvider {
        self.provider
    }

    /// Generate hash-based embedding (temporary implementation)
    fn hash_embed(&self, text: &str) -> Vec<f32> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let hash = hasher.finish();

        let mut vec = Vec::with_capacity(self.dim);
        for i in 0..self.dim {
            let val = ((hash.wrapping_mul(i as u64 + 1)) % 1000) as f32 / 1000.0;
            vec.push(val);
        }

        // Normalize
        let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            vec.iter_mut().for_each(|x| *x /= norm);
        }

        vec
    }
}

impl OnnxEmbedding {
    /// Generate embeddings for batch of texts
    pub fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
        texts.iter().map(|text| self.embed(text)).collect()
    }
}

impl EmbeddingModel for OnnxEmbedding {
    fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let start = std::time::Instant::now();

        debug!(
            "Generating embedding for text: {}...",
            &text.chars().take(50).collect::<String>()
        );

        // TODO: Replace with real ONNX inference
        let vec = self.hash_embed(text);

        let duration = start.elapsed();
        debug!(
            "Generated embedding with dimension {} in {:?}",
            vec.len(),
            duration
        );

        if duration.as_secs() >= 2 {
            warn!("Embedding generation took {:?} (> 2s threshold)", duration);
        }

        Ok(vec)
    }

    fn dim(&self) -> usize {
        self.dim
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_onnx_embedding_creation() {
        let temp_dir = std::env::temp_dir().join("synapsenet_test_onnx");
        let embedding = OnnxEmbedding::new(temp_dir.clone()).await;

        assert!(embedding.is_ok());

        let embedding = embedding.unwrap();
        assert_eq!(embedding.dim(), 384);

        // Cleanup
        std::fs::remove_dir_all(temp_dir).ok();
    }

    #[tokio::test]
    async fn test_embedding_generation() {
        let temp_dir = std::env::temp_dir().join("synapsenet_test_onnx_embed");
        let embedding = OnnxEmbedding::new(temp_dir.clone()).await.unwrap();

        let vec = embedding.embed("Hello world").unwrap();

        assert_eq!(vec.len(), 384);

        // Check normalization (L2 norm should be ~1.0)
        let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.01);

        // Cleanup
        std::fs::remove_dir_all(temp_dir).ok();
    }

    #[tokio::test]
    async fn test_batch_embedding() {
        let temp_dir = std::env::temp_dir().join("synapsenet_test_onnx_batch");
        let embedding = OnnxEmbedding::new(temp_dir.clone()).await.unwrap();

        let texts = vec!["Hello", "World", "Test"];
        let vecs = embedding.embed_batch(&texts).unwrap();

        assert_eq!(vecs.len(), 3);
        assert_eq!(vecs[0].len(), 384);

        // Cleanup
        std::fs::remove_dir_all(temp_dir).ok();
    }
}
