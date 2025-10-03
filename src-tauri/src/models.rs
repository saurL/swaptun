use serde::{Deserialize, Serialize};
use tauri_plugin_push_notifications::NotificationDataTrait;

#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct Notification {
    pub route: String,
}
impl NotificationDataTrait for Notification {}



    