// Post-Quantum Cryptography Transport Layer for libp2p
// Uses Kyber KEM for key exchange instead of Noise

#[cfg(feature = "pqc-kyber")]
use anyhow::Result;
#[cfg(feature = "pqc-kyber")]
use pqcrypto_kyber::kyber1024;
#[cfg(feature = "pqc-kyber")]
use pqcrypto_traits::kem::{Ciphertext, PublicKey, SecretKey, SharedSecret};

#[cfg(feature = "pqc-kyber")]
pub struct KyberKem {
    public_key: kyber1024::PublicKey,
    secret_key: kyber1024::SecretKey,
}

#[cfg(feature = "pqc-kyber")]
impl KyberKem {
    /// Generate new Kyber keypair
    pub fn generate() -> Self {
        let (pk, sk) = kyber1024::keypair();
        Self {
            public_key: pk,
            secret_key: sk,
        }
    }
    
    /// Encapsulate: generate shared secret and ciphertext
    pub fn encapsulate(public_key: &kyber1024::PublicKey) -> (kyber1024::SharedSecret, kyber1024::Ciphertext) {
        kyber1024::encapsulate(public_key)
    }
    
    /// Decapsulate: recover shared secret from ciphertext
    pub fn decapsulate(&self, ciphertext: &kyber1024::Ciphertext) -> kyber1024::SharedSecret {
        kyber1024::decapsulate(ciphertext, &self.secret_key)
    }
    
    /// Get public key bytes
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.public_key.as_bytes().to_vec()
    }
    
    /// Get secret key bytes
    pub fn secret_key_bytes(&self) -> Vec<u8> {
        self.secret_key.as_bytes().to_vec()
    }
    
    /// Create from bytes
    pub fn from_bytes(pk_bytes: &[u8], sk_bytes: &[u8]) -> Result<Self> {
        let pk = kyber1024::PublicKey::from_bytes(pk_bytes)
            .map_err(|_| anyhow::anyhow!("Invalid public key"))?;
        let sk = kyber1024::SecretKey::from_bytes(sk_bytes)
            .map_err(|_| anyhow::anyhow!("Invalid secret key"))?;
        
        Ok(Self {
            public_key: pk,
            secret_key: sk,
        })
    }
}

/// Kyber-based handshake protocol
#[cfg(feature = "pqc-kyber")]
pub struct KyberHandshake {
    pub(crate) kem: KyberKem,
    shared_secret: Option<Vec<u8>>,
}

#[cfg(feature = "pqc-kyber")]
impl KyberHandshake {
    pub fn new() -> Self {
        Self {
            kem: KyberKem::generate(),
            shared_secret: None,
        }
    }
    
    /// Get public key bytes
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.kem.public_key_bytes()
    }
    
    /// Initiator: create handshake initiation message
    pub fn initiate(&mut self, responder_pk: &[u8]) -> Result<Vec<u8>> {
        let pk = kyber1024::PublicKey::from_bytes(responder_pk)
            .map_err(|_| anyhow::anyhow!("Invalid responder public key"))?;
        
        let (ss, ct) = KyberKem::encapsulate(&pk);
        self.shared_secret = Some(ss.as_bytes().to_vec());
        
        // Return ciphertext + our public key
        let mut message = ct.as_bytes().to_vec();
        message.extend_from_slice(&self.kem.public_key_bytes());
        
        Ok(message)
    }
    
    /// Responder: process handshake initiation and create response
    pub fn respond(&mut self, initiation: &[u8]) -> Result<Vec<u8>> {
        // Parse ciphertext and initiator's public key
        let ct_len = kyber1024::ciphertext_bytes();
        if initiation.len() < ct_len {
            return Err(anyhow::anyhow!("Invalid initiation message"));
        }
        
        let ct_bytes = &initiation[..ct_len];
        let initiator_pk_bytes = &initiation[ct_len..];
        
        let ct = kyber1024::Ciphertext::from_bytes(ct_bytes)
            .map_err(|_| anyhow::anyhow!("Invalid ciphertext"))?;
        
        // Decapsulate to get shared secret
        let ss = self.kem.decapsulate(&ct);
        self.shared_secret = Some(ss.as_bytes().to_vec());
        
        // Now encapsulate with initiator's public key
        let initiator_pk = kyber1024::PublicKey::from_bytes(initiator_pk_bytes)
            .map_err(|_| anyhow::anyhow!("Invalid initiator public key"))?;
        
        let (ss2, ct2) = KyberKem::encapsulate(&initiator_pk);
        
        // Combine both shared secrets
        let mut combined_secret = ss.as_bytes().to_vec();
        combined_secret.extend_from_slice(ss2.as_bytes());
        self.shared_secret = Some(blake3::hash(&combined_secret).as_bytes().to_vec());
        
        // Return ciphertext
        Ok(ct2.as_bytes().to_vec())
    }
    
    /// Initiator: finalize handshake with response
    pub fn finalize(&mut self, response: &[u8]) -> Result<()> {
        let ct = kyber1024::Ciphertext::from_bytes(response)
            .map_err(|_| anyhow::anyhow!("Invalid response ciphertext"))?;
        
        let ss2 = self.kem.decapsulate(&ct);
        
        // Combine both shared secrets
        if let Some(ss1) = &self.shared_secret {
            let mut combined_secret = ss1.clone();
            combined_secret.extend_from_slice(ss2.as_bytes());
            self.shared_secret = Some(blake3::hash(&combined_secret).as_bytes().to_vec());
        }
        
        Ok(())
    }
    
    /// Get the established shared secret
    pub fn shared_secret(&self) -> Option<&[u8]> {
        self.shared_secret.as_deref()
    }
    
    /// Derive encryption key from shared secret
    pub fn derive_key(&self) -> Option<[u8; 32]> {
        self.shared_secret.as_ref().map(|ss| {
            let hash = blake3::hash(ss);
            *hash.as_bytes()
        })
    }
}

#[cfg(feature = "pqc-kyber")]
impl Default for KyberHandshake {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(feature = "pqc-kyber")]
mod tests {
    use super::*;
    
    #[test]
    fn test_kyber_kem() {
        let kem = KyberKem::generate();
        let pk_bytes = kem.public_key_bytes();
        
        let pk = kyber1024::PublicKey::from_bytes(&pk_bytes).unwrap();
        let (ss1, ct) = KyberKem::encapsulate(&pk);
        let ss2 = kem.decapsulate(&ct);
        
        assert_eq!(ss1.as_bytes(), ss2.as_bytes());
    }
    
    #[test]
    fn test_kyber_handshake() {
        // Initiator and responder
        let mut initiator = KyberHandshake::new();
        let mut responder = KyberHandshake::new();
        
        // Get responder's public key
        let responder_pk = responder.kem.public_key_bytes();
        
        // Initiator starts handshake
        let initiation = initiator.initiate(&responder_pk).unwrap();
        
        // Responder processes and responds
        let response = responder.respond(&initiation).unwrap();
        
        // Initiator finalizes
        initiator.finalize(&response).unwrap();
        
        // Both should have the same shared secret
        let initiator_secret = initiator.shared_secret().unwrap();
        let responder_secret = responder.shared_secret().unwrap();
        
        assert_eq!(initiator_secret, responder_secret);
        assert_eq!(initiator_secret.len(), 32); // blake3 hash output
    }
}
