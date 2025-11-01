//! Application state management

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    pub id: String,
    pub amount: f64,
    pub reason: String,
    pub timestamp: i64,
}

/// Global application state
#[derive(Debug)]
pub struct AppState {
    pub node_running: bool,
    pub start_time: Option<i64>,
    pub balance: f64,
    pub rewards: Vec<Reward>,
    pub peer_count: u32,
    pub grain_count: u32,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            node_running: false,
            start_time: None,
            balance: 0.0,
            rewards: Vec::new(),
            peer_count: 0,
            grain_count: 0,
        }
    }

    pub fn uptime(&self) -> u64 {
        if let Some(start_time) = self.start_time {
            (chrono::Utc::now().timestamp() - start_time) as u64
        } else {
            0
        }
    }

    pub fn add_reward(&mut self, amount: f64, reason: String) {
        let reward = Reward {
            id: uuid::Uuid::new_v4().to_string(),
            amount,
            reason,
            timestamp: chrono::Utc::now().timestamp(),
        };
        self.rewards.push(reward);
        self.balance += amount;
    }

    pub fn today_earnings(&self) -> f64 {
        let today_start = chrono::Utc::now()
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp();

        self.rewards
            .iter()
            .filter(|r| r.timestamp >= today_start)
            .map(|r| r.amount)
            .sum()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
