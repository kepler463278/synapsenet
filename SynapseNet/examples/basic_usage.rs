// Basic usage example for SynapseNet

use anyhow::Result;
use ed25519_dalek::SigningKey;
use rand::{rngs::OsRng, RngCore};
use synapsenet_core::{Grain, GrainMeta, ProofOfEmergence};
use synapsenet_storage::{HnswIndex, Store};

fn generate_signing_key() -> SigningKey {
    let mut secret_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut secret_bytes);
    SigningKey::from_bytes(&secret_bytes)
}

fn main() -> Result<()> {
    println!("=== SynapseNet Basic Usage Example ===\n");

    // 1. Generate keypair
    println!("1. Generating keypair...");
    let signing_key = generate_signing_key();
    let author_pk = signing_key.verifying_key().to_bytes();
    println!("   Public key: {:?}...\n", &author_pk[..8]);

    // 2. Create store
    println!("2. Creating in-memory store...");
    let store = Store::new(":memory:")?;
    println!("   Store created\n");

    // 3. Add grains
    println!("3. Adding knowledge grains...");
    let knowledge = vec![
        "Rust is a systems programming language focused on safety and performance",
        "Python is great for data science and machine learning",
        "JavaScript runs in browsers and on servers with Node.js",
        "Go is designed for building scalable network services",
        "TypeScript adds static typing to JavaScript",
    ];

    for (i, text) in knowledge.iter().enumerate() {
        let meta = GrainMeta {
            author_pk,
            ts_unix_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_millis() as i64,
            tags: vec!["programming".to_string()],
            mime: "text/plain".to_string(),
            lang: "en".to_string(),
            title: Some(text.chars().take(50).collect()),
            summary: None,
        };

        // Create dummy embedding (in production, use ONNX model)
        let vec = create_dummy_embedding(text, 384);

        let grain = Grain::new(vec, meta, &signing_key)?;
        store.insert_grain(&grain)?;

        println!("   Added grain {}: {}...", i + 1, &text[..40]);
    }
    println!();

    // 4. Build index
    println!("4. Building HNSW index...");
    let grains = store.get_all_grains()?;
    let mut index = HnswIndex::new(100, 384);

    for grain in &grains {
        index.add(grain)?;
    }
    println!("   Index built with {} grains\n", index.len());

    // 5. Query
    println!("5. Querying: 'What is Rust?'");
    let query_vec = create_dummy_embedding("What is Rust?", 384);
    let results = index.search(&query_vec, 3)?;

    println!("   Found {} results:\n", results.len());
    for (i, result) in results.iter().enumerate() {
        let grain = store.get_grain(&result.grain_id)?.unwrap();
        println!("   {}. Similarity: {:.3}", i + 1, result.similarity);
        if let Some(title) = &grain.meta.title {
            println!("      Title: {}", title);
        }
        println!();
    }

    // 6. Calculate PoE
    println!("6. Calculating Proof of Emergence...");
    let poe = ProofOfEmergence::default();

    // Simulate metrics
    let novelty = 0.8;
    let coherence = 0.6;
    let reuse_count = 5;

    let ngt = poe.calculate_ngt(novelty, coherence, reuse_count);
    println!("   Novelty: {:.2}", novelty);
    println!("   Coherence: {:.2}", coherence);
    println!("   Reuse count: {}", reuse_count);
    println!("   NGT reward: {:.4}\n", ngt);

    // 7. Verify signatures
    println!("7. Verifying grain signatures...");
    let mut verified = 0;
    for grain in &grains {
        if grain.verify()? {
            verified += 1;
        }
    }
    println!("   Verified {}/{} grains\n", verified, grains.len());

    println!("=== Example Complete ===");

    Ok(())
}

// Helper function to create dummy embedding
fn create_dummy_embedding(text: &str, dim: usize) -> Vec<f32> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    let hash = hasher.finish();

    let mut vec = Vec::with_capacity(dim);
    for i in 0..dim {
        let val = ((hash.wrapping_mul(i as u64 + 1)) % 1000) as f32 / 1000.0;
        vec.push(val);
    }

    // Normalize
    let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
    vec.iter_mut().for_each(|x| *x /= norm);

    vec
}
