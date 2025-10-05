use serde::{Deserialize, Serialize};
use tauri_plugin_push_notifications::NotificationDataTrait;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Notification {
    #[serde(rename = "type")]
    pub notification_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_name: Option<String>,
}

impl Notification {
    pub fn playlist_shared(playlist_id: String, playlist_name: String) -> Self {
        Self {
            notification_type: "playlist_shared".to_string(),
            route: Some("/shared-playlists".to_string()),
            playlist_id: Some(playlist_id),
            playlist_name: Some(playlist_name),
        }
    }

    pub fn get_route(&self) -> String {
        self.route.clone().unwrap_or_default()
    }
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
        ErrorNotification::ServerError { message: message.into() }
    }

    pub fn network_error(message: impl Into<String>) -> Self {
        ErrorNotification::NetworkError { message: message.into() }
    }
}

    