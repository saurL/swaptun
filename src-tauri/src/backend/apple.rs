use crate::backend::backend::BackendClient;
use crate::error::AppResult;
use swaptun_backend::{AddTokenRequest, GetDeveloperToken};
use tauri::http::StatusCode;

pub struct AppleService {
    backend_client: BackendClient,
}

impl AppleService {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self {
            backend_client: BackendClient::new(app_handle),
        }
    }

    pub async fn get_developer_token(&self) -> AppResult<GetDeveloperToken> {
        self.backend_client
            .get::<GetDeveloperToken>("apple/developer-token")
            .await
    }

    pub async fn send_authorization_token(&self, request: AddTokenRequest) -> AppResult<()> {
        let url = "apple/token";
        self.backend_client
            .post(url, serde_json::to_string(&request).unwrap())
            .await?;
        Ok(())
    }

    pub async fn synchronize_playlists(&self) -> AppResult<StatusCode> {
        let url = "apple/synchronize";
        self.backend_client
            .post(url, serde_json::to_string(&()).unwrap())
            .await
    }

    pub async fn disconnect(&self) -> AppResult<StatusCode> {
        self.backend_client
            .delete("apple/disconnect")
            .await
    }
}
