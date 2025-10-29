use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

/// Semantic link between grains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    /// Source grain ID
    pub from: [u8; 32],
    /// Target grain ID
    pub to: [u8; 32],
    /// Link strength (0.0..1.0)
    pub weight: f32,
    /// Optional rationale/explanation
    pub rationale: Option<String>,
    /// Signature
    pub sig: Vec<u8>,
}

impl Link {
    /// Create new link with signature
    pub fn new(
        from: [u8; 32],
        to: [u8; 32],
        weight: f32,
        rationale: Option<String>,
        signing_key: &SigningKey,
    ) -> Result<Self, anyhow::Error> {
        // Create link data for signing
        let mut data = Vec::new();
        data.extend_from_slice(&from);
        data.extend_from_slice(&to);
        data.extend_from_slice(&weight.to_le_bytes());
        if let Some(ref r) = rationale {
            data.extend_from_slice(r.as_bytes());
        }

        let signature = signing_key.sign(&data);

        Ok(Link {
            from,
            to,
            weight: weight.clamp(0.0, 1.0),
            rationale,
            sig: signature.to_bytes().to_vec(),
        })
    }

    /// Verify link signature
    pub fn verify(&self, author_pk: &[u8; 32]) -> Result<bool, anyhow::Error> {
        let verifying_key = VerifyingKey::from_bytes(author_pk)?;
        let signature = Signature::from_bytes(
            self.sig
                .as_slice()
                .try_into()
                .map_err(|_| anyhow::anyhow!("Invalid signature length"))?,
        );

        let mut data = Vec::new();
        data.extend_from_slice(&self.from);
        data.extend_from_slice(&self.to);
        data.extend_from_slice(&self.weight.to_le_bytes());
        if let Some(ref r) = self.rationale {
            data.extend_from_slice(r.as_bytes());
        }

        Ok(verifying_key.verify(&data, &signature).is_ok())
    }
}
