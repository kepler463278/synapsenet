//! Performance benchmarks for SynapseNet v0.4

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use synapsenet_core::{Grain, GrainMeta, UnifiedSigningKey, CryptoBackend};
use synapsenet_storage::{Store, HnswIndex};
use synapsenet_ai::{BatchConfig, chunk_text};
use tempfile::TempDir;
use std::time::Duration;

fn create_test_grain(text: &str, embedding_dim: usize) -> Grain {
    let vec = vec![0.1f32; embedding_dim];
    let meta = GrainMeta {
        author_pk: [0u8; 32],
        crypto_backend: CryptoBackend::Classical,
        ts_unix_ms: chrono::Utc::now().timestamp_millis(),
        tags: vec!["benchmark".to_string()],
        mime: "text/plain".to_string(),
        lang: "en".to_string(),
        title: Some(text.to_string()),
        summary: None,
        embedding_model: Some("benchmark-model".to_string()),
        embedding_dimensions: Some(embedding_dim),
    };
    
    let signing_key = UnifiedSigningKey::generate_classical();
    Grain::new(vec, meta, &signing_key).unwrap()
}

/// Benchmark grain creation with different embedding dimensions
fn bench_grain_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("grain_creation");
    
    for dim in [384, 768, 1536].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(dim), dim, |b, &dim| {
            b.iter(|| {
                create_test_grain(black_box("Test grain for benchmarking"), dim)
            });
        });
    }
    
    group.finish();
}

/// Benchmark grain storage operations
fn bench_storage_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("storage");
    
    // Benchmark insertion
    group.bench_function("insert_grain", |b| {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("bench.db");
        let mut store = Store::new(&db_path).unwrap();
        
        b.iter(|| {
            let grain = create_test_grain("Benchmark grain", 384);
            store.insert_grain(&grain).unwrap();
        });
    });
    
    // Benchmark retrieval
    group.bench_function("retrieve_grain", |b| {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("bench.db");
        let mut store = Store::new(&db_path).unwrap();
        
        // Pre-populate with grains
        let mut grain_ids = Vec::new();
        for i in 0..100 {
            let grain = create_test_grain(&format!("Grain {}", i), 384);
            grain_ids.push(grain.id);
            store.insert_grain(&grain).unwrap();
        }
        
        b.iter(|| {
            let id = &grain_ids[black_box(50)];
            store.get_grain(id).unwrap();
        });
    });
    
    // Benchmark bulk insertion
    group.bench_function("bulk_insert_100", |b| {
        b.iter(|| {
            let temp_dir = TempDir::new().unwrap();
            let db_path = temp_dir.path().join("bench.db");
            let mut store = Store::new(&db_path).unwrap();
            
            for i in 0..100 {
                let grain = create_test_grain(&format!("Bulk grain {}", i), 384);
                store.insert_grain(&grain).unwrap();
            }
        });
    });
    
    group.finish();
}

/// Benchmark HNSW index operations
fn bench_hnsw_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("hnsw_index");
    
    // Benchmark index creation
    group.bench_function("create_index", |b| {
        b.iter(|| {
            HnswIndex::new(black_box(10000), black_box(384))
        });
    });
    
    // Benchmark adding to index
    group.bench_function("add_to_index", |b| {
        let mut index = HnswIndex::new(10000, 384);
        
        b.iter(|| {
            let grain = create_test_grain("Index benchmark", 384);
            index.add(&grain).unwrap();
        });
    });
    
    // Benchmark search at different scales
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("search", size), size, |b, &size| {
            let mut index = HnswIndex::new(size, 384);
            
            // Pre-populate index
            for i in 0..size {
                let grain = create_test_grain(&format!("Grain {}", i), 384);
                index.add(&grain).unwrap();
            }
            
            let query = vec![0.5f32; 384];
            
            b.iter(|| {
                index.search(&query, black_box(10)).unwrap()
            });
        });
    }
    
    group.finish();
}

/// Benchmark text chunking
fn bench_text_chunking(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_chunking");
    
    let short_text = "Short text for chunking. ".repeat(10);
    let medium_text = "Medium length text for chunking. ".repeat(100);
    let long_text = "Long text for chunking. ".repeat(1000);
    
    group.bench_function("chunk_short", |b| {
        b.iter(|| {
            chunk_text(black_box(&short_text), 500)
        });
    });
    
    group.bench_function("chunk_medium", |b| {
        b.iter(|| {
            chunk_text(black_box(&medium_text), 500)
        });
    });
    
    group.bench_function("chunk_long", |b| {
        b.iter(|| {
            chunk_text(black_box(&long_text), 500)
        });
    });
    
    group.finish();
}

/// Benchmark similarity calculations
fn bench_similarity(c: &mut Criterion) {
    let mut group = c.benchmark_group("similarity");
    
    for dim in [384, 768, 1536].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(dim), dim, |b, &dim| {
            let vec1 = vec![0.5f32; dim];
            let vec2 = vec![0.6f32; dim];
            
            b.iter(|| {
                synapsenet_storage::cosine_similarity(
                    black_box(&vec1),
                    black_box(&vec2)
                )
            });
        });
    }
    
    group.finish();
}

/// Benchmark cryptographic operations
fn bench_crypto(c: &mut Criterion) {
    let mut group = c.benchmark_group("crypto");
    
    // Benchmark key generation
    group.bench_function("generate_classical_key", |b| {
        b.iter(|| {
            UnifiedSigningKey::generate_classical()
        });
    });
    
    group.bench_function("generate_pqc_key", |b| {
        b.iter(|| {
            UnifiedSigningKey::generate_pqc()
        });
    });
    
    // Benchmark signing
    group.bench_function("sign_classical", |b| {
        let key = UnifiedSigningKey::generate_classical();
        let data = b"Test data for signing";
        
        b.iter(|| {
            key.sign(black_box(data)).unwrap()
        });
    });
    
    group.bench_function("sign_pqc", |b| {
        let key = UnifiedSigningKey::generate_pqc();
        let data = b"Test data for signing";
        
        b.iter(|| {
            key.sign(black_box(data)).unwrap()
        });
    });
    
    // Benchmark verification
    group.bench_function("verify_classical", |b| {
        let key = UnifiedSigningKey::generate_classical();
        let data = b"Test data for signing";
        let signature = key.sign(data).unwrap();
        let public_key = key.public_key_bytes();
        
        b.iter(|| {
            UnifiedSigningKey::verify(
                black_box(&public_key),
                black_box(data),
                black_box(&signature),
                CryptoBackend::Classical
            ).unwrap()
        });
    });
    
    group.bench_function("verify_pqc", |b| {
        let key = UnifiedSigningKey::generate_pqc();
        let data = b"Test data for signing";
        let signature = key.sign(data).unwrap();
        let public_key = key.public_key_bytes();
        
        b.iter(|| {
            UnifiedSigningKey::verify(
                black_box(&public_key),
                black_box(data),
                black_box(&signature),
                CryptoBackend::PQC
            ).unwrap()
        });
    });
    
    group.finish();
}

/// Benchmark configuration operations
fn bench_config(c: &mut Criterion) {
    let mut group = c.benchmark_group("config");
    
    group.bench_function("create_default_config", |b| {
        b.iter(|| {
            synapsenet_core::Config::default()
        });
    });
    
    group.bench_function("validate_config", |b| {
        let config = synapsenet_core::Config::default();
        
        b.iter(|| {
            config.validate().unwrap()
        });
    });
    
    group.bench_function("save_and_load_config", |b| {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        let config = synapsenet_core::Config::default();
        
        b.iter(|| {
            config.save(&config_path).unwrap();
            synapsenet_core::Config::load(&config_path).unwrap();
        });
    });
    
    group.finish();
}

/// Benchmark memory usage patterns
fn bench_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory");
    
    // Benchmark grain cloning
    group.bench_function("clone_grain", |b| {
        let grain = create_test_grain("Clone benchmark", 384);
        
        b.iter(|| {
            black_box(grain.clone())
        });
    });
    
    // Benchmark vector operations
    group.bench_function("vector_allocation", |b| {
        b.iter(|| {
            black_box(vec![0.5f32; 384])
        });
    });
    
    group.bench_function("vector_clone", |b| {
        let vec = vec![0.5f32; 384];
        
        b.iter(|| {
            black_box(vec.clone())
        });
    });
    
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(100);
    targets = 
        bench_grain_creation,
        bench_storage_operations,
        bench_hnsw_index,
        bench_text_chunking,
        bench_similarity,
        bench_crypto,
        bench_config,
        bench_memory_patterns
}

criterion_main!(benches);
