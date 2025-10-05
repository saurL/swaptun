use crate::backend::backend::BackendClient;
use crate::error::AppResult;
use serde::{Deserialize, Serialize};
use swaptun_backend::{GetPlaylistResponse, GetPlaylistsParams, SendPlaylistRequest};
use tauri::{http::StatusCode, AppHandle};

#[derive(Debug, Serialize)]
pub struct SharePlaylistRequest {
    pub user_id: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SharedPlaylist {
    pub id: i32,
    pub playlist_id: i32,
    pub playlist_name: String,
    pub shared_by_user_id: i32,
    pub shared_by_username: String,
    pub shared_at: String,
    pub viewed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetSharedPlaylistsResponse {
    pub playlists: Vec<SharedPlaylist>,
}

pub struct PlaylistService {
    backend_client: BackendClient,
    base_url: String,
}

impl PlaylistService {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            backend_client: BackendClient::new(app_handle),
            base_url: "playlists".into(),
        }
    }

    pub async fn get_playlists(&self, params: GetPlaylistsParams) -> AppResult<GetPlaylistResponse> {
        self.backend_client
            .get_with_body::<GetPlaylistResponse, _>(
                &self.base_url,
                serde_json::to_string(&params).unwrap(),
            )
            .await
    }

    pub async fn send_playlist(&self, playlist_id: i32, req: SendPlaylistRequest) -> AppResult<StatusCode> {
        let url = format!("{}/{}/send", self.base_url, playlist_id);
        self.backend_client
            .post(&url, serde_json::to_string(&req).unwrap())
            .await
    }

    pub async fn share_playlist(&self, playlist_id: i32, req: SharePlaylistRequest) -> AppResult<StatusCode> {
        let url = format!("{}/{}/share", self.base_url, playlist_id);
        self.backend_client
            .post(&url, serde_json::to_string(&req).unwrap())
            .await
    }

    pub async fn get_shared_playlists(&self) -> AppResult<GetSharedPlaylistsResponse> {
        let url = format!("{}/shared", self.base_url);
        self.backend_client.get(&url).await
    }

    pub async fn mark_shared_playlist_viewed(&self, shared_playlist_id: i32) -> AppResult<StatusCode> {
        let url = format!("{}/shared/{}/viewed", self.base_url, shared_playlist_id);
        self.backend_client.post(&url, "{}").await
    }
}
