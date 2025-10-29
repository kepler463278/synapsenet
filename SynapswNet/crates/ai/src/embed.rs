use anyhow::Result;

/// Embedding model interface
pub trait EmbeddingModel {
    /// Generate embedding for text
    fn embed(&self, text: &str) -> Result<Vec<f32>>;

    /// Get embedding dimension
    fn dim(&self) -> usize;
}

/// Dummy embedding model (replace with ONNX)
pub struct DummyEmbedding {
    dim: usize,
}

impl DummyEmbedding {
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }
}

impl EmbeddingModel for DummyEmbedding {
    fn embed(&self, text: &str) -> Result<Vec<f32>> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let hash = hasher.finish();

        let mut vec = Vec::with_capacity(self.dim);
        for i in 0..self.dim {
            let val = ((hash.wrapping_mul(i as u64 + 1)) % 1000) as f32 / 1000.0;
            vec.push(val);
        }

        // Normalize
        let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        vec.iter_mut().for_each(|x| *x /= norm);

        Ok(vec)
    }

    fn dim(&self) -> usize {
        self.dim
    }
}

// TODO: Implement ONNX-based embedding model
// pub struct OnnxEmbedding { ... }
