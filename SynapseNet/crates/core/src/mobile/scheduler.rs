//! Background task scheduler for mobile platforms

use anyhow::Result;
use std::time::Duration;

/// Scheduled task
pub struct ScheduledTask {
    pub id: String,
    pub interval: Duration,
    pub requires_wifi: bool,
    pub requires_charging: bool,
}

/// Platform scheduler trait
pub trait PlatformScheduler: Send + Sync {
    fn schedule_task(&self, task: &ScheduledTask) -> Result<()>;
    fn cancel_task(&self, id: &str) -> Result<()>;
    fn is_background_allowed(&self) -> bool;
}

/// Background scheduler
pub struct BackgroundScheduler {
    tasks: Vec<ScheduledTask>,
}

impl BackgroundScheduler {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }
    
    pub fn schedule_sync(&mut self, interval: Duration) {
        let task = ScheduledTask {
            id: "sync".to_string(),
            interval,
            requires_wifi: false,
            requires_charging: false,
        };
        self.tasks.push(task);
    }
}

impl Default for BackgroundScheduler {
    fn default() -> Self {
        Self::new()
    }
}
