#[cfg(feature = "classical-crypto")]
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

use crate::crypto::{CryptoBackend, SigningKeyTrait, UnifiedSigningKey, UnifiedVerifyingKey, VerifyingKeyTrait};

/// Grain - atomic unit of semantic knowledge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grain {
    /// blake3(vec || meta || author_pk)
    pub id: [u8; 32],
    /// Embedding vector (256..1024 dims)
    pub vec: Vec<f32>,
    /// Metadata
    pub meta: GrainMeta,
    /// Author signature
    pub sig: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrainMeta {
    /// Author public key (variable length: 32 bytes for ed25519, ~2592 bytes for Dilithium)
    pub author_pk: Vec<u8>,
    /// Crypto backend used
    pub crypto_backend: CryptoBackend,
    /// Unix timestamp (milliseconds)
    pub ts_unix_ms: i64,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// MIME type
    pub mime: String,
    /// Language code (ISO 639-1)
    pub lang: String,
    /// Optional title
    pub title: Option<String>,
    /// Optional summary
    pub summary: Option<String>,
    /// Embedding model used (e.g., "all-MiniLM-L6-v2") - NEW in v0.4
    #[serde(default)]
    pub embedding_model: Option<String>,
    /// Embedding dimensions - NEW in v0.4
    #[serde(default)]
    pub embedding_dimensions: Option<usize>,
}

impl Grain {
    /// Create new grain with signature (classical ed25519)
    #[cfg(feature = "classical-crypto")]
    pub fn new(
        vec: Vec<f32>,
        meta: GrainMeta,
        signing_key: &SigningKey,
    ) -> Result<Self, anyhow::Error> {
        // Compute ID: blake3(vec || meta || author_pk)
        let mut hasher = blake3::Hasher::new();

        // Hash vector
        for &v in &vec {
            hasher.update(&v.to_le_bytes());
        }

        // Hash metadata
        let meta_bytes = bincode::serialize(&meta)?;
        hasher.update(&meta_bytes);

        let id = *hasher.finalize().as_bytes();

        // Sign the ID
        let signature = signing_key.sign(&id);

        Ok(Grain {
            id,
            vec,
            meta,
            sig: signature.to_bytes().to_vec(),
        })
    }
    
    /// Create new grain with unified signing key (supports both classical and PQC)
    pub fn new_with_unified_key(
        vec: Vec<f32>,
        meta: GrainMeta,
        signing_key: &UnifiedSigningKey,
    ) -> Result<Self, anyhow::Error> {
        // Compute ID: blake3(vec || meta || author_pk)
        let mut hasher = blake3::Hasher::new();

        // Hash vector
        for &v in &vec {
            hasher.update(&v.to_le_bytes());
        }

        // Hash metadata
        let meta_bytes = bincode::serialize(&meta)?;
        hasher.update(&meta_bytes);

        let id = *hasher.finalize().as_bytes();

        // Sign the ID using unified key
        let signature = signing_key.sign(&id);

        Ok(Grain {
            id,
            vec,
            meta,
            sig: signature,
        })
    }

    /// Verify grain signature (classical ed25519)
    #[cfg(feature = "classical-crypto")]
    pub fn verify(&self) -> Result<bool, anyhow::Error> {
        let pk_bytes: [u8; 32] = self.meta.author_pk.as_slice().try_into()
            .map_err(|_| anyhow::anyhow!("Invalid public key length for ed25519"))?;
        let verifying_key = VerifyingKey::from_bytes(&pk_bytes)?;
        let signature = Signature::from_bytes(
            self.sig
                .as_slice()
                .try_into()
                .map_err(|_| anyhow::anyhow!("Invalid signature length"))?,
        );

        Ok(verifying_key.verify(&self.id, &signature).is_ok())
    }
    
    /// Verify grain signature with crypto backend detection
    pub fn verify_with_backend(&self, backend: CryptoBackend) -> Result<bool, anyhow::Error> {
        let verifying_key = UnifiedVerifyingKey::from_bytes(&self.meta.author_pk, backend)?;
        verifying_key.verify(&self.id, &self.sig)
    }

    /// Compute cosine similarity with another grain
    pub fn cosine_similarity(&self, other: &Grain) -> f32 {
        cosine_similarity(&self.vec, &other.vec)
    }
}

/// Compute cosine similarity between two vectors
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot / (norm_a * norm_b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;
    use rand::RngCore;

    #[test]
    #[cfg(feature = "classical-crypto")]
    fn test_grain_creation_and_verification() {
        let mut secret_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut secret_bytes);
        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let author_pk = signing_key.verifying_key().to_bytes();

        let vec = vec![0.1, 0.2, 0.3, 0.4];
        
        let meta = GrainMeta {
            author_pk: author_pk.to_vec(),
            crypto_backend: CryptoBackend::Classical,
            ts_unix_ms: 1234567890,
            tags: vec!["test".to_string()],
            mime: "text/plain".to_string(),
            lang: "en".to_string(),
            title: Some("Test Grain".to_string()),
            summary: None,
            embedding_model: Some("test-model".to_string()),
            embedding_dimensions: Some(vec.len()),
        };

        let grain = Grain::new(vec, meta, &signing_key).unwrap();

        assert!(grain.verify().unwrap());
    }
    
    #[test]
    fn test_grain_with_unified_key() {
        let signing_key = UnifiedSigningKey::generate(UnifiedSigningKey::default_backend());
        let author_pk = signing_key.public_key();

        let vec = vec![0.1, 0.2, 0.3, 0.4];
        
        let meta = GrainMeta {
            author_pk: author_pk.clone(),
            crypto_backend: signing_key.backend(),
            ts_unix_ms: 1234567890,
            tags: vec!["test".to_string()],
            mime: "text/plain".to_string(),
            lang: "en".to_string(),
            title: Some("Test Grain".to_string()),
            summary: None,
            embedding_model: Some("test-model".to_string()),
            embedding_dimensions: Some(vec.len()),
        };

        let grain = Grain::new_with_unified_key(vec, meta, &signing_key).unwrap();

        assert!(grain.verify_with_backend(signing_key.backend()).unwrap());
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 1e-6);

        let c = vec![0.0, 1.0, 0.0];
        assert!((cosine_similarity(&a, &c) - 0.0).abs() < 1e-6);
    }
}
