use anyhow::Result;
use std::path::PathBuf;
use tracing::{debug, info, warn};

use crate::embed::EmbeddingModel;
use crate::model_manager::{ModelManager, ALL_MINILM_L6_V2};

/// ONNX-based embedding model
///
/// Note: This is a simplified implementation for v0.2
/// Full ONNX integration will be completed in a follow-up task
pub struct OnnxEmbedding {
    model_path: PathBuf,
    dim: usize,
    _ready: bool,
}

impl OnnxEmbedding {
    /// Create new ONNX embedding model
    ///
    /// This will download the model if not present locally
    ///
    /// Note: Currently uses hash-based embeddings as fallback
    /// Set auto_download=true in config to enable model download
    pub async fn new(data_dir: PathBuf) -> Result<Self> {
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
        
        if ready {
            info!("ONNX embedding service initialized (model ready)");
        } else {
            info!("ONNX embedding service initialized (hash-based fallback)");
        }

        Ok(Self {
            model_path,
            dim: 384, // all-MiniLM-L6-v2 dimension
            _ready: ready,
        })
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
