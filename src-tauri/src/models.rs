use serde::{Deserialize, Deserializer, Serialize};
use tauri_plugin_push_notifications::NotificationDataTrait;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SharedNotificationData {
    pub playlist_id: i32,
    pub playlist_name: String,
    pub shared_by_id: i32,
    pub shared_by_username: String,
}

// Custom deserializer for shared_notification field
// Deserializes from a JSON string into SharedNotificationData
fn deserialize_shared_notification<'de, D>(
    deserializer: D,
) -> Result<Option<SharedNotificationData>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(json_str) => serde_json::from_str::<SharedNotificationData>(&json_str)
            .map(Some)
            .map_err(|e| Error::custom(format!("Failed to parse shared_notification: {}", e))),
        None => Ok(None),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Notification {
    #[serde(rename = "type")]
    pub notification_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_shared_notification"
    )]
    pub shared_notification: Option<SharedNotificationData>,
}

impl Notification {
    pub fn get_route(&self) -> String {
        self.route.clone().unwrap_or_default()
    }

    pub fn get_shared_data(&self) -> Option<&SharedNotificationData> {
        self.shared_notification.as_ref()
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
