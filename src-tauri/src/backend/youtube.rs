use crate::backend::backend::BackendClient;
use tauri::{http::StatusCode, AppHandle};

use swaptun_backend::{AddTokenRequest, YoutubeUrlResponse};


pub struct YoutubeClient {
    backend_client: BackendClient,
}

impl YoutubeClient {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            backend_client: BackendClient::new(app_handle),
        }
    }

    pub async fn get_auth_url(
        &self,
    ) -> Result<YoutubeUrlResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.backend_client
            .get::<YoutubeUrlResponse>(
                "youtube/authorization-url",
            )
            .await
    }

    pub async fn add_token(
        &self,
        req: AddTokenRequest,
    ) -> Result<StatusCode, Box<dyn std::error::Error + Send + Sync>> {
        self.backend_client
            .post("youtube/token", serde_json::to_string(&req).unwrap())
            .await
    }

}
