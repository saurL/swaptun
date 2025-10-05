use std::sync::Arc;

use log::{error, info};
use swaptun_backend::{RegisterFcmTokenRequest, SendTestNotificationRequest};
use tauri::{command, AppHandle, Emitter, State};
use tauri_plugin_push_notifications::PushNotificationsExt;

use crate::{app::App, models::Notification};

/// Handle notification data by emitting appropriate events
pub fn handle_notification_data(app: &AppHandle, data: &Notification) {
    let route = data.get_route();
    info!(
        "Handling notification - Type: {}, Route: {}, Playlist: {:?}",
        data.notification_type, route, data.playlist_name
    );

    // Emit the full notification data for the frontend to handle
    if let Err(e) = app.emit("notification_data", data.clone()) {
        error!("Failed to emit notification_data event: {}", e);
    }

    // Emit routing event if route is present
    if !route.is_empty() {
        if let Err(e) = app.emit("routing", route.clone()) {
            error!("Failed to emit routing event: {}", e);
        }
    }

    // Handle specific notification types
    match data.notification_type.as_str() {
        "playlist_shared" => {
            info!(
                "Playlist shared: {} (ID: {:?})",
                data.playlist_name.as_ref().unwrap_or(&"Unknown".to_string()),
                data.playlist_id
            );
            // Emit specific event for playlist shared
            if let Err(e) = app.emit("playlist_shared", data.clone()) {
                error!("Failed to emit playlist_shared event: {}", e);
            }
        }
        _ => {
            info!("Unknown notification type: {}", data.notification_type);
        }
    }
}

/// Handle notification click (when app is closed and user clicks notification)
pub fn handle_notification(app: &AppHandle, data: Notification) {
    info!("Notification clicked (app was closed)");
    handle_notification_data(app, &data);

    // Also emit clicked event for frontend tracking
    if let Err(e) = app.emit("notification_clicked", data) {
        error!("Failed to emit notification_clicked event: {}", e);
    }
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
