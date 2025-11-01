//! Encrypted memory capsule for mobile devices

use super::encryption::{Encryptor, EncryptedBlob};
use super::keystore::KeyStore;
use crate::grain::Grain;
use anyhow::{anyhow, Result};
use rusqlite::{Connection, params};
use std::path::Path;

const KEY_ID: &str = "synapsenet-capsule-key";

/// Encrypted memory capsule
pub struct MemoryCapsule {
    db: Connection,
    encryptor: Encryptor,
    keystore: Box<dyn KeyStore>,
}

impl MemoryCapsule {
    /// Create a new memory capsule
    pub fn new<P: AsRef<Path>>(db_path: P, mut keystore: Box<dyn KeyStore>) -> Result<Self> {
        // Try to retrieve existing key
        let key = match keystore.retrieve_key(KEY_ID) {
            Ok(key_bytes) => {
                if key_bytes.len() != 32 {
                    return Err(anyhow!("Invalid key length"));
                }
                let mut key = [0u8; 32];
                key.copy_from_slice(&key_bytes);
                key
            }
            Err(_) => {
                // Generate new key
                let key = Encryptor::generate_key()?;
                keystore.store_key(KEY_ID, &key)?;
                key
            }
        };
        
        let encryptor = Encryptor::new(key);
        let db = Connection::open(db_path)?;
        
        // Initialize database schema
        Self::init_schema(&db)?;
        
        Ok(Self {
            db,
            encryptor,
            keystore,
        })
    }
    
    /// Initialize database schema
    fn init_schema(db: &Connection) -> Result<()> {
        db.execute(
            "CREATE TABLE IF NOT EXISTS encrypted_grains (
                id BLOB PRIMARY KEY,
                ciphertext BLOB NOT NULL,
                nonce BLOB NOT NULL,
                created_at INTEGER NOT NULL
            )",
            [],
        )?;
        
        db.execute(
            "CREATE INDEX IF NOT EXISTS idx_created_at ON encrypted_grains(created_at)",
            [],
        )?;
        
        Ok(())
    }
    
    /// Insert a grain into the capsule
    pub fn insert_grain(&mut self, grain: &Grain) -> Result<()> {
        // Serialize grain
        let plaintext = bincode::serialize(grain)?;
        
        // Encrypt
        let encrypted = self.encryptor.encrypt(&plaintext)?;
        
        // Store in database
        self.db.execute(
            "INSERT OR REPLACE INTO encrypted_grains (id, ciphertext, nonce, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![
                &grain.id[..],
                &encrypted.ciphertext,
                &encrypted.nonce[..],
                chrono::Utc::now().timestamp_millis(),
            ],
        )?;
        
        Ok(())
    }
    
    /// Get a grain from the capsule
    pub fn get_grain(&self, id: &[u8; 32]) -> Result<Option<Grain>> {
        let mut stmt = self.db.prepare(
            "SELECT ciphertext, nonce FROM encrypted_grains WHERE id = ?1"
        )?;
        
        let result = stmt.query_row(params![&id[..]], |row| {
            let ciphertext: Vec<u8> = row.get(0)?;
            let nonce_vec: Vec<u8> = row.get(1)?;
            
            let mut nonce = [0u8; 12];
            nonce.copy_from_slice(&nonce_vec);
            
            Ok(EncryptedBlob { ciphertext, nonce })
        });
        
        match result {
            Ok(encrypted) => {
                // Decrypt
                let plaintext = self.encryptor.decrypt(&encrypted)?;
                
                // Deserialize
                let grain: Grain = bincode::deserialize(&plaintext)?;
                Ok(Some(grain))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
    
    /// Get all grains
    pub fn get_all_grains(&self) -> Result<Vec<Grain>> {
        let mut stmt = self.db.prepare(
            "SELECT ciphertext, nonce FROM encrypted_grains ORDER BY created_at DESC"
        )?;
        
        let rows = stmt.query_map([], |row| {
            let ciphertext: Vec<u8> = row.get(0)?;
            let nonce_vec: Vec<u8> = row.get(1)?;
            
            let mut nonce = [0u8; 12];
            nonce.copy_from_slice(&nonce_vec);
            
            Ok(EncryptedBlob { ciphertext, nonce })
        })?;
        
        let mut grains = Vec::new();
        for encrypted_result in rows {
            let encrypted = encrypted_result?;
            let plaintext = self.encryptor.decrypt(&encrypted)?;
            let grain: Grain = bincode::deserialize(&plaintext)?;
            grains.push(grain);
        }
        
        Ok(grains)
    }
    
    /// Count grains in capsule
    pub fn count_grains(&self) -> Result<usize> {
        let count: i64 = self.db.query_row(
            "SELECT COUNT(*) FROM encrypted_grains",
            [],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }
    
    /// Delete a grain
    pub fn delete_grain(&mut self, id: &[u8; 32]) -> Result<()> {
        self.db.execute(
            "DELETE FROM encrypted_grains WHERE id = ?1",
            params![&id[..]],
        )?;
        Ok(())
    }
    
    /// Search local grains (placeholder - needs HNSW index)
    pub fn search_local(&self, _query: &[f32], _k: usize) -> Result<Vec<Grain>> {
        // TODO: Implement local search with HNSW index
        // For now, return all grains
        self.get_all_grains()
    }
    
    /// Export encrypted capsule
    pub fn export_encrypted<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        // TODO: Implement export
        // 1. Create backup file
        // 2. Include encrypted grains
        // 3. Include metadata
        tracing::info!("Exporting capsule to: {:?}", path.as_ref());
        Ok(())
    }
    
    /// Import encrypted capsule
    pub fn import_encrypted<P: AsRef<Path>>(&mut self, path: P, _recovery_phrase: &str) -> Result<()> {
        // TODO: Implement import
        // 1. Read backup file
        // 2. Verify integrity
        // 3. Decrypt with recovery phrase
        // 4. Import grains
        tracing::info!("Importing capsule from: {:?}", path.as_ref());
        Ok(())
    }
    
    /// Securely delete all data
    pub fn secure_delete(&mut self) -> Result<()> {
        // Delete all grains
        self.db.execute("DELETE FROM encrypted_grains", [])?;
        
        // Delete encryption key
        self.keystore.delete_key(KEY_ID)?;
        
        tracing::info!("Capsule securely deleted");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::UnifiedSigningKey;
    use crate::grain::GrainMeta;
    use crate::CryptoBackend;
    use tempfile::TempDir;
    
    fn create_test_grain() -> Grain {
        let vec = vec![0.1f32; 384];
        let meta = GrainMeta {
            author_pk: [0u8; 32],
            crypto_backend: CryptoBackend::Classical,
            ts_unix_ms: chrono::Utc::now().timestamp_millis(),
            tags: vec!["test".to_string()],
            mime: "text/plain".to_string(),
            lang: "en".to_string(),
            title: Some("Test Grain".to_string()),
            summary: None,
            embedding_model: Some("test-model".to_string()),
            embedding_dimensions: Some(384),
        };
        
        let signing_key = UnifiedSigningKey::generate_classical();
        Grain::new(vec, meta, &signing_key).unwrap()
    }
    
    #[test]
    fn test_capsule_creation() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let keystore = crate::mobile::keystore::create_keystore();
        
        let capsule = MemoryCapsule::new(&db_path, keystore);
        assert!(capsule.is_ok());
    }
    
    #[test]
    fn test_grain_storage_retrieval() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let keystore = crate::mobile::keystore::create_keystore();
        
        let mut capsule = MemoryCapsule::new(&db_path, keystore).unwrap();
        
        let grain = create_test_grain();
        let grain_id = grain.id;
        
        // Insert grain
        capsule.insert_grain(&grain).unwrap();
        
        // Retrieve grain
        let retrieved = capsule.get_grain(&grain_id).unwrap();
        assert!(retrieved.is_some());
        
        let retrieved_grain = retrieved.unwrap();
        assert_eq!(retrieved_grain.id, grain_id);
    }
    
    #[test]
    fn test_grain_count() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let keystore = crate::mobile::keystore::create_keystore();
        
        let mut capsule = MemoryCapsule::new(&db_path, keystore).unwrap();
        
        assert_eq!(capsule.count_grains().unwrap(), 0);
        
        capsule.insert_grain(&create_test_grain()).unwrap();
        assert_eq!(capsule.count_grains().unwrap(), 1);
        
        capsule.insert_grain(&create_test_grain()).unwrap();
        assert_eq!(capsule.count_grains().unwrap(), 2);
    }
    
    #[test]
    fn test_grain_deletion() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let keystore = crate::mobile::keystore::create_keystore();
        
        let mut capsule = MemoryCapsule::new(&db_path, keystore).unwrap();
        
        let grain = create_test_grain();
        let grain_id = grain.id;
        
        capsule.insert_grain(&grain).unwrap();
        assert_eq!(capsule.count_grains().unwrap(), 1);
        
        capsule.delete_grain(&grain_id).unwrap();
        assert_eq!(capsule.count_grains().unwrap(), 0);
    }
}
