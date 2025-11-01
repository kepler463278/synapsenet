//! Mobile model manager with auto-detection and battery awareness

use super::MobileAIProvider;
use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg(target_os = "ios")]
use super::coreml::CoreMLProvider;

#[cfg(target_os = "android")]
use super::nnapi::NNAPIProvider;

use super::cpu::CPUProvider;

/// Mobile model manager
pub struct MobileModelManager {
    provider: Arc<RwLock<Box<dyn MobileAIProvider>>>,
    models_dir: PathBuf,
    current_model: Option<String>,
    battery_aware: bool,
}

impl MobileModelManager {
    /// Create a new mobile model manager
    pub fn new(models_dir: PathBuf) -> Result<Self> {
        let provider = Self::create_best_provider();
        
        Ok(Self {
            provider: Arc::new(RwLock::new(provider)),
            models_dir,
            current_model: None,
            battery_aware: true,
        })
    }
    
    /// Create the best available provider for current platform
    fn create_best_provider() -> Box<dyn MobileAIProvider> {
        #[cfg(target_os = "ios")]
        {
            if CoreMLProvider::is_available() {
                tracing::info!("Using CoreML provider");
                return Box::new(CoreMLProvider::new());
            }
        }
        
        #[cfg(target_os = "android")]
        {
            if NNAPIProvider::is_available() {
                tracing::info!("Using NNAPI provider");
                return Box::new(NNAPIProvider::new());
            }
        }
        
        tracing::info!("Using CPU fallback provider");
        Box::new(CPUProvider::new())
    }
    
    /// Load a model
    pub async fn load_model(&mut self, model_name: &str) -> Result<()> {
        let model_path = self.models_dir.join(model_name);
        
        if !model_path.exists() {
            return Err(anyhow!("Model not found: {}", model_name));
        }
        
        let mut provider = self.provider.write().await;
        provider.load_model(model_path.to_str().unwrap()).await?;
        
        self.current_model = Some(model_name.to_string());
        tracing::info!("Loaded model: {}", model_name);
        
        Ok(())
    }
    
    /// Generate embedding for text
    pub async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let provider = self.provider.read().await;
        provider.embed(text).await
    }
    
    /// Generate embeddings for multiple texts
    pub async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let provider = self.provider.read().await;
        provider.embed_batch(texts).await
    }
    
    /// Unload current model
    pub async fn unload_model(&mut self) -> Result<()> {
        let mut provider = self.provider.write().await;
        provider.unload_model()?;
        self.current_model = None;
        Ok(())
    }
    
    /// Get current provider name
    pub async fn provider_name(&self) -> String {
        let provider = self.provider.read().await;
        provider.name().to_string()
    }
    
    /// Get current model name
    pub fn current_model(&self) -> Option<&str> {
        self.current_model.as_deref()
    }
    
    /// Switch to a different provider
    pub async fn switch_provider(&mut self, provider_type: ProviderType) -> Result<()> {
        let new_provider: Box<dyn MobileAIProvider> = match provider_type {
            #[cfg(target_os = "ios")]
            ProviderType::CoreML => {
                if !CoreMLProvider::is_available() {
                    return Err(anyhow!("CoreML not available"));
                }
                Box::new(CoreMLProvider::new())
            }
            
            #[cfg(target_os = "android")]
            ProviderType::NNAPI => {
                if !NNAPIProvider::is_available() {
                    return Err(anyhow!("NNAPI not available"));
                }
                Box::new(NNAPIProvider::new())
            }
            
            ProviderType::CPU => Box::new(CPUProvider::new()),
            
            #[cfg(not(target_os = "ios"))]
            ProviderType::CoreML => {
                return Err(anyhow!("CoreML only available on iOS"));
            }
            
            #[cfg(not(target_os = "android"))]
            ProviderType::NNAPI => {
                return Err(anyhow!("NNAPI only available on Android"));
            }
        };
        
        // Unload current model
        {
            let mut provider = self.provider.write().await;
            let _ = provider.unload_model();
        }
        
        // Switch provider
        *self.provider.write().await = new_provider;
        self.current_model = None;
        
        tracing::info!("Switched to {} provider", provider_type.name());
        Ok(())
    }
    
    /// Enable/disable battery-aware mode
    pub fn set_battery_aware(&mut self, enabled: bool) {
        self.battery_aware = enabled;
        tracing::info!("Battery-aware mode: {}", enabled);
    }
    
    /// Check if battery-aware mode is enabled
    pub fn is_battery_aware(&self) -> bool {
        self.battery_aware
    }
    
    /// Download a model (placeholder)
    pub async fn download_model(&self, model_name: &str, url: &str) -> Result<()> {
        // TODO: Implement model download
        tracing::info!("Downloading model {} from {}", model_name, url);
        Ok(())
    }
    
    /// List available models
    pub fn list_models(&self) -> Result<Vec<String>> {
        let mut models = Vec::new();
        
        if let Ok(entries) = std::fs::read_dir(&self.models_dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    models.push(name.to_string());
                }
            }
        }
        
        Ok(models)
    }
    
    /// Delete a model
    pub fn delete_model(&self, model_name: &str) -> Result<()> {
        let model_path = self.models_dir.join(model_name);
        
        if model_path.exists() {
            std::fs::remove_file(model_path)?;
            tracing::info!("Deleted model: {}", model_name);
        }
        
        Ok(())
    }
}

/// Provider type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderType {
    CoreML,
    NNAPI,
    CPU,
}

impl ProviderType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::CoreML => "CoreML",
            Self::NNAPI => "NNAPI",
            Self::CPU => "CPU",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let manager = MobileModelManager::new(temp_dir.path().to_path_buf());
        assert!(manager.is_ok());
    }
    
    #[tokio::test]
    async fn test_provider_name() {
        let temp_dir = TempDir::new().unwrap();
        let manager = MobileModelManager::new(temp_dir.path().to_path_buf()).unwrap();
        let name = manager.provider_name().await;
        assert!(!name.is_empty());
    }
    
    #[tokio::test]
    async fn test_battery_aware_mode() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = MobileModelManager::new(temp_dir.path().to_path_buf()).unwrap();
        
        assert!(manager.is_battery_aware());
        
        manager.set_battery_aware(false);
        assert!(!manager.is_battery_aware());
    }
}
