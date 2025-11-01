// GPU Acceleration Demo for SynapseNet
// Demonstrates CoreML, DirectML, and CUDA providers

use anyhow::Result;
use std::time::Instant;
use synapsenet_ai::{EmbeddingModel, GpuProvider, OnnxEmbedding};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("🚀 SynapseNet GPU Acceleration Demo\n");
    println!("{}", "=".repeat(60));

    // Detect available provider
    let provider = GpuProvider::detect();
    println!("\n📊 Detected GPU Provider: {}", provider);
    println!("Expected speedup: {:.1}x", provider.speedup_factor());

    // Test data
    let test_texts = vec![
        "Rust is a systems programming language",
        "Machine learning with neural networks",
        "Quantum computing and cryptography",
        "Distributed systems and consensus",
        "Natural language processing with transformers",
    ];

    println!("\n🧪 Testing with {} texts", test_texts.len());
    println!("{}", "-".repeat(60));

    // Create embedding model
    let data_dir = std::path::PathBuf::from(".synapsenet");
    let embedding = OnnxEmbedding::new_with_provider(data_dir, provider).await?;

    println!("\n✓ Model initialized with provider: {}", embedding.provider());
    println!("  Embedding dimension: {}", embedding.dim());

    // Benchmark embeddings
    println!("\n⏱️  Benchmarking embeddings...");
    let start = Instant::now();

    for (i, text) in test_texts.iter().enumerate() {
        let text_start = Instant::now();
        let vec = embedding.embed(text)?;
        let duration = text_start.elapsed();

        println!(
            "  [{}] {} chars → {} dims in {:?}",
            i + 1,
            text.len(),
            vec.len(),
            duration
        );
    }

    let total_duration = start.elapsed();
    let avg_time = total_duration / test_texts.len() as u32;

    println!("\n📈 Results:");
    println!("  Total time:   {:?}", total_duration);
    println!("  Average time: {:?}", avg_time);
    println!("  Throughput:   {:.1} embeddings/sec", 1000.0 / avg_time.as_millis() as f64);

    // Compare with expected speedup
    let cpu_baseline = std::time::Duration::from_millis(50); // Typical CPU time
    let expected_time = cpu_baseline.as_millis() as f32 / provider.speedup_factor();
    println!("\n💡 Performance:");
    println!("  CPU baseline:    ~{:?}", cpu_baseline);
    println!("  Expected (GPU):  ~{}ms", expected_time as u64);
    println!("  Actual:          {:?}", avg_time);

    if avg_time.as_millis() < cpu_baseline.as_millis() {
        let actual_speedup = cpu_baseline.as_millis() as f32 / avg_time.as_millis() as f32;
        println!("  ✓ Speedup achieved: {:.1}x", actual_speedup);
    }

    // Provider-specific info
    println!("\n🔧 Provider Details:");
    match provider {
        GpuProvider::Cpu => {
            println!("  Using CPU execution");
            println!("  To enable GPU, build with:");
            println!("    --features coreml    (Mac)");
            println!("    --features directml  (Windows)");
            println!("    --features cuda      (NVIDIA)");
        }
        GpuProvider::CoreML => {
            println!("  Using CoreML (Metal backend)");
            println!("  Optimized for Apple Silicon");
            println!("  Leverages Neural Engine when available");
        }
        GpuProvider::DirectML => {
            println!("  Using DirectML");
            println!("  Works with any GPU on Windows");
            println!("  Supports Intel, AMD, and NVIDIA");
        }
        GpuProvider::Cuda => {
            println!("  Using CUDA");
            println!("  Optimized for NVIDIA GPUs");
            println!("  Requires CUDA runtime installed");
        }
    }

    println!("\n{}", "=".repeat(60));
    println!("✅ GPU demo complete!");

    Ok(())
}
