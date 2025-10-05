use std::sync::Arc;

use log::{error, info};
use swaptun_backend::{RegisterFcmTokenRequest, SendTestNotificationRequest};
use tauri::{command, AppHandle, Emitter, State};
use tauri_plugin_push_notifications::PushNotificationsExt;

use crate::{app::App, models::Notification};

/// Handle notification data by emitting appropriate events
pub fn handle_notification_data(app: &AppHandle, data: Notification) {
    info!("Handling notification data: {:?}", data);
    if let Some(shared_playlist_notification) = data.shared_notification {
        if let Err(e) = app.emit("playlist_shared", shared_playlist_notification.clone()) {
            error!("Failed to emit playlist_shared event: {}", e);
        }
    }
}

/// Handle notification click (when app is closed and user clicks notification)
pub fn handle_notification(app: &AppHandle, data: Notification) {
    info!("Notification clicked with data: {:?}", data);
    if let Some(ref route) = data.route {
        info!("Navigating to route: {}", route);
        if let Err(e) = app.emit("routing", route.clone()) {
            error!("Failed to emit routing event: {}", e);
        }
    }
    handle_notification_data(app, data);
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
