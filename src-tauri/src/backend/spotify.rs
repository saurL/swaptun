use crate::backend::backend::BackendClient;
use crate::error::AppResult;
use serde::Deserialize;
use tauri::{http::StatusCode, AppHandle};

use swaptun_backend::{AddTokenRequest, SpotifyUrlResponse};
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

    pub async fn get_auth_url(&self) -> AppResult<SpotifyUrlResponse> {
        self.backend_client
            .get::<SpotifyUrlResponse>("spotify/authorization-url")
            .await
    }

    pub async fn add_token(&self, req: AddTokenRequest) -> AppResult<StatusCode> {
        self.backend_client
            .post("spotify/token", serde_json::to_string(&req).unwrap())
            .await
    }

    pub async fn import_playlist_backend_request(&self) -> AppResult<StatusCode> {
        self.backend_client
            .post("spotify/playlist", Body::from(""))
            .await
    }

    pub async fn disconnect(&self) -> AppResult<StatusCode> {
        self.backend_client
            .delete("spotify/disconnect")
            .await
    }
}
