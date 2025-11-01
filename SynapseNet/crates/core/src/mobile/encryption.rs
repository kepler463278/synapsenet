//! AES-GCM encryption for mobile memory capsules

use anyhow::{anyhow, Result};
use ring::aead::{Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey, AES_256_GCM};
use ring::error::Unspecified;
use ring::rand::{SecureRandom, SystemRandom};

const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;

/// Encrypted blob with nonce
#[derive(Debug, Clone)]
pub struct EncryptedBlob {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; NONCE_LEN],
}

/// AES-256-GCM encryptor
pub struct Encryptor {
    key: [u8; KEY_LEN],
    rng: SystemRandom,
}

impl Encryptor {
    /// Create a new encryptor with the given key
    pub fn new(key: [u8; KEY_LEN]) -> Self {
        Self {
            key,
            rng: SystemRandom::new(),
        }
    }
    
    /// Generate a new random encryption key
    pub fn generate_key() -> Result<[u8; KEY_LEN]> {
        let rng = SystemRandom::new();
        let mut key = [0u8; KEY_LEN];
        rng.fill(&mut key).map_err(|_| anyhow!("Failed to generate key"))?;
        Ok(key)
    }
    
    /// Encrypt data
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<EncryptedBlob> {
        // Generate random nonce
        let mut nonce_bytes = [0u8; NONCE_LEN];
        self.rng.fill(&mut nonce_bytes)
            .map_err(|_| anyhow!("Failed to generate nonce"))?;
        
        // Create sealing key
        let unbound_key = UnboundKey::new(&AES_256_GCM, &self.key)
            .map_err(|_| anyhow!("Failed to create key"))?;
        let nonce_sequence = CounterNonceSequence::new(nonce_bytes);
        let mut sealing_key = SealingKey::new(unbound_key, nonce_sequence);
        
        // Encrypt
        let mut in_out = plaintext.to_vec();
        sealing_key.seal_in_place_append_tag(Aad::empty(), &mut in_out)
            .map_err(|_| anyhow!("Encryption failed"))?;
        
        Ok(EncryptedBlob {
            ciphertext: in_out,
            nonce: nonce_bytes,
        })
    }
    
    /// Decrypt data
    pub fn decrypt(&self, blob: &EncryptedBlob) -> Result<Vec<u8>> {
        // Create opening key
        let unbound_key = UnboundKey::new(&AES_256_GCM, &self.key)
            .map_err(|_| anyhow!("Failed to create key"))?;
        let nonce_sequence = CounterNonceSequence::new(blob.nonce);
        let mut opening_key = OpeningKey::new(unbound_key, nonce_sequence);
        
        // Decrypt
        let mut in_out = blob.ciphertext.clone();
        let plaintext = opening_key.open_in_place(Aad::empty(), &mut in_out)
            .map_err(|_| anyhow!("Decryption failed"))?;
        
        Ok(plaintext.to_vec())
    }
    
    /// Derive key from password using HKDF
    pub fn derive_key_from_password(password: &str, salt: &[u8]) -> Result<[u8; KEY_LEN]> {
        use ring::hkdf;
        use ring::hmac;
        
        let salt = hkdf::Salt::new(hkdf::HKDF_SHA256, salt);
        let prk = salt.extract(password.as_bytes());
        
        let info = b"synapsenet-mobile-capsule";
        let okm = prk.expand(&[info], &AES_256_GCM)
            .map_err(|_| anyhow!("Key derivation failed"))?;
        
        let mut key = [0u8; KEY_LEN];
        okm.fill(&mut key)
            .map_err(|_| anyhow!("Key derivation failed"))?;
        
        Ok(key)
    }
}

/// Nonce sequence for AES-GCM
struct CounterNonceSequence {
    nonce: [u8; NONCE_LEN],
    used: bool,
}

impl CounterNonceSequence {
    fn new(nonce: [u8; NONCE_LEN]) -> Self {
        Self { nonce, used: false }
    }
}

impl NonceSequence for CounterNonceSequence {
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        if self.used {
            return Err(Unspecified);
        }
        self.used = true;
        Nonce::try_assume_unique_for_key(&self.nonce)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_key_generation() {
        let key = Encryptor::generate_key();
        assert!(key.is_ok());
        let key = key.unwrap();
        assert_eq!(key.len(), KEY_LEN);
    }
    
    #[test]
    fn test_encryption_decryption() {
        let key = Encryptor::generate_key().unwrap();
        let encryptor = Encryptor::new(key);
        
        let plaintext = b"Hello, SynapseNet!";
        let encrypted = encryptor.encrypt(plaintext).unwrap();
        
        assert_ne!(encrypted.ciphertext, plaintext);
        assert_eq!(encrypted.nonce.len(), NONCE_LEN);
        
        let decrypted = encryptor.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }
    
    #[test]
    fn test_key_derivation() {
        let password = "my-secure-password";
        let salt = b"random-salt-12345";
        
        let key1 = Encryptor::derive_key_from_password(password, salt).unwrap();
        let key2 = Encryptor::derive_key_from_password(password, salt).unwrap();
        
        // Same password and salt should produce same key
        assert_eq!(key1, key2);
        
        // Different salt should produce different key
        let key3 = Encryptor::derive_key_from_password(password, b"different-salt").unwrap();
        assert_ne!(key1, key3);
    }
    
    #[test]
    fn test_wrong_key_decryption_fails() {
        let key1 = Encryptor::generate_key().unwrap();
        let key2 = Encryptor::generate_key().unwrap();
        
        let encryptor1 = Encryptor::new(key1);
        let encryptor2 = Encryptor::new(key2);
        
        let plaintext = b"Secret message";
        let encrypted = encryptor1.encrypt(plaintext).unwrap();
        
        // Decryption with wrong key should fail
        let result = encryptor2.decrypt(&encrypted);
        assert!(result.is_err());
    }
}
