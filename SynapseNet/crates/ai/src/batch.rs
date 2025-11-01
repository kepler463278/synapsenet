use anyhow::Result;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, info, warn};

use crate::embed::EmbeddingModel;
use crate::gpu_providers::GpuProvider;
use crate::multi_model::MultiModelManager;
use synapsenet_core::{Grain, GrainMeta, SigningKeyTrait, UnifiedSigningKey};
use synapsenet_storage::{HnswIndex, Store};

/// Supported document formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedFormat {
    PlainText,
    Markdown,
    Json,
    // Pdf and Csv are optional for MVP
}

impl SupportedFormat {
    /// Get file extensions for this format
    pub fn extensions(&self) -> &[&str] {
        match self {
            Self::PlainText => &["txt", "text"],
            Self::Markdown => &["md", "markdown"],
            Self::Json => &["json"],
        }
    }

    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "txt" | "text" => Some(Self::PlainText),
            "md" | "markdown" => Some(Self::Markdown),
            "json" => Some(Self::Json),
            _ => None,
        }
    }
}

/// Document parser trait
pub trait DocumentParser: Send + Sync {
    /// Parse document content into text chunks
    fn parse(&self, content: &[u8]) -> Result<Vec<String>>;

    /// Get supported file extensions
    fn supported_extensions(&self) -> Vec<&'static str>;
}

/// Plain text parser
pub struct PlainTextParser;

impl DocumentParser for PlainTextParser {
    fn parse(&self, content: &[u8]) -> Result<Vec<String>> {
        let text = String::from_utf8_lossy(content).to_string();
        
        // Split into chunks by paragraphs (double newline)
        let chunks: Vec<String> = text
            .split("\n\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        if chunks.is_empty() {
            // If no paragraphs, return whole text
            Ok(vec![text])
        } else {
            Ok(chunks)
        }
    }

    fn supported_extensions(&self) -> Vec<&'static str> {
        vec!["txt", "text"]
    }
}

/// Markdown parser
pub struct MarkdownParser;

impl DocumentParser for MarkdownParser {
    fn parse(&self, content: &[u8]) -> Result<Vec<String>> {
        let text = String::from_utf8_lossy(content).to_string();
        
        // Split by headers or double newlines
        let chunks: Vec<String> = text
            .split("\n\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        if chunks.is_empty() {
            Ok(vec![text])
        } else {
            Ok(chunks)
        }
    }

    fn supported_extensions(&self) -> Vec<&'static str> {
        vec!["md", "markdown"]
    }
}

/// JSON parser
pub struct JsonParser;

impl DocumentParser for JsonParser {
    fn parse(&self, content: &[u8]) -> Result<Vec<String>> {
        let text = String::from_utf8_lossy(content).to_string();
        
        // Try to parse as JSON and extract text fields
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            let mut chunks = Vec::new();
            extract_text_from_json(&json, &mut chunks);
            
            if chunks.is_empty() {
                Ok(vec![text])
            } else {
                Ok(chunks)
            }
        } else {
            // If not valid JSON, treat as plain text
            Ok(vec![text])
        }
    }

    fn supported_extensions(&self) -> Vec<&'static str> {
        vec!["json"]
    }
}

/// Extract text from JSON recursively
fn extract_text_from_json(value: &serde_json::Value, chunks: &mut Vec<String>) {
    match value {
        serde_json::Value::String(s) => {
            if !s.trim().is_empty() {
                chunks.push(s.clone());
            }
        }
        serde_json::Value::Array(arr) => {
            for item in arr {
                extract_text_from_json(item, chunks);
            }
        }
        serde_json::Value::Object(obj) => {
            for (_key, val) in obj {
                extract_text_from_json(val, chunks);
            }
        }
        _ => {}
    }
}

/// Batch processing configuration
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Number of items to process at once
    pub batch_size: usize,
    /// Number of parallel workers
    pub parallel_workers: usize,
    /// Use GPU acceleration
    pub use_gpu: bool,
    /// Model name to use
    pub model_name: String,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            batch_size: 10,
            parallel_workers: 4,
            use_gpu: false,
            model_name: "all-MiniLM-L6-v2".to_string(),
        }
    }
}

/// Batch processing progress
#[derive(Debug, Clone)]
pub struct BatchProgress {
    pub total_files: usize,
    pub processed_files: usize,
    pub total_chunks: usize,
    pub processed_chunks: usize,
    pub current_file: String,
    pub elapsed_seconds: u64,
    pub estimated_remaining_seconds: u64,
}

/// Batch processing result
#[derive(Debug, Clone)]
pub struct BatchResult {
    pub success_count: usize,
    pub error_count: usize,
    pub total_grains_created: usize,
    pub total_time_seconds: u64,
    pub errors: Vec<String>,
}

/// Batch processor for importing multiple documents
pub struct BatchProcessor {
    embedding_manager: Arc<RwLock<MultiModelManager>>,
    store: Arc<tokio::sync::Mutex<Store>>,
    index: Arc<RwLock<HnswIndex<'static>>>,
    signing_key: Arc<UnifiedSigningKey>,
    gpu_provider: GpuProvider,
}

impl BatchProcessor {
    /// Create new batch processor
    pub fn new(
        embedding_manager: Arc<RwLock<MultiModelManager>>,
        store: Arc<tokio::sync::Mutex<Store>>,
        index: Arc<RwLock<HnswIndex<'static>>>,
        signing_key: Arc<UnifiedSigningKey>,
        gpu_provider: GpuProvider,
    ) -> Self {
        Self {
            embedding_manager,
            store,
            index,
            signing_key,
            gpu_provider,
        }
    }

    /// Import files from a directory
    pub async fn import_directory(
        &self,
        path: &Path,
        config: BatchConfig,
        progress_tx: mpsc::Sender<BatchProgress>,
    ) -> Result<BatchResult> {
        info!("Starting batch import from directory: {:?}", path);

        // Scan directory for files
        let files = self.scan_directory(path)?;
        info!("Found {} files to process", files.len());

        self.import_files(files, config, progress_tx).await
    }

    /// Import specific files
    pub async fn import_files(
        &self,
        files: Vec<PathBuf>,
        config: BatchConfig,
        progress_tx: mpsc::Sender<BatchProgress>,
    ) -> Result<BatchResult> {
        let start_time = std::time::Instant::now();
        let mut success_count = 0;
        let mut error_count = 0;
        let mut total_grains = 0;
        let mut errors = Vec::new();

        let total_files = files.len();

        for (idx, file_path) in files.iter().enumerate() {
            let current_file = file_path.display().to_string();
            debug!("Processing file {}/{}: {}", idx + 1, total_files, current_file);

            // Send progress update
            let progress = BatchProgress {
                total_files,
                processed_files: idx,
                total_chunks: 0,
                processed_chunks: 0,
                current_file: current_file.clone(),
                elapsed_seconds: start_time.elapsed().as_secs(),
                estimated_remaining_seconds: 0,
            };
            let _ = progress_tx.send(progress).await;

            // Process file
            match self.process_file(file_path, &config).await {
                Ok(grain_count) => {
                    success_count += 1;
                    total_grains += grain_count;
                    info!("✓ Processed {}: {} grains", current_file, grain_count);
                }
                Err(e) => {
                    error_count += 1;
                    let error_msg = format!("Failed to process {}: {}", current_file, e);
                    warn!("{}", error_msg);
                    errors.push(error_msg);
                }
            }
        }

        let total_time = start_time.elapsed().as_secs();

        info!(
            "Batch import complete: {} success, {} errors, {} grains in {}s",
            success_count, error_count, total_grains, total_time
        );

        Ok(BatchResult {
            success_count,
            error_count,
            total_grains_created: total_grains,
            total_time_seconds: total_time,
            errors,
        })
    }

    /// Process a single file
    async fn process_file(&self, path: &Path, config: &BatchConfig) -> Result<usize> {
        // Read file content
        let content = tokio::fs::read(path).await?;

        // Detect format
        let format = path
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(SupportedFormat::from_extension)
            .ok_or_else(|| anyhow::anyhow!("Unsupported file format"))?;

        // Parse content
        let chunks = self.parse_content(&content, format)?;
        debug!("Parsed {} chunks from file", chunks.len());

        // Process chunks in batches
        let mut grain_count = 0;
        for chunk in chunks {
            if chunk.trim().is_empty() {
                continue;
            }

            // Create grain from chunk
            match self.create_grain_from_text(&chunk, config).await {
                Ok(_) => grain_count += 1,
                Err(e) => warn!("Failed to create grain from chunk: {}", e),
            }
        }

        Ok(grain_count)
    }

    /// Parse content based on format
    fn parse_content(&self, content: &[u8], format: SupportedFormat) -> Result<Vec<String>> {
        let parser: Box<dyn DocumentParser> = match format {
            SupportedFormat::PlainText => Box::new(PlainTextParser),
            SupportedFormat::Markdown => Box::new(MarkdownParser),
            SupportedFormat::Json => Box::new(JsonParser),
        };

        parser.parse(content)
    }

    /// Create grain from text
    async fn create_grain_from_text(&self, text: &str, config: &BatchConfig) -> Result<()> {
        // Generate embedding
        let embedding = self
            .embedding_manager
            .read()
            .await
            .embed_with_model(text, &config.model_name)
            .await?;

        // Create metadata
        let meta = GrainMeta {
            author_pk: self.signing_key.public_key(),
            crypto_backend: self.signing_key.backend(),
            ts_unix_ms: chrono::Utc::now().timestamp_millis(),
            tags: vec![],
            mime: "text/plain".to_string(),
            lang: "en".to_string(),
            title: Some(text.chars().take(50).collect()),
            summary: Some(text.chars().take(200).collect()),
            embedding_model: Some(config.model_name.clone()),
            embedding_dimensions: Some(embedding.len()),
        };

        // Create and sign grain
        let grain = Grain::new_with_unified_key(embedding, meta, &self.signing_key)?;

        // Store grain
        self.store.lock().await.insert_grain(&grain)?;

        // Add to index
        self.index.write().await.add(&grain)?;

        Ok(())
    }

    /// Scan directory recursively for supported files
    fn scan_directory(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        if path.is_file() {
            files.push(path.to_path_buf());
            return Ok(files);
        }

        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Recursive scan
                files.extend(self.scan_directory(&path)?);
            } else if path.is_file() {
                // Check if supported format
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if SupportedFormat::from_extension(ext).is_some() {
                        files.push(path);
                    }
                }
            }
        }

        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_format() {
        assert_eq!(
            SupportedFormat::from_extension("txt"),
            Some(SupportedFormat::PlainText)
        );
        assert_eq!(
            SupportedFormat::from_extension("md"),
            Some(SupportedFormat::Markdown)
        );
        assert_eq!(
            SupportedFormat::from_extension("json"),
            Some(SupportedFormat::Json)
        );
        assert_eq!(SupportedFormat::from_extension("pdf"), None);
    }

    #[test]
    fn test_plain_text_parser() {
        let parser = PlainTextParser;
        let content = b"First paragraph.\n\nSecond paragraph.\n\nThird paragraph.";
        let chunks = parser.parse(content).unwrap();
        assert_eq!(chunks.len(), 3);
    }

    #[test]
    fn test_json_parser() {
        let parser = JsonParser;
        let content = br#"{"text": "Hello", "nested": {"value": "World"}}"#;
        let chunks = parser.parse(content).unwrap();
        assert!(chunks.len() >= 2);
    }
}
