//! Recovery phrase system for backup and restoration

use anyhow::{anyhow, Result};
use ring::rand::{SecureRandom, SystemRandom};
use sha2::{Sha256, Digest};

/// BIP39 word list (first 100 words for demo - full list would be 2048 words)
const WORD_LIST: &[&str] = &[
    "abandon", "ability", "able", "about", "above", "absent", "absorb", "abstract",
    "absurd", "abuse", "access", "accident", "account", "accuse", "achieve", "acid",
    "acoustic", "acquire", "across", "act", "action", "actor", "actress", "actual",
    "adapt", "add", "addict", "address", "adjust", "admit", "adult", "advance",
    "advice", "aerobic", "affair", "afford", "afraid", "again", "age", "agent",
    "agree", "ahead", "aim", "air", "airport", "aisle", "alarm", "album",
    "alcohol", "alert", "alien", "all", "alley", "allow", "almost", "alone",
    "alpha", "already", "also", "alter", "always", "amateur", "amazing", "among",
    "amount", "amused", "analyst", "anchor", "ancient", "anger", "angle", "angry",
    "animal", "ankle", "announce", "annual", "another", "answer", "antenna", "antique",
    "anxiety", "any", "apart", "apology", "appear", "apple", "approve", "april",
    "arch", "arctic", "area", "arena", "argue", "arm", "armed", "armor",
    "army", "around", "arrange", "arrest", "arrive", "arrow", "art", "artefact",
];

/// 12-word recovery phrase
#[derive(Debug, Clone)]
pub struct RecoveryPhrase {
    words: [String; 12],
}

impl RecoveryPhrase {
    /// Generate a new random recovery phrase
    pub fn generate() -> Result<Self> {
        let rng = SystemRandom::new();
        let mut entropy = [0u8; 16]; // 128 bits for 12 words
        rng.fill(&mut entropy)
            .map_err(|_| anyhow!("Failed to generate entropy"))?;
        
        Self::from_entropy(&entropy)
    }
    
    /// Create recovery phrase from entropy
    pub fn from_entropy(entropy: &[u8]) -> Result<Self> {
        if entropy.len() != 16 {
            return Err(anyhow!("Entropy must be 16 bytes"));
        }
        
        // Calculate checksum
        let mut hasher = Sha256::new();
        hasher.update(entropy);
        let hash = hasher.finalize();
        let checksum = hash[0] >> 4; // First 4 bits
        
        // Combine entropy and checksum
        let mut bits = Vec::new();
        for byte in entropy {
            for i in (0..8).rev() {
                bits.push((byte >> i) & 1);
            }
        }
        for i in (0..4).rev() {
            bits.push((checksum >> i) & 1);
        }
        
        // Convert to words (11 bits per word)
        let mut words = Vec::new();
        for chunk in bits.chunks(11) {
            let mut index = 0u16;
            for (i, &bit) in chunk.iter().enumerate() {
                index |= (bit as u16) << (10 - i);
            }
            
            // Use modulo to wrap around our limited word list
            let word_index = (index as usize) % WORD_LIST.len();
            words.push(WORD_LIST[word_index].to_string());
        }
        
        if words.len() != 12 {
            return Err(anyhow!("Failed to generate 12 words"));
        }
        
        Ok(Self {
            words: words.try_into().unwrap(),
        })
    }
    
    /// Create recovery phrase from words
    pub fn from_words(words: &[String]) -> Result<Self> {
        if words.len() != 12 {
            return Err(anyhow!("Recovery phrase must be 12 words"));
        }
        
        // Validate all words are in word list
        for word in words {
            if !WORD_LIST.contains(&word.as_str()) {
                return Err(anyhow!("Invalid word in recovery phrase: {}", word));
            }
        }
        
        Ok(Self {
            words: words.to_vec().try_into().unwrap(),
        })
    }
    
    /// Get words as slice
    pub fn words(&self) -> &[String] {
        &self.words
    }
    
    /// Convert to string (space-separated)
    pub fn to_string(&self) -> String {
        self.words.join(" ")
    }
    
    /// Parse from string
    pub fn from_string(s: &str) -> Result<Self> {
        let words: Vec<String> = s.split_whitespace()
            .map(|w| w.to_lowercase())
            .collect();
        Self::from_words(&words)
    }
    
    /// Derive seed from recovery phrase
    pub fn to_seed(&self) -> [u8; 64] {
        use pbkdf2::pbkdf2_hmac;
        
        let password = self.to_string();
        let salt = b"synapsenet-mobile";
        let mut seed = [0u8; 64];
        
        pbkdf2_hmac::<Sha256>(
            password.as_bytes(),
            salt,
            2048, // iterations
            &mut seed
        );
        
        seed
    }
    
    /// Derive encryption key from recovery phrase
    pub fn derive_key(&self) -> [u8; 32] {
        let seed = self.to_seed();
        let mut key = [0u8; 32];
        key.copy_from_slice(&seed[0..32]);
        key
    }
    
    /// Encrypt with Kyber KEM (placeholder)
    pub fn encrypt_with_kyber(&self, _public_key: &[u8]) -> Result<Vec<u8>> {
        // TODO: Implement Kyber KEM encryption
        // For now, return the seed
        Ok(self.to_seed().to_vec())
    }
    
    /// Decrypt with Kyber KEM (placeholder)
    pub fn decrypt_with_kyber(_ciphertext: &[u8], _secret_key: &[u8]) -> Result<[u8; 32]> {
        // TODO: Implement Kyber KEM decryption
        Ok([0u8; 32])
    }
}

impl std::fmt::Display for RecoveryPhrase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_recovery_phrase_generation() {
        let phrase = RecoveryPhrase::generate();
        assert!(phrase.is_ok());
        
        let phrase = phrase.unwrap();
        assert_eq!(phrase.words().len(), 12);
        
        // All words should be in word list
        for word in phrase.words() {
            assert!(WORD_LIST.contains(&word.as_str()));
        }
    }
    
    #[test]
    fn test_recovery_phrase_from_words() {
        let words = vec![
            "abandon".to_string(),
            "ability".to_string(),
            "able".to_string(),
            "about".to_string(),
            "above".to_string(),
            "absent".to_string(),
            "absorb".to_string(),
            "abstract".to_string(),
            "absurd".to_string(),
            "abuse".to_string(),
            "access".to_string(),
            "accident".to_string(),
        ];
        
        let phrase = RecoveryPhrase::from_words(&words);
        assert!(phrase.is_ok());
        
        let phrase = phrase.unwrap();
        assert_eq!(phrase.words(), words.as_slice());
    }
    
    #[test]
    fn test_recovery_phrase_string_conversion() {
        let phrase = RecoveryPhrase::generate().unwrap();
        let string = phrase.to_string();
        
        let parsed = RecoveryPhrase::from_string(&string);
        assert!(parsed.is_ok());
        
        let parsed = parsed.unwrap();
        assert_eq!(parsed.to_string(), string);
    }
    
    #[test]
    fn test_seed_derivation() {
        let phrase = RecoveryPhrase::generate().unwrap();
        let seed1 = phrase.to_seed();
        let seed2 = phrase.to_seed();
        
        // Same phrase should produce same seed
        assert_eq!(seed1, seed2);
        assert_eq!(seed1.len(), 64);
    }
    
    #[test]
    fn test_key_derivation() {
        let phrase = RecoveryPhrase::generate().unwrap();
        let key1 = phrase.derive_key();
        let key2 = phrase.derive_key();
        
        // Same phrase should produce same key
        assert_eq!(key1, key2);
        assert_eq!(key1.len(), 32);
    }
    
    #[test]
    fn test_invalid_word_count() {
        let words = vec!["abandon".to_string()]; // Only 1 word
        let result = RecoveryPhrase::from_words(&words);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_invalid_word() {
        let mut words = vec![
            "abandon".to_string(),
            "ability".to_string(),
            "able".to_string(),
            "about".to_string(),
            "above".to_string(),
            "absent".to_string(),
            "absorb".to_string(),
            "abstract".to_string(),
            "absurd".to_string(),
            "abuse".to_string(),
            "access".to_string(),
        ];
        words.push("invalidword".to_string());
        
        let result = RecoveryPhrase::from_words(&words);
        assert!(result.is_err());
    }
}
