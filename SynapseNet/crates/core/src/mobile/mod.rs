//! Mobile-specific core functionality for SynapseNet
//!
//! This module provides mobile-optimized implementations for:
//! - Hardware capability detection
//! - Battery and thermal monitoring
//! - Platform-specific integrations
//! - Encrypted storage

#[cfg(target_os = "ios")]
pub mod ios;

#[cfg(target_os = "android")]
pub mod android;

pub mod capabilities;
pub mod battery;
pub mod keystore;
pub mod encryption;
pub mod recovery;
pub mod capsule;

pub use capabilities::HardwareCapabilities;
pub use battery::{BatteryMonitor, ThermalState, MemoryPressure};
pub use keystore::KeyStore;
pub use encryption::{Encryptor, EncryptedBlob};
pub use recovery::RecoveryPhrase;
pub use capsule::MemoryCapsule;
