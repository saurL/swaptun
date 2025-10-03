use crate::backend::backend::BackendClient;
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

    pub async fn get_developer_token(
        &self,
    ) -> Result<GetDeveloperToken, Box<dyn std::error::Error + Send + Sync>> {
        self.backend_client
            .get::<GetDeveloperToken>("apple/developer-token")
            .await
    }

    pub async fn send_authorization_token(
        &self,
        request: AddTokenRequest,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let url = "apple/token";
        self.backend_client
            .post(url, serde_json::to_string(&request).unwrap())
            .await?;
        Ok(())
    }

    pub async fn synchronize_playlists(
        &self,
    ) -> Result<StatusCode, Box<dyn std::error::Error + Send + Sync>> {
        let url = "apple/synchronize";
        self.backend_client
            .post(url, serde_json::to_string(&()).unwrap())
            .await
    }
}
