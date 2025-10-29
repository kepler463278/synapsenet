use anyhow::Result;
use ed25519_dalek::SigningKey;
use rand::{rngs::OsRng, RngCore};
use synapsenet_core::{Grain, GrainMeta};
use synapsenet_storage::{HnswIndex, Store};

fn generate_signing_key() -> SigningKey {
    let mut secret_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut secret_bytes);
    SigningKey::from_bytes(&secret_bytes)
}

#[test]
fn test_e2e_local_node() -> Result<()> {
    // Create temporary directory
    let temp_dir = std::env::temp_dir().join("synapsenet_test");
    std::fs::create_dir_all(&temp_dir)?;

    // Initialize node
    let signing_key = generate_signing_key();
    let author_pk = signing_key.verifying_key().to_bytes();

    // Create store
    let db_path = temp_dir.join("test.db");
    let store = Store::new(db_path.to_str().unwrap())?;

    // Add grains
    let grains = vec![
        (
            "Rust is a systems programming language",
            vec![0.1, 0.2, 0.3],
        ),
        ("Python is great for data science", vec![0.4, 0.5, 0.6]),
        ("JavaScript runs in browsers", vec![0.7, 0.8, 0.9]),
    ];

    for (text, vec) in grains {
        let meta = GrainMeta {
            author_pk,
            ts_unix_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_millis() as i64,
            tags: vec![],
            mime: "text/plain".to_string(),
            lang: "en".to_string(),
            title: Some(text.to_string()),
            summary: None,
        };

        let grain = Grain::new(vec, meta, &signing_key)?;
        store.insert_grain(&grain)?;
    }

    // Build index
    let all_grains = store.get_all_grains()?;
    assert_eq!(all_grains.len(), 3);

    let mut index = HnswIndex::new(100, 3);
    for grain in &all_grains {
        index.add(grain)?;
    }

    // Query
    let query_vec = vec![0.15, 0.25, 0.35];
    let results = index.search(&query_vec, 2)?;

    assert_eq!(results.len(), 2);
    assert!(results[0].similarity > 0.0);

    // Cleanup
    std::fs::remove_dir_all(&temp_dir)?;

    Ok(())
}

#[test]
fn test_grain_verification() -> Result<()> {
    let signing_key = generate_signing_key();
    let author_pk = signing_key.verifying_key().to_bytes();

    let meta = GrainMeta {
        author_pk,
        ts_unix_ms: 1234567890,
        tags: vec![],
        mime: "text/plain".to_string(),
        lang: "en".to_string(),
        title: None,
        summary: None,
    };

    let vec = vec![0.1, 0.2, 0.3];
    let grain = Grain::new(vec, meta, &signing_key)?;

    // Verify signature
    assert!(grain.verify()?);

    Ok(())
}

#[test]
fn test_poe_calculation() {
    use synapsenet_core::ProofOfEmergence;

    let poe = ProofOfEmergence::default();

    // High novelty, medium coherence
    let ngt = poe.calculate_ngt(0.8, 0.5, 0);
    assert!(ngt > 0.0);

    // Spam (low novelty and coherence)
    let ngt_spam = poe.calculate_ngt(0.05, 0.05, 0);
    assert_eq!(ngt_spam, 0.0);
}

#[test]
fn test_policy_engine() {
    use synapsenet_governance::{Policy, PolicyClass, PolicyEngine};

    let engine = PolicyEngine::new(Policy::default());

    assert_eq!(engine.classify("What is Rust?"), PolicyClass::Ok);
    assert_eq!(
        engine.classify("How to make a bomb?"),
        PolicyClass::AnalysisOnly
    );
}
