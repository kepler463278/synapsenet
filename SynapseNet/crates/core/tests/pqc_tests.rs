// Post-Quantum Cryptography Tests

#[cfg(feature = "classical-crypto")]
#[test]
fn test_classical_signing_and_verification() {
    use synapsenet_core::{CryptoBackend, SigningKeyTrait, UnifiedSigningKey, UnifiedVerifyingKey, VerifyingKeyTrait};
    
    let signing_key = UnifiedSigningKey::generate(CryptoBackend::Classical);
    let message = b"Test message for classical crypto";
    
    let signature = signing_key.sign(message);
    let public_key = signing_key.public_key();
    
    let verifying_key = UnifiedVerifyingKey::from_bytes(&public_key, CryptoBackend::Classical).unwrap();
    assert!(verifying_key.verify(message, &signature).unwrap());
    
    // Wrong message should fail
    let wrong_message = b"Wrong message";
    assert!(!verifying_key.verify(wrong_message, &signature).unwrap());
}

#[cfg(feature = "pqc-dilithium")]
#[test]
fn test_pqc_signing_and_verification() {
    use synapsenet_core::{CryptoBackend, SigningKeyTrait, UnifiedSigningKey, UnifiedVerifyingKey, VerifyingKeyTrait};
    
    let signing_key = UnifiedSigningKey::generate(CryptoBackend::PostQuantum);
    let message = b"Test message for post-quantum crypto";
    
    let signature = signing_key.sign(message);
    let public_key = signing_key.public_key();
    
    // Verify signature size is larger for PQC
    assert!(signature.len() > 1000, "PQC signature should be large");
    assert!(public_key.len() > 1000, "PQC public key should be large");
    
    let verifying_key = UnifiedVerifyingKey::from_bytes(&public_key, CryptoBackend::PostQuantum).unwrap();
    assert!(verifying_key.verify(message, &signature).unwrap());
    
    // Wrong message should fail
    let wrong_message = b"Wrong message";
    assert!(!verifying_key.verify(wrong_message, &signature).unwrap());
}

#[cfg(all(feature = "classical-crypto", feature = "pqc-dilithium"))]
#[test]
fn test_crypto_backend_detection() {
    use synapsenet_core::{CryptoBackend, SigningKeyTrait, UnifiedSigningKey};
    
    let classical_key = UnifiedSigningKey::generate(CryptoBackend::Classical);
    assert_eq!(classical_key.backend(), CryptoBackend::Classical);
    
    let pqc_key = UnifiedSigningKey::generate(CryptoBackend::PostQuantum);
    assert_eq!(pqc_key.backend(), CryptoBackend::PostQuantum);
}

#[cfg(feature = "classical-crypto")]
#[test]
fn test_classical_signature_size() {
    use synapsenet_core::{CryptoBackend, SigningKeyTrait, UnifiedSigningKey};
    
    let signing_key = UnifiedSigningKey::generate(CryptoBackend::Classical);
    let message = b"Test";
    
    let signature = signing_key.sign(message);
    let public_key = signing_key.public_key();
    
    assert_eq!(signature.len(), 64, "ed25519 signature should be 64 bytes");
    assert_eq!(public_key.len(), 32, "ed25519 public key should be 32 bytes");
}

#[cfg(feature = "pqc-dilithium")]
#[test]
fn test_pqc_signature_size() {
    use synapsenet_core::{CryptoBackend, SigningKeyTrait, UnifiedSigningKey};
    
    let signing_key = UnifiedSigningKey::generate(CryptoBackend::PostQuantum);
    let message = b"Test";
    
    let signature = signing_key.sign(message);
    let public_key = signing_key.public_key();
    
    // Dilithium5 sizes
    assert!(signature.len() > 4000, "Dilithium signature should be ~4595 bytes");
    assert!(public_key.len() > 2000, "Dilithium public key should be ~2592 bytes");
}

#[cfg(feature = "classical-crypto")]
#[test]
fn test_multiple_signatures_same_key() {
    use synapsenet_core::{CryptoBackend, SigningKeyTrait, UnifiedSigningKey, UnifiedVerifyingKey, VerifyingKeyTrait};
    
    let signing_key = UnifiedSigningKey::generate(CryptoBackend::Classical);
    let public_key = signing_key.public_key();
    let verifying_key = UnifiedVerifyingKey::from_bytes(&public_key, CryptoBackend::Classical).unwrap();
    
    // Sign multiple messages
    let messages = vec![
        b"Message 1".as_slice(),
        b"Message 2".as_slice(),
        b"Message 3".as_slice(),
    ];
    
    for message in messages {
        let signature = signing_key.sign(message);
        assert!(verifying_key.verify(message, &signature).unwrap());
    }
}

#[cfg(feature = "pqc-dilithium")]
#[test]
fn test_pqc_deterministic_public_key() {
    use synapsenet_core::{CryptoBackend, SigningKeyTrait, UnifiedSigningKey};
    
    let signing_key = UnifiedSigningKey::generate(CryptoBackend::PostQuantum);
    
    let pk1 = signing_key.public_key();
    let pk2 = signing_key.public_key();
    
    assert_eq!(pk1, pk2, "Public key should be deterministic");
}
