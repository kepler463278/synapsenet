//! Network state detection and management for mobile

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Network state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkState {
    /// No network connection
    Offline,
    
    /// Connected via WiFi
    WiFi,
    
    /// Connected via cellular data
    Cellular,
    
    /// Unknown connection type
    Unknown,
}

impl NetworkState {
    /// Check if network is available
    pub fn is_online(&self) -> bool {
        !matches!(self, Self::Offline)
    }
    
    /// Check if on WiFi
    pub fn is_wifi(&self) -> bool {
        matches!(self, Self::WiFi)
    }
    
    /// Check if on cellular
    pub fn is_cellular(&self) -> bool {
        matches!(self, Self::Cellular)
    }
}

/// Network state monitor
pub struct NetworkStateMonitor {
    current_state: NetworkState,
    last_check: Instant,
    check_interval: Duration,
    cellular_limit_mbps: Option<f32>,
    bandwidth_usage_mb: f32,
}

impl NetworkStateMonitor {
    /// Create a new network state monitor
    pub fn new() -> Self {
        Self {
            current_state: NetworkState::Unknown,
            last_check: Instant::now(),
            check_interval: Duration::from_secs(5),
            cellular_limit_mbps: Some(1.0), // 1 Mbps default limit
            bandwidth_usage_mb: 0.0,
        }
    }
    
    /// Update network state
    pub fn update(&mut self) {
        if self.last_check.elapsed() < self.check_interval {
            return;
        }
        
        self.current_state = Self::detect_network_state();
        self.last_check = Instant::now();
        
        tracing::debug!("Network state: {:?}", self.current_state);
    }
    
    /// Detect current network state
    #[cfg(target_os = "ios")]
    fn detect_network_state() -> NetworkState {
        // TODO: Implement iOS network detection using NWPathMonitor
        NetworkState::WiFi
    }
    
    #[cfg(target_os = "android")]
    fn detect_network_state() -> NetworkState {
        // TODO: Implement Android network detection using ConnectivityManager
        NetworkState::WiFi
    }
    
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    fn detect_network_state() -> NetworkState {
        NetworkState::Unknown
    }
    
    /// Get current network state
    pub fn current_state(&self) -> NetworkState {
        self.current_state
    }
    
    /// Check if operation should proceed based on network state
    pub fn should_proceed(&self, requires_wifi: bool) -> bool {
        match self.current_state {
            NetworkState::Offline => false,
            NetworkState::WiFi => true,
            NetworkState::Cellular => !requires_wifi && self.within_cellular_limit(),
            NetworkState::Unknown => !requires_wifi,
        }
    }
    
    /// Set cellular bandwidth limit
    pub fn set_cellular_limit(&mut self, limit_mbps: Option<f32>) {
        self.cellular_limit_mbps = limit_mbps;
        tracing::info!("Cellular limit set to: {:?} Mbps", limit_mbps);
    }
    
    /// Check if within cellular bandwidth limit
    pub fn within_cellular_limit(&self) -> bool {
        if let Some(limit) = self.cellular_limit_mbps {
            self.bandwidth_usage_mb < limit
        } else {
            true // No limit set
        }
    }
    
    /// Record bandwidth usage
    pub fn record_bandwidth(&mut self, bytes: usize) {
        let mb = bytes as f32 / 1_048_576.0;
        self.bandwidth_usage_mb += mb;
    }
    
    /// Reset bandwidth counter
    pub fn reset_bandwidth(&mut self) {
        self.bandwidth_usage_mb = 0.0;
    }
    
    /// Get bandwidth usage
    pub fn bandwidth_usage_mb(&self) -> f32 {
        self.bandwidth_usage_mb
    }
    
    /// Get connection quality estimate
    pub fn connection_quality(&self) -> ConnectionQuality {
        match self.current_state {
            NetworkState::Offline => ConnectionQuality::None,
            NetworkState::WiFi => ConnectionQuality::Excellent,
            NetworkState::Cellular => ConnectionQuality::Good,
            NetworkState::Unknown => ConnectionQuality::Poor,
        }
    }
}

impl Default for NetworkStateMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Connection quality estimate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionQuality {
    None,
    Poor,
    Fair,
    Good,
    Excellent,
}

impl ConnectionQuality {
    /// Get recommended batch size for this quality
    pub fn recommended_batch_size(&self) -> usize {
        match self {
            Self::None => 0,
            Self::Poor => 1,
            Self::Fair => 4,
            Self::Good => 8,
            Self::Excellent => 16,
        }
    }
    
    /// Check if quality is acceptable for P2P
    pub fn is_acceptable(&self) -> bool {
        !matches!(self, Self::None | Self::Poor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_network_state_checks() {
        assert!(NetworkState::WiFi.is_online());
        assert!(NetworkState::WiFi.is_wifi());
        assert!(!NetworkState::WiFi.is_cellular());
        
        assert!(NetworkState::Cellular.is_online());
        assert!(!NetworkState::Cellular.is_wifi());
        assert!(NetworkState::Cellular.is_cellular());
        
        assert!(!NetworkState::Offline.is_online());
    }
    
    #[test]
    fn test_network_monitor_creation() {
        let monitor = NetworkStateMonitor::new();
        assert_eq!(monitor.current_state(), NetworkState::Unknown);
    }
    
    #[test]
    fn test_should_proceed() {
        let mut monitor = NetworkStateMonitor::new();
        
        monitor.current_state = NetworkState::WiFi;
        assert!(monitor.should_proceed(true));
        assert!(monitor.should_proceed(false));
        
        monitor.current_state = NetworkState::Cellular;
        assert!(!monitor.should_proceed(true)); // WiFi required
        assert!(monitor.should_proceed(false)); // WiFi not required
        
        monitor.current_state = NetworkState::Offline;
        assert!(!monitor.should_proceed(true));
        assert!(!monitor.should_proceed(false));
    }
    
    #[test]
    fn test_cellular_limit() {
        let mut monitor = NetworkStateMonitor::new();
        monitor.current_state = NetworkState::Cellular;
        monitor.set_cellular_limit(Some(1.0));
        
        assert!(monitor.within_cellular_limit());
        
        monitor.record_bandwidth(1_048_576); // 1 MB
        assert!(!monitor.within_cellular_limit());
        
        monitor.reset_bandwidth();
        assert!(monitor.within_cellular_limit());
    }
    
    #[test]
    fn test_bandwidth_tracking() {
        let mut monitor = NetworkStateMonitor::new();
        
        assert_eq!(monitor.bandwidth_usage_mb(), 0.0);
        
        monitor.record_bandwidth(1_048_576); // 1 MB
        assert_eq!(monitor.bandwidth_usage_mb(), 1.0);
        
        monitor.record_bandwidth(524_288); // 0.5 MB
        assert_eq!(monitor.bandwidth_usage_mb(), 1.5);
        
        monitor.reset_bandwidth();
        assert_eq!(monitor.bandwidth_usage_mb(), 0.0);
    }
    
    #[test]
    fn test_connection_quality() {
        let mut monitor = NetworkStateMonitor::new();
        
        monitor.current_state = NetworkState::WiFi;
        assert_eq!(monitor.connection_quality(), ConnectionQuality::Excellent);
        
        monitor.current_state = NetworkState::Cellular;
        assert_eq!(monitor.connection_quality(), ConnectionQuality::Good);
        
        monitor.current_state = NetworkState::Offline;
        assert_eq!(monitor.connection_quality(), ConnectionQuality::None);
    }
    
    #[test]
    fn test_connection_quality_batch_size() {
        assert_eq!(ConnectionQuality::Excellent.recommended_batch_size(), 16);
        assert_eq!(ConnectionQuality::Good.recommended_batch_size(), 8);
        assert_eq!(ConnectionQuality::Fair.recommended_batch_size(), 4);
        assert_eq!(ConnectionQuality::Poor.recommended_batch_size(), 1);
        assert_eq!(ConnectionQuality::None.recommended_batch_size(), 0);
    }
}
