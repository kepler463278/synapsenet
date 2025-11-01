//! Mobile AI providers for on-device inference
//!
//! This module provides platform-specific AI inference:
//! - CoreML for iOS (Metal GPU acceleration)
//! - NNAPI for Android (NPU/GPU acceleration)
//! - CPU fallback using ONNX Runtime Mobile

use anyhow::Result;
use async_trait::async_trait;

#[cfg(target_os = "ios")]
pub mod coreml;

#[cfg(target_os = "android")]
pub mod nnapi;

pub mod cpu;
pub mod manager;

pub use manager::MobileModelManager;

/// Trait for mobile AI providers
#[async_trait]
pub trait MobileAIProvider: Send + Sync {
    /// Load a model
    async fn load_model(&mut self, model_path: &str) -> Result<()>;
    
    /// Generate embedding for text
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;
    
    /// Generate embeddings for multiple texts
    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>>;
    
    /// Unload the current model
    fn unload_model(&mut self) -> Result<()>;
    
    /// Get provider name
    fn name(&self) -> &'static str;
    
    /// Check if provider is available on current device
    fn is_available() -> bool where Self: Sized;
}
