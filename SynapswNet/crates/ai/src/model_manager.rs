use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

/// Model information
pub struct ModelInfo {
    pub name: &'static str,
    pub url: &'static str,
    pub sha256: &'static str,
    pub filename: &'static str,
}

/// all-MiniLM-L6-v2 ONNX model
pub const ALL_MINILM_L6_V2: ModelInfo = ModelInfo {
    name: "all-MiniLM-L6-v2",
    url:
        "https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main/onnx/model.onnx",
    sha256: "9a8be0bd3b7d8d6c5d5e5f5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e", // Placeholder
    filename: "all-minilm-l6-v2.onnx",
};

/// Manages ONNX model downloads and verification
pub struct ModelManager {
    models_dir: PathBuf,
}

impl ModelManager {
    /// Create new ModelManager
    pub fn new(models_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&models_dir).context("Failed to create models directory")?;

        Ok(Self { models_dir })
    }

    /// Ensure model is available, download if missing
    pub async fn ensure_model(&self, model: &ModelInfo) -> Result<PathBuf> {
        let model_path = self.model_path(model);

        if model_path.exists() {
            info!("Model {} found at {:?}", model.name, model_path);

            // Verify checksum
            if self.verify_checksum(&model_path, model.sha256)? {
                return Ok(model_path);
            } else {
                warn!("Model checksum mismatch, re-downloading...");
                fs::remove_file(&model_path)?;
            }
        }

        // Download model
        info!("Downloading model {} from {}", model.name, model.url);
        self.download_model(model).await?;

        // Verify downloaded model
        if !self.verify_checksum(&model_path, model.sha256)? {
            anyhow::bail!("Downloaded model checksum verification failed");
        }

        info!("Model {} downloaded successfully", model.name);
        Ok(model_path)
    }

    /// Download model from URL
    async fn download_model(&self, model: &ModelInfo) -> Result<()> {
        let model_path = self.model_path(model);

        let response = reqwest::get(model.url)
            .await
            .context("Failed to download model")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to download model: HTTP {}", response.status());
        }

        let bytes = response
            .bytes()
            .await
            .context("Failed to read model bytes")?;

        fs::write(&model_path, bytes).context("Failed to write model file")?;

        Ok(())
    }

    /// Verify model file checksum
    pub fn verify_checksum(&self, path: &Path, expected: &str) -> Result<bool> {
        if !path.exists() {
            return Ok(false);
        }

        let bytes = fs::read(path).context("Failed to read model file for checksum")?;

        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let hash = hasher.finalize();
        let hash_hex = format!("{:x}", hash);

        Ok(hash_hex == expected)
    }

    /// Get model path
    pub fn model_path(&self, model: &ModelInfo) -> PathBuf {
        self.models_dir.join(model.filename)
    }

    /// Get models directory
    pub fn models_dir(&self) -> &Path {
        &self.models_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_model_manager_creation() {
        let temp_dir = env::temp_dir().join("synapsenet_test_models");
        let manager = ModelManager::new(temp_dir.clone()).unwrap();

        assert!(temp_dir.exists());
        assert_eq!(manager.models_dir(), temp_dir.as_path());

        // Cleanup
        fs::remove_dir_all(temp_dir).ok();
    }

    #[test]
    fn test_model_path() {
        let temp_dir = env::temp_dir().join("synapsenet_test_models");
        let manager = ModelManager::new(temp_dir.clone()).unwrap();

        let path = manager.model_path(&ALL_MINILM_L6_V2);
        assert!(path.ends_with("all-minilm-l6-v2.onnx"));

        // Cleanup
        fs::remove_dir_all(temp_dir).ok();
    }

    #[test]
    fn test_verify_checksum() {
        let temp_dir = env::temp_dir().join("synapsenet_test_models_checksum");
        fs::create_dir_all(&temp_dir).unwrap();
        let manager = ModelManager::new(temp_dir.clone()).unwrap();

        // Create test file
        let test_file = temp_dir.join("test.txt");
        fs::write(&test_file, b"test content").unwrap();

        // Calculate actual checksum
        let bytes = fs::read(&test_file).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let hash = hasher.finalize();
        let expected = format!("{:x}", hash);

        // Verify
        assert!(manager.verify_checksum(&test_file, &expected).unwrap());
        assert!(!manager
            .verify_checksum(&test_file, "wrong_checksum")
            .unwrap());

        // Cleanup
        fs::remove_dir_all(temp_dir).ok();
    }
}
