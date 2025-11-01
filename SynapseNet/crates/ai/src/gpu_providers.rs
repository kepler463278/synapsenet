// GPU Execution Providers for ONNX Runtime
// Supports CoreML (Mac), DirectML (Windows), CUDA (Linux/Windows)

use anyhow::Result;
use tracing::{info, warn};

/// GPU provider type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuProvider {
    /// CPU only (fallback)
    Cpu,
    /// CoreML (Mac - uses Metal)
    CoreML,
    /// DirectML (Windows - any GPU)
    DirectML,
    /// CUDA (NVIDIA GPUs)
    Cuda,
}

impl GpuProvider {
    /// Detect best available provider for current platform
    pub fn detect() -> Self {
        #[cfg(all(target_os = "macos", feature = "coreml"))]
        {
            info!("Detected macOS with CoreML support");
            return Self::CoreML;
        }

        #[cfg(all(target_os = "windows", feature = "directml"))]
        {
            info!("Detected Windows with DirectML support");
            return Self::DirectML;
        }

        #[cfg(feature = "cuda")]
        {
            // Check if CUDA is available
            if Self::is_cuda_available() {
                info!("Detected CUDA support");
                return Self::Cuda;
            }
        }

        info!("No GPU provider available, using CPU");
        Self::Cpu
    }

    /// Check if CUDA is available
    #[cfg(feature = "cuda")]
    fn is_cuda_available() -> bool {
        // Simple check - try to detect CUDA runtime
        // In production, you'd check for libcuda.so / nvcuda.dll
        std::env::var("CUDA_PATH").is_ok() || std::path::Path::new("/usr/local/cuda").exists()
    }

    /// Get provider name for ONNX Runtime
    pub fn provider_name(&self) -> &'static str {
        match self {
            Self::Cpu => "CPUExecutionProvider",
            Self::CoreML => "CoreMLExecutionProvider",
            Self::DirectML => "DmlExecutionProvider",
            Self::Cuda => "CUDAExecutionProvider",
        }
    }

    /// Log provider configuration
    pub fn log_configuration(&self) {
        match self {
            Self::Cpu => {
                info!("Using CPU execution provider");
            }

            #[cfg(feature = "coreml")]
            Self::CoreML => {
                info!("Using CoreML execution provider (Metal backend)");
            }

            #[cfg(not(feature = "coreml"))]
            Self::CoreML => {
                warn!("CoreML requested but not compiled in, falling back to CPU");
            }

            #[cfg(feature = "directml")]
            Self::DirectML => {
                info!("Using DirectML execution provider");
            }

            #[cfg(not(feature = "directml"))]
            Self::DirectML => {
                warn!("DirectML requested but not compiled in, falling back to CPU");
            }

            #[cfg(feature = "cuda")]
            Self::Cuda => {
                info!("Using CUDA execution provider");
            }

            #[cfg(not(feature = "cuda"))]
            Self::Cuda => {
                warn!("CUDA requested but not compiled in, falling back to CPU");
            }
        }
    }

    /// Get expected speedup factor (approximate)
    pub fn speedup_factor(&self) -> f32 {
        match self {
            Self::Cpu => 1.0,
            Self::CoreML => 3.0,   // Metal on Apple Silicon
            Self::DirectML => 2.5, // DirectML on modern GPUs
            Self::Cuda => 4.0,     // CUDA on NVIDIA GPUs
        }
    }
}

impl std::fmt::Display for GpuProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cpu => write!(f, "CPU"),
            Self::CoreML => write!(f, "CoreML (Metal)"),
            Self::DirectML => write!(f, "DirectML"),
            Self::Cuda => write!(f, "CUDA"),
        }
    }
}

impl std::str::FromStr for GpuProvider {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "cpu" => Ok(Self::Cpu),
            "coreml" | "metal" => Ok(Self::CoreML),
            "directml" | "dml" => Ok(Self::DirectML),
            "cuda" => Ok(Self::Cuda),
            _ => Err(anyhow::anyhow!("Unknown GPU provider: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_detection() {
        let provider = GpuProvider::detect();
        println!("Detected provider: {}", provider);
        assert!(matches!(
            provider,
            GpuProvider::Cpu | GpuProvider::CoreML | GpuProvider::DirectML | GpuProvider::Cuda
        ));
    }

    #[test]
    fn test_provider_names() {
        assert_eq!(GpuProvider::Cpu.provider_name(), "CPUExecutionProvider");
        assert_eq!(
            GpuProvider::CoreML.provider_name(),
            "CoreMLExecutionProvider"
        );
        assert_eq!(
            GpuProvider::DirectML.provider_name(),
            "DmlExecutionProvider"
        );
        assert_eq!(GpuProvider::Cuda.provider_name(), "CUDAExecutionProvider");
    }

    #[test]
    fn test_provider_from_str() {
        assert!(matches!(
            "cpu".parse::<GpuProvider>().unwrap(),
            GpuProvider::Cpu
        ));
        assert!(matches!(
            "coreml".parse::<GpuProvider>().unwrap(),
            GpuProvider::CoreML
        ));
        assert!(matches!(
            "directml".parse::<GpuProvider>().unwrap(),
            GpuProvider::DirectML
        ));
        assert!(matches!(
            "cuda".parse::<GpuProvider>().unwrap(),
            GpuProvider::Cuda
        ));
    }
}
