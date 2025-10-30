use anyhow::Result;
use hnsw_rs::prelude::*;
use synapsenet_core::Grain;

/// HNSW vector index for KNN search
pub struct HnswIndex<'a> {
    index: Hnsw<'a, f32, DistCosine>,
    id_map: Vec<[u8; 32]>,
}

impl<'a> HnswIndex<'a> {
    /// Create new HNSW index
    pub fn new(max_elements: usize, dim: usize) -> Self {
        let index = Hnsw::<f32, DistCosine>::new(
            16,           // max_nb_connection
            max_elements, // max_elements
            16,           // ef_construction
            200,          // ef_search
            DistCosine,
        );

        Self {
            index,
            id_map: Vec::new(),
        }
    }

    /// Add grain to index
    pub fn add(&mut self, grain: &Grain) -> Result<()> {
        let idx = self.id_map.len();
        self.index.insert((&grain.vec, idx));
        self.id_map.push(grain.id);
        Ok(())
    }

    /// Search for k nearest neighbors
    pub fn search(&self, query: &[f32], k: usize) -> Result<Vec<SearchResult>> {
        let neighbors = self.index.search(query, k, 200);

        let results = neighbors
            .into_iter()
            .map(|neighbor| SearchResult {
                grain_id: self.id_map[neighbor.d_id],
                distance: neighbor.distance,
                similarity: 1.0 - neighbor.distance, // cosine distance to similarity
            })
            .collect();

        Ok(results)
    }
    
    /// Rebuild index from grains
    pub fn rebuild(&mut self, grains: &[Grain]) -> Result<()> {
        // Clear existing index
        self.id_map.clear();
        
        // Recreate index with appropriate size
        self.index = Hnsw::<f32, DistCosine>::new(
            16,
            grains.len().max(1000),
            16,
            200,
            DistCosine,
        );
        
        // Add all grains
        for grain in grains {
            self.add(grain)?;
        }
        
        Ok(())
    }
    
    /// Get number of indexed grains
    pub fn len(&self) -> usize {
        self.id_map.len()
    }
    
    /// Check if index is empty
    pub fn is_empty(&self) -> bool {
        self.id_map.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub grain_id: [u8; 32],
    pub distance: f32,
    pub similarity: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::{rngs::OsRng, RngCore};
    use synapsenet_core::GrainMeta;

    fn generate_signing_key() -> SigningKey {
        let mut secret_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut secret_bytes);
        SigningKey::from_bytes(&secret_bytes)
    }

    #[test]
    fn test_hnsw_index() {
        let mut index = HnswIndex::new(100, 3);

        let signing_key = generate_signing_key();
        let author_pk = signing_key.verifying_key().to_bytes();

        // Add some grains
        for i in 0..5 {
            let meta = GrainMeta {
                author_pk,
                ts_unix_ms: 1234567890 + i,
                tags: vec![],
                mime: "text/plain".to_string(),
                lang: "en".to_string(),
                title: None,
                summary: None,
            };

            let vec = vec![i as f32 * 0.1, 0.5, 0.3];
            let grain = Grain::new(vec, meta, &signing_key).unwrap();
            index.add(&grain).unwrap();
        }

        // Search
        let query = vec![0.0, 0.5, 0.3];
        let results = index.search(&query, 3).unwrap();

        assert_eq!(results.len(), 3);
        assert!(results[0].similarity >= results[1].similarity);
    }
}
