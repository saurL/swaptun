use crate::backend::backend::BackendClient;
use serde::Deserialize;
use tauri::{http::StatusCode, AppHandle};

use swaptun_backend::{AddTokenRequest, GetAuthorizationUrlRequest, SpotifyUrlResponse};
use tauri_plugin_http::reqwest::Body;

#[derive(Debug, Deserialize)]
pub struct SpotifyAuthResponse {
    pub _access_token: String,
    pub _token_type: String,
    pub _expires_in: u64,
}

pub struct SpotifyClient {
    backend_client: BackendClient,
}

impl SpotifyClient {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            backend_client: BackendClient::new(app_handle),
        }
    }

    pub async fn get_auth_url(
        &self,
        req: GetAuthorizationUrlRequest,
    ) -> Result<SpotifyUrlResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.backend_client
            .get_with_body::<SpotifyUrlResponse, _>(
                "spotify/authorization-url",
                serde_json::to_string(&req).unwrap(),
            )
            .await
    }

    pub async fn add_token(
        &self,
        req: AddTokenRequest,
    ) -> Result<StatusCode, Box<dyn std::error::Error + Send + Sync>> {
        self.backend_client
            .post("spotify/token", serde_json::to_string(&req).unwrap())
            .await
    }
    pub async fn import_playlist_backend_request(
        &self,
    ) -> Result<StatusCode, Box<dyn std::error::Error + Send + Sync>> {
        self.backend_client
            .post("spotify/playlist", Body::from(""))
            .await
    }

    pub async fn set_auth_header(&self, token: String) {
        self.backend_client.set_auth_header(token).await;
    }
}
