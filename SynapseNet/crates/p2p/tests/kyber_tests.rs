// Kyber KEM Tests for P2P

#[cfg(feature = "pqc-kyber")]
#[test]
fn test_kyber_keypair_generation() {
    use synapsenet_p2p::KyberKem;
    
    let kem = KyberKem::generate();
    let pk = kem.public_key_bytes();
    let sk = kem.secret_key_bytes();
    
    assert!(pk.len() > 1000, "Kyber1024 public key should be ~1568 bytes");
    assert!(sk.len() > 3000, "Kyber1024 secret key should be ~3168 bytes");
}

#[cfg(feature = "pqc-kyber")]
#[test]
fn test_kyber_encapsulation_decapsulation() {
    use synapsenet_p2p::KyberKem;
    use pqcrypto_kyber::kyber1024;
    use pqcrypto_traits::kem::PublicKey;
    
    let kem = KyberKem::generate();
    let pk_bytes = kem.public_key_bytes();
    let pk = kyber1024::PublicKey::from_bytes(&pk_bytes).unwrap();
    
    // Encapsulate
    let (ss1, ct) = KyberKem::encapsulate(&pk);
    
    // Decapsulate
    let ss2 = kem.decapsulate(&ct);
    
    assert_eq!(ss1.as_bytes(), ss2.as_bytes(), "Shared secrets should match");
}

#[cfg(feature = "pqc-kyber")]
#[test]
fn test_kyber_handshake_full_flow() {
    use synapsenet_p2p::KyberHandshake;
    
    // Create two parties
    let mut alice = KyberHandshake::new();
    let mut bob = KyberHandshake::new();
    
    // Get Bob's public key
    let bob_pk = bob.kem.public_key_bytes();
    
    // Alice initiates
    let initiation = alice.initiate(&bob_pk).unwrap();
    assert!(initiation.len() > 3000, "Initiation should contain ciphertext + public key");
    
    // Bob responds
    let response = bob.respond(&initiation).unwrap();
    assert!(response.len() > 1500, "Response should contain ciphertext");
    
    // Alice finalizes
    alice.finalize(&response).unwrap();
    
    // Both should have shared secret
    let alice_secret = alice.shared_secret().unwrap();
    let bob_secret = bob.shared_secret().unwrap();
    
    assert_eq!(alice_secret, bob_secret, "Shared secrets must match");
    assert_eq!(alice_secret.len(), 32, "Shared secret should be 32 bytes (blake3 hash)");
}

#[cfg(feature = "pqc-kyber")]
#[test]
fn test_kyber_derive_key() {
    use synapsenet_p2p::KyberHandshake;
    
    let mut alice = KyberHandshake::new();
    let mut bob = KyberHandshake::new();
    
    let bob_pk = bob.kem.public_key_bytes();
    let initiation = alice.initiate(&bob_pk).unwrap();
    let response = bob.respond(&initiation).unwrap();
    alice.finalize(&response).unwrap();
    
    // Derive encryption keys
    let alice_key = alice.derive_key().unwrap();
    let bob_key = bob.derive_key().unwrap();
    
    assert_eq!(alice_key, bob_key, "Derived keys must match");
    assert_eq!(alice_key.len(), 32, "Derived key should be 32 bytes");
}

#[cfg(feature = "pqc-kyber")]
#[test]
fn test_kyber_handshake_invalid_public_key() {
    use synapsenet_p2p::KyberHandshake;
    
    let mut alice = KyberHandshake::new();
    
    // Try to initiate with invalid public key
    let invalid_pk = vec![0u8; 100]; // Too short
    let result = alice.initiate(&invalid_pk);
    
    assert!(result.is_err(), "Should fail with invalid public key");
}

#[cfg(feature = "pqc-kyber")]
#[test]
fn test_kyber_handshake_invalid_initiation() {
    use synapsenet_p2p::KyberHandshake;
    
    let mut bob = KyberHandshake::new();
    
    // Try to respond to invalid initiation
    let invalid_initiation = vec![0u8; 100]; // Too short
    let result = bob.respond(&invalid_initiation);
    
    assert!(result.is_err(), "Should fail with invalid initiation");
}

#[cfg(feature = "pqc-kyber")]
#[test]
fn test_kyber_handshake_invalid_response() {
    use synapsenet_p2p::KyberHandshake;
    
    let mut alice = KyberHandshake::new();
    let mut bob = KyberHandshake::new();
    
    let bob_pk = bob.kem.public_key_bytes();
    let _initiation = alice.initiate(&bob_pk).unwrap();
    
    // Try to finalize with invalid response
    let invalid_response = vec![0u8; 100]; // Too short
    let result = alice.finalize(&invalid_response);
    
    assert!(result.is_err(), "Should fail with invalid response");
}

#[cfg(feature = "pqc-kyber")]
#[test]
fn test_kyber_multiple_handshakes() {
    use synapsenet_p2p::KyberHandshake;
    
    // Test multiple independent handshakes
    for _ in 0..5 {
        let mut alice = KyberHandshake::new();
        let mut bob = KyberHandshake::new();
        
        let bob_pk = bob.kem.public_key_bytes();
        let initiation = alice.initiate(&bob_pk).unwrap();
        let response = bob.respond(&initiation).unwrap();
        alice.finalize(&response).unwrap();
        
        let alice_secret = alice.shared_secret().unwrap();
        let bob_secret = bob.shared_secret().unwrap();
        
        assert_eq!(alice_secret, bob_secret);
    }
}

#[cfg(feature = "pqc-kyber")]
#[test]
fn test_kyber_shared_secret_uniqueness() {
    use synapsenet_p2p::KyberHandshake;
    use std::collections::HashSet;
    
    let mut secrets = HashSet::new();
    
    // Generate multiple handshakes and collect secrets
    for _ in 0..10 {
        let mut alice = KyberHandshake::new();
        let mut bob = KyberHandshake::new();
        
        let bob_pk = bob.kem.public_key_bytes();
        let initiation = alice.initiate(&bob_pk).unwrap();
        let response = bob.respond(&initiation).unwrap();
        alice.finalize(&response).unwrap();
        
        let secret = alice.shared_secret().unwrap().to_vec();
        secrets.insert(secret);
    }
    
    // All secrets should be unique
    assert_eq!(secrets.len(), 10, "All shared secrets should be unique");
}
