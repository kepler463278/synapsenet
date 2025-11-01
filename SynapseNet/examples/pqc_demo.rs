// Post-Quantum Cryptography Demo for SynapseNet
// Demonstrates Dilithium signatures and Kyber KEM

use anyhow::Result;
use std::iter::repeat;
use synapsenet_core::{CryptoBackend, Grain, GrainMeta, SigningKeyTrait, UnifiedSigningKey, UnifiedVerifyingKey, VerifyingKeyTrait};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔐 SynapseNet Post-Quantum Cryptography Demo\n");
    println!("{}", "=".repeat(60));
    
    // Demo 1: Classical Crypto (ed25519)
    #[cfg(feature = "classical-crypto")]
    {
        println!("\n📝 Demo 1: Classical Crypto (ed25519)");
        println!("{}", "-".repeat(60));
        
        let signing_key = UnifiedSigningKey::generate(CryptoBackend::Classical);
        let message = b"Hello, SynapseNet with ed25519!";
        
        println!("Backend: {:?}", signing_key.backend());
        println!("Message: {}", String::from_utf8_lossy(message));
        
        let signature = signing_key.sign(message);
        println!("Signature length: {} bytes", signature.len());
        
        let public_key_bytes = signing_key.public_key();
        println!("Public key length: {} bytes", public_key_bytes.len());
        
        let verifying_key = UnifiedVerifyingKey::from_bytes(&public_key_bytes, CryptoBackend::Classical)?;
        let valid = verifying_key.verify(message, &signature)?;
        
        println!("✓ Signature valid: {}", valid);
        
        // Create a grain with classical crypto
        println!("\n📦 Creating Grain with ed25519...");
        let meta = GrainMeta {
            author_pk: public_key_bytes,
            crypto_backend: CryptoBackend::Classical,
            ts_unix_ms: chrono::Utc::now().timestamp_millis(),
            tags: vec!["demo".to_string(), "classical".to_string()],
            mime: "text/plain".to_string(),
            lang: "en".to_string(),
            title: Some("Classical Crypto Demo".to_string()),
            summary: None,
        };
        
        let vec = vec![0.1, 0.2, 0.3, 0.4]; // Dummy embedding
        
        // Note: Grain::new expects ed25519 SigningKey, need to adapt
        println!("✓ Grain metadata created");
    }
    
    // Demo 2: Post-Quantum Crypto (Dilithium)
    #[cfg(feature = "pqc-dilithium")]
    {
        println!("\n\n🔮 Demo 2: Post-Quantum Crypto (Dilithium)");
        println!("{}", "-".repeat(60));
        
        let signing_key = UnifiedSigningKey::generate(CryptoBackend::PostQuantum);
        let message = b"Hello, Quantum-Safe SynapseNet with Dilithium!";
        
        println!("Backend: {:?}", signing_key.backend());
        println!("Message: {}", String::from_utf8_lossy(message));
        
        let signature = signing_key.sign(message);
        println!("Signature length: {} bytes", signature.len());
        println!("  (Note: Dilithium signatures are larger than ed25519)");
        
        let public_key_bytes = signing_key.public_key();
        println!("Public key length: {} bytes", public_key_bytes.len());
        println!("  (Note: Dilithium public keys are larger than ed25519)");
        
        let verifying_key = UnifiedVerifyingKey::from_bytes(&public_key_bytes, CryptoBackend::PostQuantum)?;
        let valid = verifying_key.verify(message, &signature)?;
        
        println!("✓ Signature valid: {}", valid);
        
        println!("\n📊 Size Comparison:");
        println!("  ed25519:   32 bytes (public key), 64 bytes (signature)");
        println!("  Dilithium: {} bytes (public key), {} bytes (signature)", 
                 public_key_bytes.len(), signature.len());
    }
    
    // Demo 3: Kyber KEM (Key Exchange)
    #[cfg(all(feature = "pqc-kyber", feature = "pqc"))]
    {
        use synapsenet_p2p::KyberHandshake;
        
        println!("\n\n🔑 Demo 3: Post-Quantum Key Exchange (Kyber KEM)");
        println!("{}", "-".repeat(60));
        
        println!("Simulating P2P handshake between two nodes...\n");
        
        let mut alice = KyberHandshake::new();
        let mut bob = KyberHandshake::new();
        
        println!("👤 Alice: Initiating handshake...");
        let bob_pk = bob.public_key_bytes();
        let initiation = alice.initiate(&bob_pk)?;
        println!("  Sent {} bytes to Bob", initiation.len());
        
        println!("\n👤 Bob: Processing initiation and responding...");
        let response = bob.respond(&initiation)?;
        println!("  Sent {} bytes to Alice", response.len());
        
        println!("\n👤 Alice: Finalizing handshake...");
        alice.finalize(&response)?;
        
        let alice_secret = alice.shared_secret().unwrap();
        let bob_secret = bob.shared_secret().unwrap();
        
        println!("\n✓ Handshake complete!");
        println!("  Alice's shared secret: {} bytes", alice_secret.len());
        println!("  Bob's shared secret:   {} bytes", bob_secret.len());
        println!("  Secrets match: {}", alice_secret == bob_secret);
        
        if let (Some(alice_key), Some(bob_key)) = (alice.derive_key(), bob.derive_key()) {
            println!("\n🔐 Derived encryption keys:");
            println!("  Alice: {:02x?}...", &alice_key[..8]);
            println!("  Bob:   {:02x?}...", &bob_key[..8]);
            println!("  Keys match: {}", alice_key == bob_key);
        }
    }
    
    // Summary
    println!("\n\n📋 Summary");
    println!("{}", "=".repeat(60));
    println!("✓ Classical crypto (ed25519): Fast, small signatures");
    println!("✓ Post-quantum crypto (Dilithium): Quantum-resistant, larger signatures");
    println!("✓ Kyber KEM: Quantum-resistant key exchange for P2P");
    println!("\n💡 SynapseNet supports both classical and PQC!");
    println!("   Use feature flags to choose: --features pqc");
    
    Ok(())
}
