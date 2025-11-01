//! Unit tests for batch processing

use synapsenet_ai::{BatchProcessor, BatchConfig, DocumentParser, SupportedFormat};
use std::time::Duration;
use tempfile::TempDir;

#[test]
fn test_supported_format_detection() {
    assert_eq!(SupportedFormat::from_extension("txt"), Some(SupportedFormat::PlainText));
    assert_eq!(SupportedFormat::from_extension("md"), Some(SupportedFormat::Markdown));
    assert_eq!(SupportedFormat::from_extension("pdf"), Some(SupportedFormat::Pdf));
    assert_eq!(SupportedFormat::from_extension("json"), Some(SupportedFormat::Json));
    assert_eq!(SupportedFormat::from_extension("csv"), Some(SupportedFormat::Csv));
    assert_eq!(SupportedFormat::from_extension("unknown"), None);
}

#[test]
fn test_batch_config_creation() {
    let config = BatchConfig {
        batch_size: 10,
        parallel_workers: 4,
        use_gpu: true,
        model_name: "test-model".to_string(),
        timeout: Duration::from_secs(30),
    };
    
    assert_eq!(config.batch_size, 10);
    assert_eq!(config.parallel_workers, 4);
    assert!(config.use_gpu);
    assert_eq!(config.model_name, "test-model");
}

#[test]
fn test_batch_config_default() {
    let config = BatchConfig::default();
    
    assert_eq!(config.batch_size, 32);
    assert_eq!(config.parallel_workers, 4);
    assert!(!config.use_gpu);
    assert_eq!(config.timeout, Duration::from_secs(60));
}

#[test]
fn test_plain_text_parser() {
    let parser = synapsenet_ai::PlainTextParser;
    let content = "This is a test document.\nWith multiple lines.";
    
    let result = parser.parse(content.as_bytes());
    assert!(result.is_ok());
    
    let parsed = result.unwrap();
    assert_eq!(parsed, content);
}

#[test]
fn test_markdown_parser() {
    let parser = synapsenet_ai::MarkdownParser;
    let content = "# Heading\n\nThis is **bold** text.";
    
    let result = parser.parse(content.as_bytes());
    assert!(result.is_ok());
    
    let parsed = result.unwrap();
    assert!(parsed.contains("Heading"));
    assert!(parsed.contains("bold"));
}

#[test]
fn test_json_parser() {
    let parser = synapsenet_ai::JsonParser;
    let content = r#"{"key": "value", "number": 42}"#;
    
    let result = parser.parse(content.as_bytes());
    assert!(result.is_ok());
    
    let parsed = result.unwrap();
    assert!(parsed.contains("key"));
    assert!(parsed.contains("value"));
}

#[test]
fn test_csv_parser() {
    let parser = synapsenet_ai::CsvParser;
    let content = "name,age\nAlice,30\nBob,25";
    
    let result = parser.parse(content.as_bytes());
    assert!(result.is_ok());
    
    let parsed = result.unwrap();
    assert!(parsed.contains("Alice"));
    assert!(parsed.contains("Bob"));
}

#[test]
fn test_file_scanner() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();
    
    // Create test files
    std::fs::write(dir_path.join("test1.txt"), "content1").unwrap();
    std::fs::write(dir_path.join("test2.md"), "content2").unwrap();
    std::fs::write(dir_path.join("test3.pdf"), "content3").unwrap();
    std::fs::write(dir_path.join("ignored.xyz"), "ignored").unwrap();
    
    let files = synapsenet_ai::scan_directory(dir_path).unwrap();
    
    // Should find 3 supported files
    assert_eq!(files.len(), 3);
    
    // Check file extensions
    let extensions: Vec<_> = files.iter()
        .filter_map(|p| p.extension())
        .filter_map(|e| e.to_str())
        .collect();
    
    assert!(extensions.contains(&"txt"));
    assert!(extensions.contains(&"md"));
    assert!(extensions.contains(&"pdf"));
    assert!(!extensions.contains(&"xyz"));
}

#[test]
fn test_batch_progress_tracking() {
    let progress = synapsenet_ai::BatchProgress {
        total_files: 10,
        processed_files: 5,
        total_chunks: 100,
        processed_chunks: 50,
        elapsed_seconds: 30.0,
        estimated_remaining_seconds: 30.0,
    };
    
    assert_eq!(progress.total_files, 10);
    assert_eq!(progress.processed_files, 5);
    assert_eq!(progress.progress_percent(), 50.0);
}

#[test]
fn test_batch_result() {
    let result = synapsenet_ai::BatchResult {
        total_files: 10,
        successful_files: 8,
        failed_files: 2,
        total_grains: 100,
        total_time_seconds: 60.0,
        errors: vec![
            "Error 1".to_string(),
            "Error 2".to_string(),
        ],
    };
    
    assert_eq!(result.total_files, 10);
    assert_eq!(result.successful_files, 8);
    assert_eq!(result.failed_files, 2);
    assert_eq!(result.success_rate(), 0.8);
    assert_eq!(result.errors.len(), 2);
}

#[test]
fn test_chunk_text() {
    let text = "This is a long text that needs to be chunked. ".repeat(100);
    let chunks = synapsenet_ai::chunk_text(&text, 500);
    
    assert!(chunks.len() > 1);
    
    // Each chunk should be <= 500 chars (except possibly last one)
    for chunk in &chunks[..chunks.len()-1] {
        assert!(chunk.len() <= 500);
    }
    
    // All chunks combined should equal original text
    let combined: String = chunks.join("");
    assert_eq!(combined.len(), text.len());
}

#[test]
fn test_chunk_text_small() {
    let text = "Short text";
    let chunks = synapsenet_ai::chunk_text(&text, 500);
    
    // Should return single chunk
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0], text);
}

#[test]
fn test_estimate_processing_time() {
    let config = BatchConfig {
        batch_size: 10,
        parallel_workers: 4,
        use_gpu: false,
        model_name: "test-model".to_string(),
        timeout: Duration::from_secs(60),
    };
    
    let total_chunks = 100;
    let estimated = synapsenet_ai::estimate_processing_time(total_chunks, &config);
    
    // Should return a reasonable estimate
    assert!(estimated > 0.0);
    assert!(estimated < 1000.0); // Less than 1000 seconds
}
