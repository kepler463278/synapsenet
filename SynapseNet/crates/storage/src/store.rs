use anyhow::Result;
use rusqlite::{params, Connection};
use synapsenet_core::poe::Credit;
use synapsenet_core::{Grain, Link};

/// SQLite storage for grains, links, credits, and peers
pub struct Store {
    conn: Connection,
}

impl Store {
    /// Create or open database
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        let store = Self { conn };
        store.init_schema()?;
        
        // Run migrations
        crate::migrations::run_migrations(&store.conn)?;
        
        Ok(store)
    }

    /// Initialize database schema
    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS grains (
                id BLOB PRIMARY KEY,
                vec BLOB NOT NULL,
                meta BLOB NOT NULL,
                sig BLOB NOT NULL,
                created_at INTEGER NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS links (
                from_id BLOB NOT NULL,
                to_id BLOB NOT NULL,
                weight REAL NOT NULL,
                rationale TEXT,
                sig BLOB NOT NULL,
                created_at INTEGER NOT NULL,
                PRIMARY KEY (from_id, to_id)
            );
            
            CREATE TABLE IF NOT EXISTS credits (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                grain_id BLOB NOT NULL,
                node_pk BLOB NOT NULL,
                ngt REAL NOT NULL,
                reason TEXT NOT NULL,
                ts_unix_ms INTEGER NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS peers (
                peer_id TEXT PRIMARY KEY,
                public_key BLOB NOT NULL,
                last_seen INTEGER NOT NULL,
                reputation REAL DEFAULT 0.0
            );
            
            CREATE INDEX IF NOT EXISTS idx_grains_created ON grains(created_at);
            CREATE INDEX IF NOT EXISTS idx_links_from ON links(from_id);
            CREATE INDEX IF NOT EXISTS idx_credits_grain ON credits(grain_id);
            "#,
        )?;
        Ok(())
    }

    /// Insert grain
    pub fn insert_grain(&self, grain: &Grain) -> Result<()> {
        let vec_bytes = bincode::serialize(&grain.vec)?;
        let meta_bytes = bincode::serialize(&grain.meta)?;
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis() as i64;

        self.conn.execute(
            "INSERT OR REPLACE INTO grains (id, vec, meta, sig, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![&grain.id[..], vec_bytes, meta_bytes, &grain.sig, ts],
        )?;
        Ok(())
    }

    /// Get grain by ID
    pub fn get_grain(&self, id: &[u8; 32]) -> Result<Option<Grain>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, vec, meta, sig FROM grains WHERE id = ?1")?;

        let mut rows = stmt.query(params![&id[..]])?;

        if let Some(row) = rows.next()? {
            let id_bytes: Vec<u8> = row.get(0)?;
            let vec_bytes: Vec<u8> = row.get(1)?;
            let meta_bytes: Vec<u8> = row.get(2)?;
            let sig: Vec<u8> = row.get(3)?;

            let mut id_arr = [0u8; 32];
            id_arr.copy_from_slice(&id_bytes);

            let vec: Vec<f32> = bincode::deserialize(&vec_bytes)?;
            let meta = bincode::deserialize(&meta_bytes)?;

            Ok(Some(Grain {
                id: id_arr,
                vec,
                meta,
                sig,
            }))
        } else {
            Ok(None)
        }
    }

    /// Get all grains
    pub fn get_all_grains(&self) -> Result<Vec<Grain>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, vec, meta, sig FROM grains ORDER BY created_at DESC")?;

        let rows = stmt.query_map([], |row| {
            let id_bytes: Vec<u8> = row.get(0)?;
            let vec_bytes: Vec<u8> = row.get(1)?;
            let meta_bytes: Vec<u8> = row.get(2)?;
            let sig: Vec<u8> = row.get(3)?;

            Ok((id_bytes, vec_bytes, meta_bytes, sig))
        })?;

        let mut grains = Vec::new();
        for row in rows {
            let (id_bytes, vec_bytes, meta_bytes, sig) = row?;

            let mut id = [0u8; 32];
            id.copy_from_slice(&id_bytes);

            let vec: Vec<f32> = bincode::deserialize(&vec_bytes)?;
            let meta = bincode::deserialize(&meta_bytes)?;

            grains.push(Grain { id, vec, meta, sig });
        }

        Ok(grains)
    }

    /// Insert link
    pub fn insert_link(&self, link: &Link) -> Result<()> {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis() as i64;

        self.conn.execute(
            "INSERT OR REPLACE INTO links (from_id, to_id, weight, rationale, sig, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                &link.from[..],
                &link.to[..],
                link.weight,
                &link.rationale,
                &link.sig,
                ts
            ],
        )?;
        Ok(())
    }

    /// Insert credit
    pub fn insert_credit(&self, credit: &Credit) -> Result<()> {
        self.conn.execute(
            "INSERT INTO credits (grain_id, node_pk, ngt, reason, ts_unix_ms) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                &credit.grain_id[..],
                &credit.node_pk[..],
                credit.ngt,
                &credit.reason,
                credit.ts_unix_ms
            ],
        )?;
        Ok(())
    }

    /// Get total NGT for a node
    pub fn get_node_ngt(&self, node_pk: &[u8; 32]) -> Result<f64> {
        let ngt: f64 = self.conn.query_row(
            "SELECT COALESCE(SUM(ngt), 0.0) FROM credits WHERE node_pk = ?1",
            params![&node_pk[..]],
            |row| row.get(0),
        )?;
        Ok(ngt)
    }

    /// Count grains
    pub fn count_grains(&self) -> Result<usize> {
        let count: usize = self
            .conn
            .query_row("SELECT COUNT(*) FROM grains", [], |row| row.get(0))?;
        Ok(count)
    }

    // ===== Grain Access Tracking =====

    /// Record grain access event
    pub fn record_grain_access(
        &self,
        grain_id: &[u8; 32],
        peer_id: &str,
        access_type: &str,
    ) -> Result<()> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        self.conn.execute(
            "INSERT INTO grain_access (grain_id, peer_id, access_type, timestamp) VALUES (?1, ?2, ?3, ?4)",
            params![&grain_id[..], peer_id, access_type, timestamp],
        )?;
        Ok(())
    }

    /// Get access events for a grain
    pub fn get_grain_access_events(
        &self,
        grain_id: &[u8; 32],
        limit: usize,
    ) -> Result<Vec<(String, String, i64)>> {
        let mut stmt = self.conn.prepare(
            "SELECT peer_id, access_type, timestamp FROM grain_access 
             WHERE grain_id = ?1 ORDER BY timestamp DESC LIMIT ?2",
        )?;

        let rows = stmt.query_map(params![&grain_id[..], limit], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Get access count for a grain
    pub fn get_grain_access_count(&self, grain_id: &[u8; 32]) -> Result<usize> {
        let count: usize = self.conn.query_row(
            "SELECT COUNT(*) FROM grain_access WHERE grain_id = ?1",
            params![&grain_id[..]],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    /// Clean old access events (older than cutoff timestamp)
    pub fn cleanup_old_access_events(&self, cutoff_timestamp: i64) -> Result<usize> {
        let deleted = self.conn.execute(
            "DELETE FROM grain_access WHERE timestamp < ?1",
            params![cutoff_timestamp],
        )?;
        Ok(deleted)
    }

    // ===== Embedding Models Metadata =====

    /// Register embedding model
    pub fn register_embedding_model(
        &self,
        name: &str,
        dimensions: usize,
        file_size_mb: f64,
    ) -> Result<()> {
        let loaded_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        self.conn.execute(
            "INSERT OR REPLACE INTO embedding_models (name, dimensions, file_size_mb, loaded_at) 
             VALUES (?1, ?2, ?3, ?4)",
            params![name, dimensions as i64, file_size_mb, loaded_at],
        )?;
        Ok(())
    }

    /// Get embedding model info
    pub fn get_embedding_model(&self, name: &str) -> Result<Option<(usize, f64, i64)>> {
        let result = self.conn.query_row(
            "SELECT dimensions, file_size_mb, loaded_at FROM embedding_models WHERE name = ?1",
            params![name],
            |row| Ok((row.get::<_, i64>(0)? as usize, row.get(1)?, row.get(2)?)),
        );

        match result {
            Ok(data) => Ok(Some(data)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Get all registered models
    pub fn get_all_embedding_models(&self) -> Result<Vec<(String, usize, f64, i64)>> {
        let mut stmt = self.conn.prepare(
            "SELECT name, dimensions, file_size_mb, loaded_at FROM embedding_models 
             ORDER BY loaded_at DESC",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get::<_, i64>(1)? as usize,
                row.get(2)?,
                row.get(3)?,
            ))
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    // ===== Peer Clusters =====

    /// Insert or update peer cluster entry
    pub fn upsert_peer_cluster(
        &self,
        topic: &str,
        peer_id: &str,
        similarity: f32,
    ) -> Result<()> {
        let last_seen = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        self.conn.execute(
            "INSERT INTO peer_clusters (topic, peer_id, similarity, last_seen) 
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(topic, peer_id) DO UPDATE SET 
             similarity = excluded.similarity, last_seen = excluded.last_seen",
            params![topic, peer_id, similarity, last_seen],
        )?;
        Ok(())
    }

    /// Get peers in a cluster (topic)
    pub fn get_cluster_peers(&self, topic: &str, limit: usize) -> Result<Vec<(String, f32, i64)>> {
        let mut stmt = self.conn.prepare(
            "SELECT peer_id, similarity, last_seen FROM peer_clusters 
             WHERE topic = ?1 ORDER BY similarity DESC LIMIT ?2",
        )?;

        let rows = stmt.query_map(params![topic, limit], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Get all topics for a peer
    pub fn get_peer_topics(&self, peer_id: &str) -> Result<Vec<(String, f32)>> {
        let mut stmt = self.conn.prepare(
            "SELECT topic, similarity FROM peer_clusters 
             WHERE peer_id = ?1 ORDER BY similarity DESC",
        )?;

        let rows = stmt.query_map(params![peer_id], |row| Ok((row.get(0)?, row.get(1)?)))?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Clean stale cluster entries (older than cutoff timestamp)
    pub fn cleanup_stale_clusters(&self, cutoff_timestamp: i64) -> Result<usize> {
        let deleted = self.conn.execute(
            "DELETE FROM peer_clusters WHERE last_seen < ?1",
            params![cutoff_timestamp],
        )?;
        Ok(deleted)
    }

    /// Get cluster statistics
    pub fn get_cluster_stats(&self) -> Result<(usize, usize)> {
        let topic_count: usize = self.conn.query_row(
            "SELECT COUNT(DISTINCT topic) FROM peer_clusters",
            [],
            |row| row.get(0),
        )?;

        let peer_count: usize = self.conn.query_row(
            "SELECT COUNT(DISTINCT peer_id) FROM peer_clusters",
            [],
            |row| row.get(0),
        )?;

        Ok((topic_count, peer_count))
    }
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
    fn test_store_grain() {
        let store = Store::new(":memory:").unwrap();

        let signing_key = generate_signing_key();
        let author_pk = signing_key.verifying_key().to_bytes();

        let meta = GrainMeta {
            author_pk: author_pk.to_vec(),
            crypto_backend: synapsenet_core::CryptoBackend::Classical,
            ts_unix_ms: 1234567890,
            tags: vec!["test".to_string()],
            mime: "text/plain".to_string(),
            lang: "en".to_string(),
            title: Some("Test".to_string()),
            summary: None,
            embedding_model: Some("test-model".to_string()),
            embedding_dimensions: Some(384),
        };

        let vec = vec![0.1, 0.2, 0.3];
        let grain = Grain::new(vec, meta, &signing_key).unwrap();

        store.insert_grain(&grain).unwrap();

        let retrieved = store.get_grain(&grain.id).unwrap().unwrap();
        assert_eq!(retrieved.id, grain.id);
        assert_eq!(retrieved.vec, grain.vec);
    }
}
