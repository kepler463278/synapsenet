//! Hardware capability detection for mobile devices

use serde::{Deserialize, Serialize};

/// Hardware capabilities of the mobile device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareCapabilities {
    /// Device has GPU available
    pub has_gpu: bool,
    
    /// Device has NPU (Neural Processing Unit) available
    pub has_npu: bool,
    
    /// Device supports CoreML (iOS)
    pub supports_coreml: bool,
    
    /// Device supports NNAPI (Android)
    pub supports_nnapi: bool,
    
    /// Available RAM in megabytes
    pub ram_mb: usize,
    
    /// Number of CPU cores
    pub cpu_cores: usize,
    
    /// Device model/name
    pub device_model: String,
    
    /// OS version
    pub os_version: String,
}

impl HardwareCapabilities {
    /// Detect hardware capabilities of the current device
    pub fn detect() -> Self {
        #[cfg(target_os = "ios")]
        {
            Self::detect_ios()
        }
        
        #[cfg(target_os = "android")]
        {
            Self::detect_android()
        }
        
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        {
            Self::default()
        }
    }
    
    #[cfg(target_os = "ios")]
    fn detect_ios() -> Self {
        // TODO: Implement iOS-specific detection using UIDevice, Metal, etc.
        Self {
            has_gpu: true, // Most iOS devices have Metal-capable GPU
            has_npu: true, // A11+ has Neural Engine
            supports_coreml: true,
            supports_nnapi: false,
            ram_mb: 4096, // Placeholder
            cpu_cores: num_cpus::get(),
            device_model: "iPhone".to_string(), // TODO: Get actual model
            os_version: "iOS 14+".to_string(), // TODO: Get actual version
        }
    }
    
    #[cfg(target_os = "android")]
    fn detect_android() -> Self {
        // TODO: Implement Android-specific detection using Build, ActivityManager, etc.
        Self {
            has_gpu: true, // Most Android devices have GPU
            has_npu: false, // Varies by device
            supports_coreml: false,
            supports_nnapi: true, // Android 8.1+
            ram_mb: 4096, // Placeholder
            cpu_cores: num_cpus::get(),
            device_model: "Android Device".to_string(), // TODO: Get actual model
            os_version: "Android 8+".to_string(), // TODO: Get actual version
        }
    }
    
    /// Check if device can run AI models efficiently
    pub fn can_run_ai(&self) -> bool {
        self.ram_mb >= 2048 && (self.has_gpu || self.has_npu || self.cpu_cores >= 4)
    }
    
    /// Get recommended AI provider for this device
    pub fn recommended_ai_provider(&self) -> AIProvider {
        if self.supports_coreml && self.has_npu {
            AIProvider::CoreML
        } else if self.supports_nnapi && self.has_npu {
            AIProvider::NNAPI
        } else if self.has_gpu {
            AIProvider::GPU
        } else {
            AIProvider::CPU
        }
    }
}

impl Default for HardwareCapabilities {
    fn default() -> Self {
        Self {
            has_gpu: false,
            has_npu: false,
            supports_coreml: false,
            supports_nnapi: false,
            ram_mb: 2048,
            cpu_cores: num_cpus::get(),
            device_model: "Unknown".to_string(),
            os_version: "Unknown".to_string(),
        }
    }
}

/// AI provider types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AIProvider {
    /// Apple CoreML (iOS)
    CoreML,
    
    /// Android NNAPI
    NNAPI,
    
    /// GPU acceleration
    GPU,
    
    /// CPU fallback
    CPU,
}

impl AIProvider {
    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            Self::CoreML => "CoreML",
            Self::NNAPI => "NNAPI",
            Self::GPU => "GPU",
            Self::CPU => "CPU",
        }
    }
    
    /// Check if this provider is available on current platform
    pub fn is_available(&self) -> bool {
        match self {
            Self::CoreML => cfg!(target_os = "ios"),
            Self::NNAPI => cfg!(target_os = "android"),
            Self::GPU => true, // Most devices have some GPU
            Self::CPU => true, // Always available
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_capabilities_detection() {
        let caps = HardwareCapabilities::detect();
        assert!(caps.cpu_cores > 0);
        assert!(caps.ram_mb > 0);
    }
    
    #[test]
    fn test_ai_provider_availability() {
        assert!(AIProvider::CPU.is_available());
    }
}
