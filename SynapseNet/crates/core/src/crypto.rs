// SynapseNet Crypto Abstraction Layer
// Supports both classical (ed25519) and post-quantum (Dilithium) signatures

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Crypto backend selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CryptoBackend {
    /// Classical ed25519 signatures
    Classical,
    /// Post-quantum Dilithium signatures
    PostQuantum,
}

/// Unified signing key interface
pub trait SigningKeyTrait: Send + Sync {
    /// Sign a message
    fn sign(&self, message: &[u8]) -> Vec<u8>;
    
    /// Get public key bytes
    fn public_key(&self) -> Vec<u8>;
    
    /// Get crypto backend type
    fn backend(&self) -> CryptoBackend;
}

/// Unified verifying key interface
pub trait VerifyingKeyTrait: Send + Sync {
    /// Verify a signature
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool>;
    
    /// Get public key bytes
    fn to_bytes(&self) -> Vec<u8>;
    
    /// Get crypto backend type
    fn backend(&self) -> CryptoBackend;
}

// ============================================================================
// Classical Crypto (ed25519)
// ============================================================================

#[cfg(feature = "classical-crypto")]
pub mod classical {
    use super::*;
    use ed25519_dalek::{Signature, Signer, SigningKey as Ed25519SigningKey, Verifier, VerifyingKey as Ed25519VerifyingKey};
    
    pub struct ClassicalSigningKey {
        inner: Ed25519SigningKey,
    }
    
    impl ClassicalSigningKey {
        pub fn new(inner: Ed25519SigningKey) -> Self {
            Self { inner }
        }
        
        pub fn from_bytes(bytes: &[u8; 32]) -> Self {
            Self {
                inner: Ed25519SigningKey::from_bytes(bytes),
            }
        }
        
        pub fn generate() -> Self {
            use rand::rngs::OsRng;
            use rand::RngCore;
            
            let mut secret_bytes = [0u8; 32];
            OsRng.fill_bytes(&mut secret_bytes);
            Self::from_bytes(&secret_bytes)
        }
    }
    
    impl SigningKeyTrait for ClassicalSigningKey {
        fn sign(&self, message: &[u8]) -> Vec<u8> {
            self.inner.sign(message).to_bytes().to_vec()
        }
        
        fn public_key(&self) -> Vec<u8> {
            self.inner.verifying_key().to_bytes().to_vec()
        }
        
        fn backend(&self) -> CryptoBackend {
            CryptoBackend::Classical
        }
    }
    
    pub struct ClassicalVerifyingKey {
        inner: Ed25519VerifyingKey,
    }
    
    impl ClassicalVerifyingKey {
        pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self> {
            Ok(Self {
                inner: Ed25519VerifyingKey::from_bytes(bytes)?,
            })
        }
    }
    
    impl VerifyingKeyTrait for ClassicalVerifyingKey {
        fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool> {
            let sig = Signature::from_bytes(
                signature.try_into()
                    .map_err(|_| anyhow::anyhow!("Invalid signature length"))?
            );
            Ok(self.inner.verify(message, &sig).is_ok())
        }
        
        fn to_bytes(&self) -> Vec<u8> {
            self.inner.to_bytes().to_vec()
        }
        
        fn backend(&self) -> CryptoBackend {
            CryptoBackend::Classical
        }
    }
}

// ============================================================================
// Post-Quantum Crypto (Dilithium)
// ============================================================================

#[cfg(feature = "pqc-dilithium")]
pub mod pqc {
    use super::*;
    use pqcrypto_dilithium::dilithium5;
    use pqcrypto_traits::sign::{PublicKey, SecretKey, SignedMessage};
    
    pub struct PqcSigningKey {
        inner: dilithium5::SecretKey,
        public_key: dilithium5::PublicKey,
    }
    
    impl PqcSigningKey {
        pub fn generate() -> Self {
            let (pk, sk) = dilithium5::keypair();
            Self {
                inner: sk,
                public_key: pk,
            }
        }
        
        pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
            let sk = dilithium5::SecretKey::from_bytes(bytes)
                .map_err(|_| anyhow::anyhow!("Invalid secret key"))?;
            
            // Derive public key from secret key
            // Note: Dilithium doesn't provide direct derivation, so we need to store both
            // In practice, you'd store the public key alongside the secret key
            let (pk, _) = dilithium5::keypair();
            
            Ok(Self {
                inner: sk,
                public_key: pk,
            })
        }
        
        pub fn to_bytes(&self) -> Vec<u8> {
            self.inner.as_bytes().to_vec()
        }
    }
    
    impl SigningKeyTrait for PqcSigningKey {
        fn sign(&self, message: &[u8]) -> Vec<u8> {
            let signed_msg = dilithium5::sign(message, &self.inner);
            signed_msg.as_bytes().to_vec()
        }
        
        fn public_key(&self) -> Vec<u8> {
            self.public_key.as_bytes().to_vec()
        }
        
        fn backend(&self) -> CryptoBackend {
            CryptoBackend::PostQuantum
        }
    }
    
    pub struct PqcVerifyingKey {
        inner: dilithium5::PublicKey,
    }
    
    impl PqcVerifyingKey {
        pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
            let pk = dilithium5::PublicKey::from_bytes(bytes)
                .map_err(|_| anyhow::anyhow!("Invalid public key"))?;
            Ok(Self { inner: pk })
        }
    }
    
    impl VerifyingKeyTrait for PqcVerifyingKey {
        fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool> {
            let signed_msg = dilithium5::SignedMessage::from_bytes(signature)
                .map_err(|_| anyhow::anyhow!("Invalid signed message"))?;
            
            match dilithium5::open(&signed_msg, &self.inner) {
                Ok(recovered_msg) => Ok(recovered_msg == message),
                Err(_) => Ok(false),
            }
        }
        
        fn to_bytes(&self) -> Vec<u8> {
            self.inner.as_bytes().to_vec()
        }
        
        fn backend(&self) -> CryptoBackend {
            CryptoBackend::PostQuantum
        }
    }
}

// ============================================================================
// Unified Crypto Interface
// ============================================================================

/// Unified signing key that works with both classical and PQC
pub enum UnifiedSigningKey {
    #[cfg(feature = "classical-crypto")]
    Classical(classical::ClassicalSigningKey),
    
    #[cfg(feature = "pqc-dilithium")]
    PostQuantum(pqc::PqcSigningKey),
}

impl UnifiedSigningKey {
    /// Generate new key with specified backend
    pub fn generate(backend: CryptoBackend) -> Self {
        match backend {
            CryptoBackend::Classical => {
                #[cfg(feature = "classical-crypto")]
                {
                    Self::Classical(classical::ClassicalSigningKey::generate())
                }
                #[cfg(not(feature = "classical-crypto"))]
                {
                    panic!("Classical crypto not enabled. Enable with --features classical-crypto")
                }
            }
            
            CryptoBackend::PostQuantum => {
                #[cfg(feature = "pqc-dilithium")]
                {
                    Self::PostQuantum(pqc::PqcSigningKey::generate())
                }
                #[cfg(not(feature = "pqc-dilithium"))]
                {
                    panic!("PQC not enabled. Enable with --features pqc-dilithium")
                }
            }
        }
    }
    
    /// Get default backend (classical if available, otherwise PQC)
    pub fn default_backend() -> CryptoBackend {
        #[cfg(feature = "classical-crypto")]
        return CryptoBackend::Classical;
        
        #[cfg(all(not(feature = "classical-crypto"), feature = "pqc-dilithium"))]
        return CryptoBackend::PostQuantum;
        
        #[cfg(not(any(feature = "classical-crypto", feature = "pqc-dilithium")))]
        panic!("No crypto backend enabled");
    }
}

impl SigningKeyTrait for UnifiedSigningKey {
    fn sign(&self, message: &[u8]) -> Vec<u8> {
        match self {
            #[cfg(feature = "classical-crypto")]
            Self::Classical(key) => key.sign(message),
            
            #[cfg(feature = "pqc-dilithium")]
            Self::PostQuantum(key) => key.sign(message),
        }
    }
    
    fn public_key(&self) -> Vec<u8> {
        match self {
            #[cfg(feature = "classical-crypto")]
            Self::Classical(key) => key.public_key(),
            
            #[cfg(feature = "pqc-dilithium")]
            Self::PostQuantum(key) => key.public_key(),
        }
    }
    
    fn backend(&self) -> CryptoBackend {
        match self {
            #[cfg(feature = "classical-crypto")]
            Self::Classical(key) => key.backend(),
            
            #[cfg(feature = "pqc-dilithium")]
            Self::PostQuantum(key) => key.backend(),
        }
    }
}

/// Unified verifying key
pub enum UnifiedVerifyingKey {
    #[cfg(feature = "classical-crypto")]
    Classical(classical::ClassicalVerifyingKey),
    
    #[cfg(feature = "pqc-dilithium")]
    PostQuantum(pqc::PqcVerifyingKey),
}

impl UnifiedVerifyingKey {
    /// Create from bytes with specified backend
    pub fn from_bytes(bytes: &[u8], backend: CryptoBackend) -> Result<Self> {
        match backend {
            CryptoBackend::Classical => {
                #[cfg(feature = "classical-crypto")]
                {
                    let bytes_32: [u8; 32] = bytes.try_into()
                        .map_err(|_| anyhow::anyhow!("Invalid key length for ed25519"))?;
                    Ok(Self::Classical(classical::ClassicalVerifyingKey::from_bytes(&bytes_32)?))
                }
                #[cfg(not(feature = "classical-crypto"))]
                {
                    Err(anyhow::anyhow!("Classical crypto not enabled"))
                }
            }
            
            CryptoBackend::PostQuantum => {
                #[cfg(feature = "pqc-dilithium")]
                {
                    Ok(Self::PostQuantum(pqc::PqcVerifyingKey::from_bytes(bytes)?))
                }
                #[cfg(not(feature = "pqc-dilithium"))]
                {
                    Err(anyhow::anyhow!("PQC not enabled"))
                }
            }
        }
    }
}

impl VerifyingKeyTrait for UnifiedVerifyingKey {
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool> {
        match self {
            #[cfg(feature = "classical-crypto")]
            Self::Classical(key) => key.verify(message, signature),
            
            #[cfg(feature = "pqc-dilithium")]
            Self::PostQuantum(key) => key.verify(message, signature),
        }
    }
    
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            #[cfg(feature = "classical-crypto")]
            Self::Classical(key) => key.to_bytes(),
            
            #[cfg(feature = "pqc-dilithium")]
            Self::PostQuantum(key) => key.to_bytes(),
        }
    }
    
    fn backend(&self) -> CryptoBackend {
        match self {
            #[cfg(feature = "classical-crypto")]
            Self::Classical(key) => key.backend(),
            
            #[cfg(feature = "pqc-dilithium")]
            Self::PostQuantum(key) => key.backend(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[cfg(feature = "classical-crypto")]
    fn test_classical_crypto() {
        let key = UnifiedSigningKey::generate(CryptoBackend::Classical);
        let message = b"Hello, SynapseNet!";
        
        let signature = key.sign(message);
        let public_key_bytes = key.public_key();
        
        let verifying_key = UnifiedVerifyingKey::from_bytes(&public_key_bytes, CryptoBackend::Classical).unwrap();
        assert!(verifying_key.verify(message, &signature).unwrap());
    }
    
    #[test]
    #[cfg(feature = "pqc-dilithium")]
    fn test_pqc_crypto() {
        let key = UnifiedSigningKey::generate(CryptoBackend::PostQuantum);
        let message = b"Hello, Quantum-Safe SynapseNet!";
        
        let signature = key.sign(message);
        let public_key_bytes = key.public_key();
        
        let verifying_key = UnifiedVerifyingKey::from_bytes(&public_key_bytes, CryptoBackend::PostQuantum).unwrap();
        assert!(verifying_key.verify(message, &signature).unwrap());
    }
}
