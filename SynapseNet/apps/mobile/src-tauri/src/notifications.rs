//! Cross-platform notification system for mobile

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub body: String,
    pub category: NotificationCategory,
    pub data: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationCategory {
    SyncComplete,
    NewPeer,
    RewardEarned,
    BackupReminder,
    LowBattery,
    Custom,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub enabled: bool,
    pub sync_notifications: bool,
    pub peer_notifications: bool,
    pub reward_notifications: bool,
    pub reminder_notifications: bool,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            sync_notifications: true,
            peer_notifications: true,
            reward_notifications: true,
            reminder_notifications: true,
        }
    }
}

/// Request notification permission
#[tauri::command]
pub async fn request_notification_permission() -> Result<bool, String> {
    tracing::info!("Requesting notification permission");
    
    #[cfg(target_os = "ios")]
    {
        request_ios_notification_permission().await
    }
    
    #[cfg(target_os = "android")]
    {
        request_android_notification_permission().await
    }
    
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        Ok(false)
    }
}

/// Send a notification
#[tauri::command]
pub async fn send_notification(notification: Notification) -> Result<(), String> {
    tracing::info!("Sending notification: {:?}", notification.title);
    
    #[cfg(target_os = "ios")]
    {
        send_ios_notification(notification).await
    }
    
    #[cfg(target_os = "android")]
    {
        send_android_notification(notification).await
    }
    
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        Ok(())
    }
}

/// Schedule a notification for later
#[tauri::command]
pub async fn schedule_notification(
    notification: Notification,
    delay_seconds: u64,
) -> Result<String, String> {
    tracing::info!(
        "Scheduling notification '{}' for {} seconds",
        notification.title,
        delay_seconds
    );
    
    #[cfg(target_os = "ios")]
    {
        schedule_ios_notification(notification, delay_seconds).await
    }
    
    #[cfg(target_os = "android")]
    {
        schedule_android_notification(notification, delay_seconds).await
    }
    
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        Ok("mock_id".to_string())
    }
}

/// Cancel a scheduled notification
#[tauri::command]
pub async fn cancel_notification(notification_id: String) -> Result<(), String> {
    tracing::info!("Cancelling notification: {}", notification_id);
    
    #[cfg(target_os = "ios")]
    {
        cancel_ios_notification(notification_id).await
    }
    
    #[cfg(target_os = "android")]
    {
        cancel_android_notification(notification_id).await
    }
    
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        Ok(())
    }
}

/// Get notification settings
#[tauri::command]
pub async fn get_notification_settings() -> Result<NotificationSettings, String> {
    // TODO: Load from persistent storage
    Ok(NotificationSettings::default())
}

/// Update notification settings
#[tauri::command]
pub async fn update_notification_settings(
    settings: NotificationSettings,
) -> Result<(), String> {
    tracing::info!("Updating notification settings: {:?}", settings);
    // TODO: Save to persistent storage
    Ok(())
}

// iOS Implementation
#[cfg(target_os = "ios")]
async fn request_ios_notification_permission() -> Result<bool, String> {
    // TODO: Implement UNUserNotificationCenter authorization
    // 1. Get UNUserNotificationCenter.current()
    // 2. Request authorization with options: .alert, .sound, .badge
    // 3. Return authorization status
    
    tracing::info!("iOS notification permission requested");
    Ok(true)
}

#[cfg(target_os = "ios")]
async fn send_ios_notification(notification: Notification) -> Result<(), String> {
    // TODO: Implement UNNotificationRequest
    // 1. Create UNMutableNotificationContent
    // 2. Set title, body, sound
    // 3. Set category identifier
    // 4. Set userInfo with data
    // 5. Create UNNotificationRequest with trigger
    // 6. Add to notification center
    
    tracing::info!("iOS notification sent: {}", notification.title);
    Ok(())
}

#[cfg(target_os = "ios")]
async fn schedule_ios_notification(
    notification: Notification,
    delay_seconds: u64,
) -> Result<String, String> {
    // TODO: Implement scheduled notification
    // 1. Create UNTimeIntervalNotificationTrigger
    // 2. Set time interval
    // 3. Create notification request
    // 4. Add to notification center
    // 5. Return request identifier
    
    let id = format!("ios_scheduled_{}", notification.id);
    tracing::info!("iOS notification scheduled: {}", id);
    Ok(id)
}

#[cfg(target_os = "ios")]
async fn cancel_ios_notification(notification_id: String) -> Result<(), String> {
    // TODO: Implement notification cancellation
    // 1. Get UNUserNotificationCenter.current()
    // 2. Call removePendingNotificationRequests(withIdentifiers:)
    
    tracing::info!("iOS notification cancelled: {}", notification_id);
    Ok(())
}

// Android Implementation
#[cfg(target_os = "android")]
async fn request_android_notification_permission() -> Result<bool, String> {
    // TODO: Implement Android notification permission
    // For Android 13+ (API 33+):
    // 1. Check if POST_NOTIFICATIONS permission is granted
    // 2. Request permission if needed
    // For older versions, notifications are enabled by default
    
    tracing::info!("Android notification permission requested");
    Ok(true)
}

#[cfg(target_os = "android")]
async fn send_android_notification(notification: Notification) -> Result<(), String> {
    // TODO: Implement NotificationCompat.Builder
    // 1. Get NotificationManager
    // 2. Create notification channel if needed
    // 3. Build notification with NotificationCompat.Builder
    // 4. Set title, content, icon, priority
    // 5. Set category and extras
    // 6. Show notification
    
    tracing::info!("Android notification sent: {}", notification.title);
    Ok(())
}

#[cfg(target_os = "android")]
async fn schedule_android_notification(
    notification: Notification,
    delay_seconds: u64,
) -> Result<String, String> {
    // TODO: Implement scheduled notification with AlarmManager
    // 1. Create PendingIntent for notification
    // 2. Use AlarmManager.setExactAndAllowWhileIdle
    // 3. Set trigger time
    // 4. Return unique identifier
    
    let id = format!("android_scheduled_{}", notification.id);
    tracing::info!("Android notification scheduled: {}", id);
    Ok(id)
}

#[cfg(target_os = "android")]
async fn cancel_android_notification(notification_id: String) -> Result<(), String> {
    // TODO: Implement notification cancellation
    // 1. Get NotificationManager
    // 2. Call cancel() with notification ID
    // 3. Cancel AlarmManager if scheduled
    
    tracing::info!("Android notification cancelled: {}", notification_id);
    Ok(())
}

// Helper functions
impl Notification {
    pub fn sync_complete(grains_synced: usize) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Sync Complete".to_string(),
            body: format!("Synced {} grains with peers", grains_synced),
            category: NotificationCategory::SyncComplete,
            data: HashMap::from([
                ("grains_synced".to_string(), grains_synced.to_string()),
            ]),
        }
    }
    
    pub fn new_peer(peer_id: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: "New Peer Connected".to_string(),
            body: format!("Connected to peer {}", &peer_id[..8]),
            category: NotificationCategory::NewPeer,
            data: HashMap::from([
                ("peer_id".to_string(), peer_id.to_string()),
            ]),
        }
    }
    
    pub fn reward_earned(amount: f64) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Reward Earned!".to_string(),
            body: format!("You earned {:.2} NGT tokens", amount),
            category: NotificationCategory::RewardEarned,
            data: HashMap::from([
                ("amount".to_string(), amount.to_string()),
            ]),
        }
    }
    
    pub fn backup_reminder() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Backup Reminder".to_string(),
            body: "Don't forget to backup your recovery phrase".to_string(),
            category: NotificationCategory::BackupReminder,
            data: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_notification_creation() {
        let notif = Notification::sync_complete(42);
        assert_eq!(notif.title, "Sync Complete");
        assert!(notif.body.contains("42"));
    }
    
    #[test]
    fn test_notification_settings_default() {
        let settings = NotificationSettings::default();
        assert!(settings.enabled);
        assert!(settings.sync_notifications);
    }
}
