use std::sync::Arc;

use log::{error, info};
use swaptun_backend::{RegisterFcmTokenRequest, SendTestNotificationRequest};
use tauri::{command, AppHandle, Emitter, State};
use tauri_plugin_push_notifications::PushNotificationsExt;

use crate::{app::App, models::Notification};

pub fn handle_notification(app: &AppHandle, data: Notification) {
    info!("Notification clicked: {}", data.route);
    app.emit("routing", data.route)
        .expect("Failed to emit routing event");
}

#[command]
pub async fn set_fcm_token(app: State<'_, Arc<App>>, token: String) -> Result<bool, String> {
    let register_fcm_token_request = RegisterFcmTokenRequest {
        token,
        device_id: Some("default_device_id".to_string()), // Replace with actual device id if available
        platform: Some("default_platform".to_string()), // Replace with actual platform if available
    };
    match app.set_fcm_token(register_fcm_token_request).await {
        Ok(status) => {
            if status.is_success() {
                Ok(true)
            } else {
                Err(format!("Failed to set FCM token, status: {}", status))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub async fn send_test_notification(
    app: State<'_, Arc<App>>,
    title: String,
    body: String,
    user_id: i32,
) -> Result<bool, String> {
    let mut data = std::collections::HashMap::new();
    data.insert("route".to_string(), "/register".to_string());
    let notification_data = SendTestNotificationRequest {
        user_id,
        title,
        body,
        data: Some(data),
    };
    match app.send_test_notification(notification_data).await {
        Ok(status) => {
            if status.is_success() {
                Ok(true)
            } else {
                Err(format!(
                    "Failed to send test notification, status: {}",
                    status
                ))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub async fn check_opening_notification(app: State<'_, Arc<App>>) -> Result<(), String> {
    let app_handle = app.app_handle().clone();
    match app_handle
        .push_notifications()
        .get_opening_notification_data::<Notification>()
    {
        Ok(Some(data)) => {
            handle_notification(&app_handle, data);
            info!("Opening notification data");
        }
        Ok(None) => {
            info!("No opening notification data found");
        }
        Err(e) => {
            error!("Error getting opening notification data: {}", e);
        }
    };
    Ok(())
}
