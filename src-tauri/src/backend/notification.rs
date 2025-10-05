use log::info;
use serde_json;
use tauri::AppHandle;
use tauri_plugin_http::reqwest::StatusCode;

use crate::backend::backend::BackendClient;
use crate::error::AppResult;
use swaptun_backend::{RegisterFcmTokenRequest, SendTestNotificationRequest};

pub struct NotificationService {
    backend_client: BackendClient,
}

impl NotificationService {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            backend_client: BackendClient::new(app_handle),
        }
    }

    pub async fn set_fcm_token(&self, register_fcm_token_request: RegisterFcmTokenRequest) -> AppResult<StatusCode> {
        info!("Setting FCM token");
        let body = serde_json::to_string(&register_fcm_token_request)?;
        self.backend_client
            .post("notifications/fcm-token", body)
            .await
    }

    pub async fn send_test_notification(&self, notification_request: SendTestNotificationRequest) -> AppResult<StatusCode> {
        info!("Sending test notification");
        let body = serde_json::to_string(&notification_request)?;
        self.backend_client
            .post("notifications/test-notification", body)
            .await
    }
}
