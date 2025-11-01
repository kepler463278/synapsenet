//! Unit tests for MultiModelManager

use synapsenet_ai::{MultiModelManager, ModelInfo, ModelSize};
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_model_info_creation() {
    let info = ModelInfo {
        name: "test-model".to_string(),
        size: ModelSize::Small,
        dimensions: 384,
        file_size_mb: 22.0,
        ram_required_mb: 100,
        supports_gpu: true,
    };
    
    assert_eq!(info.name, "test-model");
    assert_eq!(info.dimensions, 384);
    assert!(info.supports_gpu);
}

#[test]
fn test_model_size_ordering() {
    assert!(ModelSize::Small < ModelSize::Medium);
    assert!(ModelSize::Medium < ModelSize::Large);
    assert!(ModelSize::Small < ModelSize::Large);
}

#[test]
fn test_multi_model_manager_creation() {
    let temp_dir = TempDir::new().unwrap();
    let models_dir = temp_dir.path().to_path_buf();
    
    let manager = MultiModelManager::new(models_dir.clone());
    assert!(manager.is_ok());
    
    let manager = manager.unwrap();
    assert_eq!(manager.list_models().len(), 0);
}

#[test]
fn test_model_registration() {
    let temp_dir = TempDir::new().unwrap();
    let models_dir = temp_dir.path().to_path_buf();
    
    let mut manager = MultiModelManager::new(models_dir).unwrap();
    
    // Register a model
    let result = manager.register_model("test-model", 384, 22.0);
    assert!(result.is_ok());
    
    // Check it's registered
    let models = manager.list_models();
    assert_eq!(models.len(), 1);
    assert!(models.contains_key("test-model"));
    
    let info = &models["test-model"];
    assert_eq!(info.dimensions, 384);
    assert_eq!(info.file_size_mb, 22.0);
}

#[test]
fn test_duplicate_model_registration() {
    let temp_dir = TempDir::new().unwrap();
    let models_dir = temp_dir.path().to_path_buf();
    
    let mut manager = MultiModelManager::new(models_dir).unwrap();
    
    // Register once
    manager.register_model("test-model", 384, 22.0).unwrap();
    
    // Try to register again - should update, not error
    let result = manager.register_model("test-model", 768, 45.0);
    assert!(result.is_ok());
    
    // Check updated info
    let models = manager.list_models();
    assert_eq!(models.len(), 1);
    assert_eq!(models["test-model"].dimensions, 768);
}

#[test]
fn test_default_model() {
    let temp_dir = TempDir::new().unwrap();
    let models_dir = temp_dir.path().to_path_buf();
    
    let mut manager = MultiModelManager::new(models_dir).unwrap();
    
    // No default initially
    assert_eq!(manager.get_default_model(), None);
    
    // Register and set default
    manager.register_model("test-model", 384, 22.0).unwrap();
    manager.set_default_model("test-model").unwrap();
    
    assert_eq!(manager.get_default_model(), Some("test-model"));
}

#[test]
fn test_set_nonexistent_default() {
    let temp_dir = TempDir::new().unwrap();
    let models_dir = temp_dir.path().to_path_buf();
    
    let mut manager = MultiModelManager::new(models_dir).unwrap();
    
    // Try to set non-existent model as default
    let result = manager.set_default_model("nonexistent");
    assert!(result.is_err());
}

#[test]
fn test_multiple_models() {
    let temp_dir = TempDir::new().unwrap();
    let models_dir = temp_dir.path().to_path_buf();
    
    let mut manager = MultiModelManager::new(models_dir).unwrap();
    
    // Register multiple models
    manager.register_model("small-model", 384, 22.0).unwrap();
    manager.register_model("medium-model", 768, 45.0).unwrap();
    manager.register_model("large-model", 1024, 90.0).unwrap();
    
    let models = manager.list_models();
    assert_eq!(models.len(), 3);
    assert!(models.contains_key("small-model"));
    assert!(models.contains_key("medium-model"));
    assert!(models.contains_key("large-model"));
}

#[test]
fn test_model_size_from_dimensions() {
    // Small models: < 512 dimensions
    assert_eq!(ModelSize::from_dimensions(384), ModelSize::Small);
    
    // Medium models: 512-1024 dimensions
    assert_eq!(ModelSize::from_dimensions(768), ModelSize::Medium);
    
    // Large models: > 1024 dimensions
    assert_eq!(ModelSize::from_dimensions(1536), ModelSize::Large);
}

#[test]
fn test_model_info_display() {
    let info = ModelInfo {
        name: "test-model".to_string(),
        size: ModelSize::Small,
        dimensions: 384,
        file_size_mb: 22.0,
        ram_required_mb: 100,
        supports_gpu: true,
    };
    
    let display = format!("{:?}", info);
    assert!(display.contains("test-model"));
    assert!(display.contains("384"));
}
