use serde::{Deserialize, Serialize};
use tauri_plugin_push_notifications::NotificationDataTrait;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SharedNotificationData {
    pub playlist_id: String,
    pub playlist_name: String,
    pub shared_by_id: String,
    pub shared_by_username: String,
    pub shared_by_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Notification {
    #[serde(rename = "type")]
    pub notification_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_notification: Option<SharedNotificationData>,
    pub route: Option<String>,
}

impl NotificationDataTrait for Notification {}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum ErrorNotification {
    #[serde(rename = "server_error")]
    ServerError { message: String },
    #[serde(rename = "network_error")]
    NetworkError { message: String },
}

impl ErrorNotification {
    pub fn server_error(message: impl Into<String>) -> Self {
        ErrorNotification::ServerError {
            message: message.into(),
        }
    }

    pub fn network_error(message: impl Into<String>) -> Self {
        ErrorNotification::NetworkError {
            message: message.into(),
        }
    }
}
